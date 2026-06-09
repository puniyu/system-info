#[derive(Debug, Clone)]
pub struct CpuInfo {
	/// CPU名称
	pub model_name: String,
	/// CPU核心数
	pub physical_cores: u32,
	/// CPU 线程数
	pub logical_cores: u32,
	/// CPU基本频率(单位: GHz)
	pub frequency: f32,
	/// CPU使用率
	pub usage: Option<f32>,
	/// CPU温度(单位: °C)，无法读取时为 None
	pub temperature: Option<f32>,
}

impl Default for CpuInfo {
	fn default() -> Self {
		use std::thread::sleep;
		use sysinfo::System;
		let mut system = System::new();
		system.refresh_cpu_all();

		sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
		system.refresh_cpu_usage();
		let cpu = &system.cpus()[0];

		Self {
			model_name: cpu.brand().to_string(),
			physical_cores: num_cpus::get_physical() as u32,
			logical_cores: num_cpus::get() as u32,
			frequency: cpu.frequency() as f32 / 1000.0,
			usage: Some(system.global_cpu_usage().round()),
			temperature: read_cpu_temperature(),
		}
	}
}

impl CpuInfo {
	pub fn new() -> Self {
		Self::default()
	}
}

fn read_cpu_temperature() -> Option<f32> {
	use sysinfo::Components;

	let components = Components::new_with_refreshed_list();

	const PREFERRED_IDS: &[&str] = &["TC0P", "TC0D", "TC0E", "TC0F"];
	for id in PREFERRED_IDS {
		for component in &components {
			if component.id() == Some(*id) {
				if let Some(temp) = component.temperature() {
					return Some(crate::round(temp as f64) as f32);
				}
			}
		}
	}

	let keywords = ["cpu", "core", "package", "processor", "soc", "tctl", "tdie"];
	let mut best: Option<f32> = None;
	for component in &components {
		let label = component.label().to_ascii_lowercase();
		let id = component.id().unwrap_or("").to_ascii_lowercase();
		if keywords.iter().any(|keyword| label.contains(keyword) || id.contains(keyword)) {
			if let Some(temp) = component.temperature() {
				best = Some(best.map_or(temp, |current| current.max(temp)));
			}
		}
	}
	if let Some(temp) = best {
		return Some(crate::round(temp as f64) as f32);
	}

	for component in &components {
		if let Some(id) = component.id() {
			if id.starts_with("TXC") {
				if let Some(temp) = component.temperature() {
					return Some(crate::round(temp as f64) as f32);
				}
			}
		}
	}

	for component in &components {
		if component.id() == Some("thermal_zone0") {
			if let Some(temp) = component.temperature() {
				return Some(crate::round(temp as f64) as f32);
			}
		}
	}

	None
}
