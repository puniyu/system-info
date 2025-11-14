#[cfg(feature = "host")]
use chrono::Utc;
#[cfg(feature = "process")]
use chrono::{DateTime, TimeZone};
#[cfg(feature = "process")]
use std::time::{SystemTime, UNIX_EPOCH};
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
	pub boot_time: DateTime<Utc>,
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
#[cfg(feature = "process")]
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
	pub cpu_usage: Option<f32>,
	/// 进程内存使用率
	pub memory_usage: Option<f32>,
	/// 进程已用内存(单位: MB)
	pub used_memory: f64,
}
#[derive(Debug, Clone)]
#[cfg(feature = "cpu")]
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
}

#[derive(Debug, Clone)]
#[cfg(feature = "gpu")]
pub struct GpuInfo {
	/// GPU型号
	pub model: String,
	///  GPU已用内存(单位: MB)
	pub memory_used: f32,
	/// GPU总内存(单位: MB)
	pub memory_total: f32,
	///  GPU可用内存(单位: MB)
	pub memory_free: f32,
	/// GPU使用率
	pub usage: u8,
}

#[derive(Debug, Clone)]
#[cfg(feature = "memory")]
pub struct MemoryInfo {
	/// 总内存(单位: MB)
	pub total: u64,
	/// 内存使用率
	pub usage: f32,
	/// 已用内存(单位: MB)
	pub used: u64,
	/// 可用内存(单位: MB)
	pub free: u64,
	/// 交换内存(单位: MB)
	pub swap_total: Option<u64>,
	/// 交换内存已用(单位: MB)
	pub swap_used: Option<u64>,
	/// 交换内存可用(单位: MB)
	pub swap_free: Option<u64>,
	/// 交换内存使用率
	pub swap_usage: Option<f32>,
}

#[derive(Debug, Clone)]
#[cfg(feature = "disk")]
pub struct DiskDetail {
	/// 磁盘名称
	pub name: String,
	/// 磁盘挂载点
	pub mount: String,
	/// 总磁盘空间(单位: GB)
	pub total_space: u64,
	/// 已用磁盘空间(单位: GB)
	pub used_space: u64,
	/// 可用磁盘空间(单位: GB)
	pub free_space: u64,
	/// 磁盘使用率
	pub usage: f32,
}

#[derive(Debug, Clone)]
#[cfg(feature = "disk")]
pub struct DiskInfo {
	/// 总磁盘空间(单位: GB)
	pub total_space: u64,
	/// 总已用磁盘空间(单位: GB)
	pub total_used_space: u64,
	/// 总可用磁盘空间(单位: GB)
	pub total_free_space: u64,
	/// 总体磁盘使用率
	pub total_usage: f64,
	/// 磁盘读速度(单位: KB/S)
	pub read_speed: f32,
	/// 磁盘写入速度(单位: KB/S)
	pub write_speed: f32,
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
		let boot_time_secs = System::boot_time();
		let boot_time = Utc
			.timestamp_opt(boot_time_secs as i64, 0)
			.single()
			.expect("Invalid boot time timestamp");
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
		let cpu = &system.cpus()[0];

		CpuInfo {
			model_name: cpu.brand().to_string(),
			physical_cores: num_cpus::get_physical() as u32,
			logical_cores: num_cpus::get() as u32,
			frequency: cpu.frequency() as f32 / 1000.0,
			usage: Some(system.global_cpu_usage().round()),
		}
	}

	/// 获取内存信息
	///
	/// 此函数可以获取内存信息，包括总内存、已用内存、可用内存、内存使用率等
	/// # 返回值
	///
	/// * [MemoryInfo] - 内存信息
	///
	#[cfg(feature = "memory")]
	pub fn memory() -> MemoryInfo {
		use sysinfo::System;
		let mut system = System::new();
		system.refresh_memory();

		let total_memory = system.total_memory() / 1024 / 1024;
		let used_memory = system.used_memory() / 1024 / 1024;
		let swap_memory_total = system.total_swap() / 1024 / 1024;
		let swap_memory_used = system.used_swap() / 1024 / 1024;

		let usage =
			if total_memory > 0 { (used_memory as f32 / total_memory as f32) * 100.0 } else { 0.0 };

		let swap_memory_usage = if swap_memory_total > 0 {
			Some((swap_memory_used as f32 / swap_memory_total as f32) * 100.0)
		} else {
			None
		};

		MemoryInfo {
			total: total_memory,
			usage,
			used: used_memory,
			free: total_memory - used_memory,
			swap_total: if swap_memory_total > 0 { Some(swap_memory_total) } else { None },
			swap_used: if swap_memory_total > 0 { Some(swap_memory_used) } else { None },
			swap_free: if swap_memory_total > 0 {
				Some(swap_memory_total - swap_memory_used)
			} else {
				None
			},
			swap_usage: swap_memory_usage,
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
	pub fn disk() -> DiskInfo {
		use sysinfo::{Disks, ProcessesToUpdate, System};
		let mut s = System::new_all();
		s.refresh_processes(ProcessesToUpdate::All, true);

		let mut read_speed = 0f32;
		let mut write_speed = 0f32;
		for process in s.processes() {
			let disk_usage = process.1.disk_usage();
			read_speed += disk_usage.read_bytes as f32 / 1024.0;
			write_speed += disk_usage.written_bytes as f32 / 1024.0;
		}

		let disks = Disks::new_with_refreshed_list();

		let mut total_disk_space = 0u64;
		let mut total_used_space = 0u64;
		let mut total_free_space = 0u64;
		let mut disk_details = Vec::new();

		for disk in disks.list() {
			let total_space = disk.total_space() / (1024 * 1024 * 1024);
			let free_space = disk.available_space() / (1024 * 1024 * 1024);
			let used_space = total_space - free_space;

			let usage = if total_space > 0 {
				(used_space as f64 / total_space as f64) * 100.0
			} else {
				0.0
			};

			let disk_detail = DiskDetail {
				name: disk.name().to_string_lossy().to_string(),
				mount: disk.mount_point().to_string_lossy().trim_end_matches('\\').to_string(),
				total_space,
				used_space,
				free_space,
				usage: usage.round() as f32,
			};

			total_disk_space += total_space;
			total_used_space += used_space;
			total_free_space += free_space;
			disk_details.push(disk_detail);
		}

		let total_usage = if total_disk_space > 0 {
			(total_used_space / total_disk_space) as f64 * 100.0
		} else {
			0.0
		};

		DiskInfo {
			total_space: total_disk_space,
			total_used_space,
			total_free_space,
			total_usage,
			read_speed: read_speed.round(),
			write_speed: write_speed.round(),
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
				upload: round((data.total_received() as f32 / 1024.0) as f64),
				download: round((data.total_transmitted() as f32 / 1024.0) as f64),
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
					upload: round(data.received() as f64 / 1024.0),
					download: round(data.transmitted() as f64 / 1024.0),
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
		let run_time = {
			let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
			let process_start_time = process.map(|p| p.start_time()).unwrap_or(0);
			current_time.saturating_sub(process_start_time)
		};

		let cpu_usage = process.map(|p| round(p.cpu_usage() as f64) as f32);

		let memory_usage = process.and_then(|p| {
			let total_memory = system.total_memory();
			if total_memory > 0 {
				Some(round(p.memory() as f64 / (total_memory as f64) * 100.0) as f32)
			} else {
				None
			}
		});

		let used_memory = match process {
			Some(process) => process.memory() as f64 / 1024.0 / 1024.0,
			None => 0.0,
		};

		ProcessInfo { pid, name, start_time, run_time, cpu_usage, memory_usage, used_memory }
	}

	#[cfg(feature = "process")]
	pub fn process_all() -> Vec<ProcessInfo> {
		use sysinfo::{ProcessesToUpdate, System};
		let mut system = System::new();
		system.refresh_processes(ProcessesToUpdate::All, true);
		let total_memory = system.total_memory();
		system
			.processes()
			.values()
			.map(|process| {
				let cpu_usage = {
					let usage = process.cpu_usage();
					if usage > 0.0 { Some(usage.round()) } else { None }
				};
				let used_memory = process.memory() as f64 / 1024.0 / 1024.0;
				let memory_usage = if total_memory > 0 {
					Some((process.memory() as f32 / total_memory as f32 * 100.0).round())
				} else {
					None
				};

				let run_time = {
					let current_time =
						SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
					let process_start_time = process.start_time();
					current_time.saturating_sub(process_start_time)
				};

				ProcessInfo {
					pid: process.pid(),
					name: process.name().to_string_lossy().to_string(),
					start_time: process.start_time(),
					run_time,
					cpu_usage,
					memory_usage,
					used_memory,
				}
			})
			.collect()
	}

	/// 获取GPU信息
	///
	/// 此函数可以获取GPU信息，包括型号、已用内存、总内存、可用内存、使用率等
	///
	/// 暂时只支持 Windows
	/// # 返回值
	///
	/// * [GpuInfo] - GPU信息
	///
	#[cfg(feature = "gpu")]
	pub fn gpu() -> Option<GpuInfo> {
		#[cfg(not(target_os = "windows"))]
		{
			return None;
		}

		#[cfg(target_os = "windows")]
		{
			use gfxinfo::active_gpu;
			let gpu = active_gpu();
			match gpu {
				Ok(gpu) => {
					let info = gpu.info();
					let gpu_usage = round(info.used_vram() as f64 / (1024.0 * 1024.0)) as f32;
					let gpu_total = round(info.total_vram() as f64 / (1024.0 * 1024.0)) as f32;
					Some(GpuInfo {
						model: gpu.model().to_string(),
						memory_used: gpu_usage,
						memory_total: gpu_total,
						memory_free: gpu_total - gpu_usage,
						usage: info.load_pct() as u8,
					})
				}
				Err(_) => None,
			}
		}
	}
}

fn round(value: f64) -> f64 {
	(value * 100.0).round() / 100.0
}
