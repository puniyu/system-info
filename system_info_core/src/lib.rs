#[cfg(feature = "host")]
mod host;
#[cfg(feature = "host")]
pub use host::HostInfo;

#[cfg(feature = "cpu")]
mod cpu;

#[cfg(feature = "cpu")]
pub use cpu::CpuInfo;

#[cfg(feature = "memory")]
mod memory;

pub use memory::MemoryInfo;

#[cfg(feature = "disk")]
mod disk;

#[cfg(feature = "disk")]
pub use disk::{DiskDetail, DiskInfo};

#[cfg(feature = "network")]
mod network;
#[cfg(feature = "network")]
pub use network::{IpInfo, NetworkInfo};

#[cfg(feature = "process")]
mod process;

#[cfg(feature = "process")]
pub use process::ProcessInfo;

#[cfg(feature = "gpu")]
mod gpu;
#[cfg(feature = "gpu")]
pub use gpu::GpuInfo;

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
		HostInfo::default()
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
		CpuInfo::default()
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
		MemoryInfo::default()
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
		DiskInfo::default()
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
		NetworkInfo::all()
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
		NetworkInfo::default()
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

	/// 获取指定 PID 的进程信息及其子进程
	///
	/// 此函数可以获取指定进程的详细信息，包括进程ID、进程名称、CPU使用率、内存使用率、已用内存，
	/// 以及该进程的所有子进程信息
	///
	/// # 参数
	///
	/// * `pid` - 进程ID
	///
	/// # 返回值
	///
	/// * [ProcessInfo] - 进程信息，包含子进程列表
	#[cfg(feature = "process")]
	pub fn process_with_pid(pid: u32) -> ProcessInfo {
		ProcessInfo::new(pid)
	}

	#[cfg(feature = "process")]
	pub fn process_all() -> Vec<ProcessInfo> {
		ProcessInfo::all()
	}

	/// 获取GPU信息
	///
	/// 此函数可以获取GPU信息，包括型号、已用内存、总内存、可用内存、使用率等
	///
	/// # 返回值
	///
	/// * `Option<GpuInfo>` - GPU信息，如果无法检测到GPU则返回None
	///
	#[cfg(feature = "gpu")]
	pub fn gpu() -> Option<GpuInfo> {
		GpuInfo::new()
	}
}

fn round(value: f64) -> f64 {
	(value * 100.0).round() / 100.0
}
