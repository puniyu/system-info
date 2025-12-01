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
	/// 上传速度(单位: KB/S)
	pub upload: f64,
	/// 下载速度(单位: KB/S)
	pub download: f64,
	/// 总上传流量(单位: MB)
	pub total_upload: f64,
	/// 总下载流量(单位: MB)
	pub total_download: f64,
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
			total_upload: network_info.total_upload,
			total_download: network_info.total_download,
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
	/// 子进程列表
	pub sub_list: Option<Vec<ProcessInfo>>,
	/// 进程启动时间
	pub start_time: u32,
	/// 进程运行时间，单位：秒
	pub run_time: u32,
	/// 进程CPU使用率
	pub cpu_usage: Option<f64>,
	/// 进程内存使用率
	pub memory_usage: Option<f64>,
	/// 进程已用内存(单位: MB)
	pub used_memory: f64,
}

impl From<system_info::ProcessInfo> for ProcessInfo {
	fn from(process_info: system_info::ProcessInfo) -> Self {
		Self {
			pid: process_info.pid.as_u32(),
			name: process_info.name,
			sub_list: process_info.sub_list.map(|list| list.into_iter().map(|p| p.into()).collect()),
			start_time: process_info.start_time as u32,
			run_time: process_info.run_time as u32,
			cpu_usage: process_info.cpu_usage.map(|d| d as f64),
			memory_usage: process_info.memory_usage.map(|d| d as f64),
			used_memory: process_info.used_memory,
		}
	}
}

#[derive(Debug, Clone)]
#[napi(object)]
pub struct CpuInfo {
	/// CPU名称
	pub model_name: String,
	/// CPU核心数
	pub physical_cores: u32,
	/// CPU 线程数
	pub logical_cores: u32,
	/// CPU基本频率(单位: GHz)
	pub frequency: f64,
	/// CPU使用率
	pub usage: Option<f64>,
}

impl From<system_info::CpuInfo> for CpuInfo {
	fn from(cpu_info: system_info::CpuInfo) -> Self {
		Self {
			model_name: cpu_info.model_name,
			physical_cores: cpu_info.physical_cores,
			logical_cores: cpu_info.logical_cores,
			frequency: cpu_info.frequency as f64,
			usage: cpu_info.usage.map(|d| d as f64),
		}
	}
}

#[derive(Debug, Clone)]
#[napi(object)]
pub struct MemoryInfo {
	/// 总内存(单位: MB)
	pub total: u32,
	/// 内存使用率
	pub usage: f64,
	/// 已用内存(单位: MB)
	pub used: u32,
	/// 可用内存(单位: MB)
	pub free: u32,
	/// 交换内存(单位: MB)
	pub swap_total: Option<u32>,
	/// 交换内存已用(单位: MB)
	pub swap_used: Option<u32>,
	/// 交换内存可用(单位: MB)
	pub swap_free: Option<u32>,
	/// 交换内存使用率
	pub swap_usage: Option<f64>,
}

impl From<system_info::MemoryInfo> for MemoryInfo {
	fn from(memory_info: system_info::MemoryInfo) -> Self {
		Self {
			total: memory_info.total as u32,
			usage: memory_info.usage as f64,
			used: memory_info.used as u32,
			free: memory_info.free as u32,
			swap_total: memory_info.swap_total.map(|d| d as u32),
			swap_used: memory_info.swap_used.map(|d| d as u32),
			swap_free: memory_info.swap_free.map(|d| d as u32),
			swap_usage: memory_info.swap_usage.map(|d| d as f64),
		}
	}
}

#[derive(Debug, Clone)]
#[napi(object)]
pub struct DiskInfo {
	/// 总磁盘空间(单位: GB)
	pub total_space: u32,
	/// 总已用磁盘空间(单位: GB)
	pub total_used_space: u32,
	/// 总可用磁盘空间(单位: GB)
	pub total_free_space: u32,
	/// 总体磁盘使用率
	pub total_usage: f64,
	/// 磁盘读速度(单位: MB/S)
	pub read_speed: f64,
	/// 磁盘写入速度(单位: MB/S)
	pub write_speed: f64,
	/// 各个磁盘详细信息
	pub disks: Vec<DiskDetail>,
}

impl From<system_info::DiskInfo> for DiskInfo {
	fn from(disk_info: system_info::DiskInfo) -> Self {
		Self {
			total_space: disk_info.total_space as u32,
			total_used_space: disk_info.total_used_space as u32,
			total_free_space: disk_info.total_free_space as u32,
			total_usage: disk_info.total_usage,
			read_speed: disk_info.read_speed as f64,
			write_speed: disk_info.write_speed as f64,
			disks: disk_info.disks.into_iter().map(|d| d.into()).collect(),
		}
	}
}
#[derive(Debug, Clone)]
#[napi(object)]
pub struct DiskDetail {
	/// 磁盘名称
	pub name: String,
	/// 磁盘挂载点
	pub mount: String,
	/// 总磁盘空间(单位: GB)
	pub total_space: u32,
	/// 已用磁盘空间(单位: GB)
	pub used_space: u32,
	/// 可用磁盘空间(单位: GB)
	pub free_space: u32,
	/// 磁盘使用率
	pub usage: f64,
}

impl From<system_info::DiskDetail> for DiskDetail {
	fn from(disk_detail: system_info::DiskDetail) -> Self {
		Self {
			name: disk_detail.name,
			mount: disk_detail.mount,
			total_space: disk_detail.total_space as u32,
			used_space: disk_detail.used_space as u32,
			free_space: disk_detail.free_space as u32,
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
