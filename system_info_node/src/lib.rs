use napi_derive::napi;
use system_info::SystemInfo;

mod types;

#[napi]
/// 获取主机信息
pub fn get_host_info() -> types::HostInfo {
	SystemInfo::host().into()
}

#[napi]
/// 获取网络信息
pub fn get_network_info() -> Vec<types::NetworkInfo> {
	let network_infos = SystemInfo::network();
	network_infos.into_iter().map(|info| info.into()).collect()
}

#[napi]
/// 获取当前网络信息
pub fn get_current_network() -> types::NetworkInfo {
	SystemInfo::current_network().into()
}
#[napi]
/// 获取进程信息
pub fn get_process_info(pid: u32) -> types::ProcessInfo {
	SystemInfo::process_with_pid(pid).into()
}

#[napi]
/// 获取当前进程信息
pub fn get_current_process_info() -> types::ProcessInfo {
	SystemInfo::process().into()
}

#[napi]
/// 获取CPU信息
pub fn get_cpu_info() -> types::CpuInfo {
	SystemInfo::cpu().into()
}

#[napi]
/// 获取内存信息
pub fn get_memory_info() -> types::MemoryInfo {
	SystemInfo::memory().into()
}

#[napi]
/// 获取硬盘信息
pub fn get_disk_info() -> types::DiskInfo {
	SystemInfo::disk().into()
}

#[napi]
/// 获取GPU信息
pub fn get_gpu_info() -> Option<types::GpuInfo> {
	SystemInfo::gpu().map(|g| g.into())
}
