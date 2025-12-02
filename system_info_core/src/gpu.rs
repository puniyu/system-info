#[cfg(any(target_os = "windows", target_os = "linux"))]
use crate::round;

#[derive(Debug, Clone)]
pub struct GpuInfo {
	/// GPU型号
	pub model: String,
	/// GPU总内存(单位: MB)
	pub memory_total: Option<f32>,
	/// GPU已用内存(单位: MB)
	pub memory_used: Option<f32>,
	/// GPU可用内存(单位: MB)
	pub memory_free: Option<f32>,
	/// GPU使用率
	pub usage: Option<u8>,
}

impl GpuInfo {
	pub fn new() -> Option<Self> {
		#[cfg(target_os = "windows")]
		{
			Self::from_windows()
		}

		#[cfg(target_os = "linux")]
		{
			Self::from_linux()
		}

		#[cfg(target_os = "macos")]
		{
			Self::from_iokit()
		}

		#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
		{
			None
		}
	}

	#[cfg(target_os = "windows")]
	fn from_windows() -> Option<Self> {
		Self::from_nvml()
			.or_else(Self::from_wmi)
			.or_else(Self::from_dxgi)
	}

	#[cfg(target_os = "windows")]
	fn from_nvml() -> Option<Self> {
		use nvml_wrapper::Nvml;

		let nvml = Nvml::init().ok()?;
		let device = nvml.device_by_index(0).ok()?;

		let model = device.name().ok()?;
		let memory_info = device.memory_info().ok()?;
		let utilization = device.utilization_rates().ok();

		Some(Self {
			model,
			memory_total: Some(round(memory_info.total as f64 / 1024.0 / 1024.0) as f32),
			memory_used: Some(round(memory_info.used as f64 / 1024.0 / 1024.0) as f32),
			memory_free: Some(round(memory_info.free as f64 / 1024.0 / 1024.0) as f32),
			usage: utilization.map(|u| u.gpu as u8),
		})
	}

	#[cfg(target_os = "windows")]
	fn from_wmi() -> Option<Self> {
		use serde::Deserialize;
		use wmi::WMIConnection;

		#[derive(Deserialize)]
		#[serde(rename_all = "PascalCase")]
		struct Win32VideoController {
			name: Option<String>,
			adapter_ram: Option<u64>,
		}

		let wmi = WMIConnection::new().ok()?;

		let results: Vec<Win32VideoController> = wmi.query().ok()?;
		let gpu = results.into_iter().next()?;

		let model = gpu.name.unwrap_or_else(|| "Unknown".to_string());
		let memory_total = gpu
			.adapter_ram
			.map(|r| round(r as f64 / 1024.0 / 1024.0) as f32);

		Some(Self {
			model,
			memory_total,
			memory_used: None,
			memory_free: None,
			usage: None,
		})
	}

	#[cfg(target_os = "windows")]
	fn from_dxgi() -> Option<Self> {
		use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory1, IDXGIAdapter1, IDXGIFactory1};

		unsafe {
			let factory: IDXGIFactory1 = CreateDXGIFactory1().ok()?;
			let adapter: IDXGIAdapter1 = factory.EnumAdapters1(0).ok()?;
			let desc = adapter.GetDesc1().ok()?;

			let model = String::from_utf16_lossy(
				&desc.Description[..desc
					.Description
					.iter()
					.position(|&c| c == 0)
					.unwrap_or(desc.Description.len())],
			);

			let memory_total = Some(round(desc.DedicatedVideoMemory as f64 / 1024.0 / 1024.0) as f32);

			Some(Self {
				model,
				memory_total,
				memory_used: None,
				memory_free: None,
				usage: None,
			})
		}
	}

	#[cfg(target_os = "linux")]
	fn from_nvml() -> Option<Self> {
		use nvml_wrapper::Nvml;

		let nvml = Nvml::init().ok()?;
		let device = nvml.device_by_index(0).ok()?;

		let model = device.name().ok()?;
		let memory_info = device.memory_info().ok()?;
		let utilization = device.utilization_rates().ok();

		Some(Self {
			model,
			memory_total: Some(round(memory_info.total as f64 / 1024.0 / 1024.0) as f32),
			memory_used: Some(round(memory_info.used as f64 / 1024.0 / 1024.0) as f32),
			memory_free: Some(round(memory_info.free as f64 / 1024.0 / 1024.0) as f32),
			usage: utilization.map(|u| u.gpu as u8),
		})
	}

	#[cfg(target_os = "linux")]
	fn from_linux() -> Option<Self> {
		Self::from_nvml().or_else(Self::from_lspci)
	}

	#[cfg(target_os = "linux")]
	fn from_lspci() -> Option<Self> {
		use std::process::Command;

		let output = Command::new("lspci")
			.args(["-v"])
			.output()
			.ok()?;

		if !output.status.success() {
			return None;
		}

		let stdout = String::from_utf8_lossy(&output.stdout);

		for section in stdout.split("\n\n") {
			let lower = section.to_lowercase();
			if lower.contains("vga") || lower.contains("3d controller") {
				let first_line = section.lines().next()?;
				let model = first_line.split(':').nth(2).unwrap_or("Unknown GPU").trim();

				let memory_total = section
					.lines()
					.find(|l| l.to_lowercase().contains("memory") && l.contains("M"))
					.and_then(|l| {
						l.split_whitespace()
							.find(|s| s.ends_with('M') || s.ends_with("MB"))
							.and_then(|s| s.trim_end_matches(|c| !char::is_ascii_digit(&c)).parse::<f32>().ok())
					});

				return Some(Self {
					model: model.to_string(),
					memory_total,
					memory_used: None,
					memory_free: None,
					usage: None,
				});
			}
		}
		None
	}

	#[cfg(target_os = "macos")]
	fn from_iokit() -> Option<Self> {
		use core_foundation::base::{CFType, TCFType};
		use core_foundation::dictionary::{CFDictionary, CFMutableDictionaryRef};
		use core_foundation::number::CFNumber;
		use core_foundation::string::CFString;
		use io_kit_sys::*;
		use io_kit_sys::types::*;
		use mach2::kern_return::KERN_SUCCESS;

		unsafe {
			let matching = IOServiceMatching(b"IOPCIDevice\0".as_ptr() as *const i8);
			if matching.is_null() {
				return None;
			}

			let mut iterator: io_iterator_t = 0;
			let result = IOServiceGetMatchingServices(kIOMasterPortDefault, matching, &mut iterator);

			if result != KERN_SUCCESS as i32 {
				return None;
			}

			let mut gpu_model = String::new();
			let mut vram: f32 = 0.0;

			loop {
				let service = IOIteratorNext(iterator);
				if service == 0 {
					break;
				}

				let mut properties: CFMutableDictionaryRef = std::ptr::null_mut();
				let result =
					IORegistryEntryCreateCFProperties(service, &mut properties, std::ptr::null(), 0);

				if result == KERN_SUCCESS as i32 && !properties.is_null() {
					let dict =
						CFDictionary::<CFString, CFType>::wrap_under_get_rule(properties as _);

					if let Some(class_code) = dict.find(CFString::new("class-code")) {
						if let Some(num) = class_code.downcast::<CFNumber>() {
							let code: i32 = num.to_i32().unwrap_or(0);
							if (code & 0xFF0000) == 0x030000 {
								if let Some(model) = dict.find(CFString::new("model")) {
									if let Some(data) =
										model.downcast::<core_foundation::data::CFData>()
									{
										let bytes = data.bytes();
										if let Ok(s) = std::str::from_utf8(bytes) {
											gpu_model = s.trim_end_matches('\0').to_string();
										}
									}
								}

								if let Some(vram_size) = dict.find(CFString::new("VRAM,totalMB")) {
									if let Some(num) = vram_size.downcast::<CFNumber>() {
										vram = num.to_i64().unwrap_or(0) as f32;
									}
								}

								if !gpu_model.is_empty() {
									IOObjectRelease(service);
									break;
								}
							}
						}
					}
				}

				IOObjectRelease(service);
			}

			IOObjectRelease(iterator);

			if gpu_model.is_empty() {
				return None;
			}

			let memory_total = if vram > 0.0 { Some(vram) } else { None };
			Some(Self {
				model: gpu_model,
				memory_total,
				memory_used: None,
				memory_free: None,
				usage: None,
			})
		}
	}
}