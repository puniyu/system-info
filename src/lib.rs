#[cfg(feature = "gpu")]
use gfxinfo::active_gpu;
use rust_decimal::{
	Decimal,
	prelude::{FromPrimitive, ToPrimitive},
};
#[cfg(feature = "process")]
pub use sysinfo::Pid;
#[cfg(feature = "network")]
use {
	std::net::IpAddr,
	sysinfo::{MacAddr, Networks},
};

#[derive(Debug, Clone)]
#[cfg(feature = "host")]
pub struct HostInfo {
	/// 主机名
	pub host_name: String,
	/// 操作系统名
	pub os_name: String,
	/// 操作系统版本
	pub os_version: String,
	/// 操作系统类型
	pub os_type: String,
	/// 系统架构
	pub arch: String,
	/// 系统启动时间
	pub boot_time: u64,
	/// 系统运行时间， 单位：秒
	pub uptime: u64,
}

#[derive(Debug, Clone)]
#[cfg(feature = "network")]
pub struct NetworkInfo {
	/// 网卡名称
	pub name: String,
	/// 网卡ip信息
	pub ip_info: Vec<IpInfo>,
	/// 网卡接收字节数(单位: KB/S)
	pub upload: f64,
	/// 网卡发送字节数(单位: KB/S)
	pub download: f64,
	/// 网卡mac地址
	pub mac_addr: MacAddr,
}

#[derive(Debug, Clone)]
#[cfg(feature = "network")]
pub struct IpInfo {
	/// ip地址
	pub ip_address: IpAddr,
	/// 子网掩码
	pub netmask: u8,
}

#[derive(Debug, Clone)]
pub struct ProcessInfo {
	/// 进程ID
	pub pid: Pid,
	/// 进程名称
	pub name: String,
	/// 进程启动时间
	pub start_time: u64,
	/// 进程运行时间，单位：秒
	pub run_time: u64,
	/// 进程CPU使用率
	pub cpu_usage: Option<u8>,
	/// 进程内存使用率
	pub memory_usage: Option<u8>,
	/// 进程已用内存(单位: MB)
	pub used_memory: f32,
}
#[derive(Debug, Clone)]
#[cfg(feature = "cpu")]
pub struct CpuInfo {
	/// CPU型号
	pub cpu_model: String,
	/// CPU核心数
	pub cpu_cores: usize,
	/// CPU频率(单位: GHz)
	pub cpu_frequency: Option<f32>,
	/// CPU使用率
	pub cpu_usage: Option<u8>,
}

#[derive(Debug, Clone)]
#[cfg(feature = "gpu")]
pub struct GpuInfo {
	/// GPU型号
	pub gpu_model: String,
	///  GPU已用内存(单位: MB)
	pub gpu_memory_used: f32,
	/// GPU总内存(单位: MB)
	pub gpu_memory_total: f32,
	///  GPU可用内存(单位: MB)
	pub gpu_memory_free: f32,
	/// GPU使用率
	pub gpu_usage: u8,
}

#[derive(Debug, Clone)]
#[cfg(feature = "memory")]
pub struct MemoryInfo {
	/// 总内存(单位: MB)
	pub total_memory: f32,
	/// 已用内存(单位: MB)
	pub used_memory: f32,
	/// 可用内存(单位: MB)
	pub free_memory: f32,
	/// 交换内存(单位: MB)
	pub swap_memory_total: f32,
	/// 交换内存已用(单位: MB)
	pub swap_memory_used: f32,
	/// 交换内存可用(单位: MB)
	pub swap_memory_free: f32,
	/// 交换内存使用率
	pub swap_memory_usage: Option<u8>,
	/// 内存使用率
	pub memory_usage: Option<u8>,
}

#[derive(Debug, Clone)]
#[cfg(feature = "disk")]
pub struct DiskDetail {
	/// 磁盘名称
	pub name: String,
	/// 总磁盘空间(单位: GB)
	pub total_space: f32,
	/// 已用磁盘空间(单位: GB)
	pub used_space: f32,
	/// 可用磁盘空间(单位: GB)
	pub free_space: f32,
	/// 磁盘使用率
	pub usage: f32,
}

#[derive(Debug, Clone)]
#[cfg(feature = "disk")]
pub struct DiskInfo {
	/// 总磁盘空间(单位: GB)
	pub total_disk_space: f32,
	/// 总已用磁盘空间(单位: GB)
	pub total_used_space: f32,
	/// 总可用磁盘空间(单位: GB)
	pub total_free_space: f32,
	/// 总体磁盘使用率
	pub total_usage: f32,
	/// 各个磁盘详细信息
	pub disks: Vec<DiskDetail>,
}

#[derive(Debug, Clone)]
pub struct SystemInfo;

impl SystemInfo {
	/// 获取主机信息
	///
	/// 此函数可以获取主机信息，包括主机名、操作系统名、操作系统版本、操作系统类型、系统启动时间等
	/// # 返回值
	///
	/// * [HostInfo] - 主机信息
	///
	#[cfg(feature = "host")]
	pub fn host() -> HostInfo {
		use std::env;
		use sysinfo::System;
		let hostname = System::host_name().unwrap();
		let os_name = System::name().unwrap();
		let arch = System::cpu_arch();
		let os_version = System::os_version().unwrap();
		let os_type = env::consts::OS.to_string();
		let boot_time = System::boot_time();
		let uptime = System::uptime();
		HostInfo { host_name: hostname, os_name, arch, os_version, os_type, boot_time, uptime }
	}

	/// 获取CPU信息
	///
	/// 此函数可以获取CPU信息，包括型号、核心数、频率、使用率等
	/// # 返回值
	///
	/// * [CpuInfo] - CPU信息
	///
	#[cfg(feature = "cpu")]
	pub fn cpu() -> CpuInfo {
		use std::thread::sleep;
		use sysinfo::System;
		let mut system = System::new();
		system.refresh_cpu_all();

		sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
		system.refresh_cpu_usage();

		let cpus = system.cpus();

		let cpu_usage = if !cpus.is_empty() {
			let usage = cpus.iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / cpus.len() as f32;
			Some(usage.round() as u8)
		} else {
			None
		};

		let cpu_cores = cpus.len();

		let cpu_model =
			if !cpus.is_empty() { cpus[0].brand().to_string() } else { "Unknown".to_string() };

		let cpu_frequency = if !cpus.is_empty() { Some(cpus[0].frequency() as f32) } else { None };

		CpuInfo { cpu_usage, cpu_frequency, cpu_cores, cpu_model }
	}

	/// 获取内存信息
	///
	/// 此函数可以获取内存信息，包括总内存、已用内存、可用内存、内存使用率等
	/// # 返回值
	///
	/// * [MemoryInfo] - 内存信息
	///
	#[cfg(feature = "memory")]
	#[inline]
	pub fn memory() -> MemoryInfo {
		use sysinfo::System;
		let mut system = System::new();
		system.refresh_memory();

		let total_memory = system.total_memory() / 1024 / 1024;
		let used_memory = system.used_memory() / 1024 / 1024;
		let free_memory = total_memory - used_memory;

		let swap_memory_total = system.total_swap() / 1024 / 1024;
		let swap_memory_used = system.used_swap() / 1024 / 1024;
		let swap_memory_free = swap_memory_total - swap_memory_used;

		let total_memory_f32 = format_float(total_memory as f64, 2);
		let used_memory_f32 = format_float(used_memory as f64, 2);
		let free_memory_f32 = format_float(free_memory as f64, 2);

		let swap_memory_usage_f32 = format_float(swap_memory_used as f64, 2);
		let swap_memory_free_f32 = format_float(swap_memory_free as f64, 2);
		let swap_memory_total_f32 = format_float(swap_memory_total as f64, 2);

		let memory_usage = if total_memory > 0 {
			Some(((used_memory as f32 / total_memory as f32) * 100.0) as u8)
		} else {
			None
		};
		let swap_memory_usage = if swap_memory_total > 0 {
			Some(((swap_memory_used as f32 / swap_memory_total as f32) * 100.0) as u8)
		} else {
			None
		};

		MemoryInfo {
			total_memory: total_memory_f32 as f32,
			used_memory: used_memory_f32 as f32,
			free_memory: free_memory_f32 as f32,
			memory_usage,
			swap_memory_total: swap_memory_total_f32 as f32,
			swap_memory_used: swap_memory_usage_f32 as f32,
			swap_memory_free: swap_memory_free_f32 as f32,
			swap_memory_usage,
		}
	}

	/// 获取磁盘信息
	///
	/// 此函数可以获取磁盘信息，包括总磁盘空间、已用磁盘空间、可用磁盘空间、磁盘使用率等
	/// # 返回值
	///
	/// * [DiskInfo] - 磁盘信息
	///
	#[cfg(feature = "disk")]
	#[inline]
	pub fn disk() -> DiskInfo {
		use sysinfo::Disks;
		let disks = Disks::new_with_refreshed_list();

		let mut total_disk_space = 0f32;
		let mut total_used_space = 0f32;
		let mut total_free_space = 0f32;
		let mut disk_details = Vec::new();

		for disk in disks.list() {
			let total_space = disk.total_space() as f32 / (1024.0 * 1024.0 * 1024.0);
			let free_space = disk.available_space() as f32 / (1024.0 * 1024.0 * 1024.0);
			let used_space = total_space - free_space;

			let usage =
				if disk.total_space() > 0 { (used_space / total_space) * 100.0 } else { 0.0 };

			let disk_detail = DiskDetail {
				name: disk.name().to_string_lossy().to_string(),
				total_space: format_float(total_space as f64, 2) as f32,
				used_space: format_float(used_space as f64, 2) as f32,
				free_space: format_float(free_space as f64, 2) as f32,
				usage: format_float(usage as f64, 2) as f32,
			};

			total_disk_space += total_space;
			total_used_space += used_space;
			total_free_space += free_space;
			disk_details.push(disk_detail);
		}

		let total_usage = if total_disk_space > 0.0 {
			(total_used_space / total_disk_space) * 100.0
		} else {
			0.0
		};

		DiskInfo {
			total_disk_space: format_float(total_disk_space as f64, 2) as f32,
			total_used_space: format_float(total_used_space as f64, 2) as f32,
			total_free_space: format_float(total_free_space as f64, 2) as f32,
			total_usage: format_float(total_usage as f64, 2) as f32,
			disks: disk_details,
		}
	}

	/// 获取网卡信息
	///
	/// 此函数可以获取网络信息，包括网络名称、MAC地址、上传速度、下载速度、IP地址等
	/// # 返回值
	///
	/// * [NetworkInfo] - 网络信息
	///
	#[cfg(feature = "network")]
	pub fn network() -> Vec<NetworkInfo> {
		let networks = Networks::new_with_refreshed_list();
		let mut network_infos = Vec::new();
		for (network, data) in networks.list() {
			let mut ip_info_list: Vec<IpInfo> = Vec::new();

			for ip_network in data.ip_networks() {
				ip_info_list
					.push(IpInfo { ip_address: ip_network.addr, netmask: ip_network.prefix });
			}
			network_infos.push(NetworkInfo {
				name: network.to_string(),
				mac_addr: data.mac_address(),
				upload: format_float((data.total_received() as f32 / 1024.0) as f64, 2),
				download: format_float((data.total_transmitted() as f32 / 1024.0) as f64, 2),
				ip_info: ip_info_list,
			});
		}
		network_infos
	}

	/// 获取当前网络信息
	///
	/// 此函数可以获取当前网络信息，包括网络名称、MAC地址、上传速度、下载速度、IP地址等
	/// # 返回值
	///
	/// * [NetworkInfo] - 当前网络信息
	///
	#[cfg(feature = "network")]
	pub fn current_network() -> NetworkInfo {
		use std::thread::sleep;
		use std::time::Duration;
		let mut networks = Networks::new_with_refreshed_list();

		sleep(Duration::from_millis(100));

		networks.refresh(true);

		let process_network_data = |_: &str, data: &sysinfo::NetworkData| -> (Vec<IpInfo>, bool) {
			let mut ip_info_list: Vec<IpInfo> = Vec::new();
			let mut has_ipv4 = false;

			for ip_network in data.ip_networks() {
				ip_info_list
					.push(IpInfo { ip_address: ip_network.addr, netmask: ip_network.prefix });

				if ip_network.addr.is_ipv4() {
					has_ipv4 = true;
				}
			}

			(ip_info_list, has_ipv4)
		};
		let is_loopback = |name: &str| -> bool {
			name.starts_with("lo") || name.starts_with("Loopback") || name.contains("loopback")
		};

		for (network_name, data) in networks.list() {
			let (ip_info_list, has_ipv4) = process_network_data(network_name, data);

			let recent_traffic = data.received() + data.transmitted();
			if !is_loopback(network_name)
				&& has_ipv4 && !ip_info_list.is_empty()
				&& recent_traffic > 0
			{
				return NetworkInfo {
					name: network_name.to_string(),
					mac_addr: data.mac_address(),
					upload: format_float(data.received() as f64 / 1024.0, 2),
					download: format_float(data.transmitted() as f64 / 1024.0, 2),
					ip_info: ip_info_list,
				};
			}
		}

		for (network_name, data) in networks.list() {
			let (ip_info_list, has_ipv4) = process_network_data(network_name, data);

			if !is_loopback(network_name) && has_ipv4 && !ip_info_list.is_empty() {
				return NetworkInfo {
					name: network_name.to_string(),
					mac_addr: data.mac_address(),
					upload: 0.0,
					download: 0.0,
					ip_info: ip_info_list,
				};
			}
		}

		NetworkInfo {
			name: "unknown".to_string(),
			mac_addr: MacAddr([0, 0, 0, 0, 0, 0]),
			upload: 0.0,
			download: 0.0,
			ip_info: vec![],
		}
	}

	/// 获取进程信息
	/// 此函数可以获取进程信息，包括进程ID、进程名称、CPU使用率、内存使用率、已用内存等
	/// # 返回值
	///
	/// * [ProcessInfo] - 进程信息
	#[cfg(feature = "process")]
	pub fn process() -> ProcessInfo {
		use std::process;
		Self::process_with_pid(process::id())
	}

	#[cfg(feature = "process")]
	pub fn process_with_pid(pid: u32) -> ProcessInfo {
		use std::time::{SystemTime, UNIX_EPOCH};
		use sysinfo::{ProcessesToUpdate, System};
		let mut system = System::new();
		let pid = Pid::from_u32(pid);
		system.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);
		let process = system.process(pid);

		let name = if let Some(process) = process {
			process.name().to_string_lossy().into_owned()
		} else {
			"Unknown".to_string()
		};
		let start_time = process.map(|p| p.start_time()).unwrap_or(0);
		let run_time = process
			.map(|p| {
				let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
				let process_start_time = p.start_time();
				current_time.saturating_sub(process_start_time)
			})
			.unwrap_or(0);

		let cpu_usage = process.map(|p| format_float(p.cpu_usage() as f64, 2) as u8);

		let memory_usage = process.map(|p| {
			format_float(p.memory() as f64 / (system.total_memory() as f64) * 100.0, 2) as u8
		});

		let used_memory = match process {
			Some(process) => process.memory() as f32 / 1024.0 / 1024.0,
			None => 0.0,
		};

		ProcessInfo { pid, name, start_time, run_time, cpu_usage, memory_usage, used_memory }
	}

	/// 获取GPU信息
	///
	/// 此函数可以获取GPU信息，包括型号、已用内存、总内存、可用内存、使用率等
	/// # 返回值
	///
	/// * [GpuInfo] - GPU信息
	///
	#[cfg(feature = "gpu")]
	pub fn gpu() -> Option<GpuInfo> {
		let gpu = active_gpu();
		match gpu {
			Ok(gpu) => {
				let info = gpu.info();
				let gpu_usage = format_float(info.used_vram() as f64 / (1024.0 * 1024.0), 2) as f32;
				let gpu_total =
					format_float(info.total_vram() as f64 / (1024.0 * 1024.0), 2) as f32;
				Some(GpuInfo {
					gpu_model: gpu.model().to_string(),
					gpu_memory_used: gpu_usage,
					gpu_memory_total: gpu_total as f32,
					gpu_memory_free: gpu_total - gpu_usage,
					gpu_usage: info.load_pct() as u8,
				})
			}
			Err(_) => None,
		}
	}
}

fn format_float(value: f64, decimals: u32) -> f64 {
	let decimal_value = Decimal::from_f64(value).unwrap_or(Decimal::ZERO);
	let rounded = decimal_value.round_dp(decimals);
	rounded.to_f64().unwrap_or(0.0)
}
