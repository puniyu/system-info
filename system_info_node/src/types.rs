use chrono::{DateTime, Utc};
use napi_derive::napi;

#[derive(Debug, Clone)]
#[napi(object)]
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
	pub uptime: u32,
}

impl From<system_info::HostInfo> for HostInfo {
	fn from(host_info: system_info::HostInfo) -> Self {
		Self {
			host_name: host_info.host_name,
			os_name: host_info.os_name,
			os_version: host_info.os_version,
			os_type: host_info.os_type,
			arch: host_info.arch,
			boot_time: host_info.boot_time,
			uptime: host_info.uptime as u32,
		}
	}
}

#[derive(Debug, Clone)]
#[napi(object)]
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
	pub mac_addr: String,
}

impl From<system_info::NetworkInfo> for NetworkInfo {
	fn from(network_info: system_info::NetworkInfo) -> Self {
		Self {
			name: network_info.name,
			ip_info: network_info.ip_info.into_iter().map(|ip_info| ip_info.into()).collect(),
			upload: network_info.upload,
			download: network_info.download,
			mac_addr: network_info.mac_addr.to_string(),
		}
	}
}

#[derive(Debug, Clone)]
#[napi(object)]
pub struct IpInfo {
	/// ip地址
	pub ip_address: String,
	/// 子网掩码
	pub netmask: u8,
}

impl From<system_info::IpInfo> for IpInfo {
	fn from(ip_info: system_info::IpInfo) -> Self {
		Self { ip_address: ip_info.ip_address.to_string(), netmask: ip_info.netmask }
	}
}

#[derive(Debug, Clone)]
#[napi(object)]
pub struct ProcessInfo {
	/// 进程ID
	pub pid: u32,
	/// 进程名称
	pub name: String,
	/// 进程启动时间
	pub start_time: u32,
	/// 进程运行时间，单位：秒
	pub run_time: u32,
	/// 进程CPU使用率
	pub cpu_usage: Option<u8>,
	/// 进程内存使用率
	pub memory_usage: Option<u8>,
	/// 进程已用内存(单位: MB)
	pub used_memory: f64,
}

impl From<system_info::ProcessInfo> for ProcessInfo {
	fn from(process_info: system_info::ProcessInfo) -> Self {
		Self {
			pid: process_info.pid.as_u32(),
			name: process_info.name,
			start_time: process_info.start_time as u32,
			run_time: process_info.run_time as u32,
			cpu_usage: process_info.cpu_usage,
			memory_usage: process_info.memory_usage,
			used_memory: process_info.used_memory,
		}
	}
}

#[derive(Debug, Clone)]
#[napi(object)]
pub struct CpuInfo {
	/// CPU型号
	pub cpu_model: String,
	/// CPU核心数
	pub cpu_cores: u32,
	/// CPU频率(单位: GHz)
	pub cpu_frequency: Option<f64>,
	/// CPU使用率
	pub cpu_usage: Option<u8>,
}

impl From<system_info::CpuInfo> for CpuInfo {
	fn from(cpu_info: system_info::CpuInfo) -> Self {
		Self {
			cpu_model: cpu_info.cpu_model,
			cpu_cores: cpu_info.cpu_cores as u32,
			cpu_frequency: cpu_info.cpu_frequency.map(|d| d as f64),
			cpu_usage: cpu_info.cpu_usage,
		}
	}
}

#[derive(Debug, Clone)]
#[napi(object)]
pub struct MemoryInfo {
	/// 总内存(单位: MB)
	pub total: f64,
	/// 内存使用率
	pub usage: Option<u32>,
	/// 已用内存(单位: MB)
	pub used_memory: f64,
	/// 可用内存(单位: MB)
	pub free_memory: f64,
	/// 交换内存(单位: MB)
	pub swap_memory_total: Option<f64>,
	/// 交换内存已用(单位: MB)
	pub swap_memory_used: Option<f64>,
	/// 交换内存可用(单位: MB)
	pub swap_memory_free: Option<f64>,
	/// 交换内存使用率
	pub swap_memory_usage: Option<u32>,
}

impl From<system_info::MemoryInfo> for MemoryInfo {
	fn from(memory_info: system_info::MemoryInfo) -> Self {
		Self {
			total: memory_info.total as f64,
			usage: memory_info.usage.map(|d| d as u32),
			used_memory: memory_info.used_memory as f64,
			free_memory: memory_info.free_memory as f64,
			swap_memory_total: memory_info.swap_memory_total.map(|d| d as f64),
			swap_memory_used: memory_info.swap_memory_used.map(|d| d as f64),
			swap_memory_free: memory_info.swap_memory_free.map(|d| d as f64),
			swap_memory_usage: memory_info.swap_memory_usage.map(|d| d as u32),
		}
	}
}

#[derive(Debug, Clone)]
#[napi(object)]
pub struct DiskInfo {
	/// 总磁盘空间(单位: GB)
	pub total_disk_space: f64,
	/// 总已用磁盘空间(单位: GB)
	pub total_used_space: f64,
	/// 总可用磁盘空间(单位: GB)
	pub total_free_space: f64,
	/// 总体磁盘使用率
	pub total_usage: f64,
	/// 各个磁盘详细信息
	pub disks: Vec<DiskDetail>,
}

impl From<system_info::DiskInfo> for DiskInfo {
	fn from(disk_info: system_info::DiskInfo) -> Self {
		Self {
			total_disk_space: disk_info.total_disk_space as f64,
			total_used_space: disk_info.total_used_space as f64,
			total_free_space: disk_info.total_free_space as f64,
			total_usage: disk_info.total_usage as f64,
			disks: disk_info.disks.into_iter().map(|d| d.into()).collect(),
		}
	}
}
#[derive(Debug, Clone)]
#[napi(object)]
pub struct DiskDetail {
	/// 磁盘名称
	pub name: String,
	/// 总磁盘空间(单位: GB)
	pub total_space: f64,
	/// 已用磁盘空间(单位: GB)
	pub used_space: f64,
	/// 可用磁盘空间(单位: GB)
	pub free_space: f64,
	/// 磁盘使用率
	pub usage: f64,
}

impl From<system_info::DiskDetail> for DiskDetail {
	fn from(disk_detail: system_info::DiskDetail) -> Self {
		Self {
			name: disk_detail.name,
			total_space: disk_detail.total_space as f64,
			used_space: disk_detail.used_space as f64,
			free_space: disk_detail.free_space as f64,
			usage: disk_detail.usage as f64,
		}
	}
}

#[derive(Debug, Clone)]
#[napi(object)]
pub struct GpuInfo {
	/// GPU型号
	pub model: String,
	///  GPU已用内存(单位: MB)
	pub memory_used: f64,
	/// GPU总内存(单位: MB)
	pub memory_total: f64,
	///  GPU可用内存(单位: MB)
	pub memory_free: f64,
	/// GPU使用率
	pub usage: u32,
}

impl From<system_info::GpuInfo> for GpuInfo {
	fn from(gpu_info: system_info::GpuInfo) -> Self {
		Self {
			model: gpu_info.model,
			memory_used: gpu_info.memory_used as f64,
			memory_total: gpu_info.memory_total as f64,
			memory_free: gpu_info.memory_free as f64,
			usage: gpu_info.usage as u32,
		}
	}
}
