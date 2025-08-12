use chrono::{offset::FixedOffset, TimeZone, Utc};
#[cfg(feature = "gpu")]
use gfxinfo::active_gpu;
use rust_decimal::{prelude::{FromPrimitive, ToPrimitive}, Decimal};
use std::{env, process};
use std::thread::sleep;
use sysinfo::{Disks, Pid, ProcessesToUpdate, System};

#[derive(Debug)]
pub struct SystemInfo {
    /// 主机信息
    #[cfg(feature = "host")]
    pub host: HostInfo,
    /// CPU信息
    #[cfg(feature = "cpu")]
    pub cpu: CpuInfo,
    /// 内存信息
    #[cfg(feature = "memory")]
    pub memory: MemoryInfo,
    /// 硬盘信息
    #[cfg(feature = "disk")]
    pub disk: DiskInfo,
    /// 进程信息
    #[cfg(feature = "process")]
    pub process: ProcessInfo,
    /// GPU信息
    #[cfg(feature = "gpu")]
    pub gpu: Option<GpuInfo>,
}

#[derive(Debug)]
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
    /// 系统启动时间， 上海时区
    pub boot_time: String,
}

#[derive(Debug)]
pub struct ProcessInfo {
    /// 进程ID
    pub pid: Pid,
    /// 进程名称
    pub name: String,
    /// 进程CPU使用率
    pub cpu_usage: Option<u8>,
    /// 进程内存使用率
    pub memory_usage: Option<u8>,
    /// 进程已用内存(单位: MB)
    pub used_memory: f32,
}
#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
#[cfg(feature = "memory")]
pub struct MemoryInfo {
    /// 总内存(单位: MB)
    pub total_memory: f32,
    /// 已用内存(单位: MB)
    pub used_memory: f32,
    /// 可用内存(单位: MB)
    pub free_memory: f32,
    /// 内存使用率
    pub memory_usage: Option<u8>,
}

#[derive(Debug)]
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

#[derive(Debug)]
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

/// 获取系统信息
///
/// 此函数可以获取系统信息，包括CPU、内存、磁盘、Bot信息等
/// # 返回值
///
/// * [SystemInfo] - 系统信息
///
pub fn get_system_info() -> SystemInfo {
    let process = get_procss_info() ;
    #[cfg(feature = "host")]
    let host = get_host_info();
    #[cfg(feature = "cpu")]
    let cpu = get_cpu_info();
    #[cfg(feature = "memory")]
    let memory = get_memory_info();
    #[cfg(feature = "disk")]
    let disk = get_disk_info();
    #[cfg(feature = "gpu")]
    let gpu = get_gpu_info();

    SystemInfo {
        #[cfg(feature = "host")]
        host,
        #[cfg(feature = "process")]
        process,
        #[cfg(feature = "cpu")]
        cpu,
        #[cfg(feature = "memory")]
        memory,
        #[cfg(feature = "disk")]
        disk,
        #[cfg(feature = "gpu")]
        gpu,
    }
}

/// 获取CPU信息
///
/// 此函数可以获取CPU信息，包括型号、核心数、频率、使用率等
/// # 返回值
///
/// * [CpuInfo] - CPU信息
///
#[cfg(feature = "cpu")]
pub fn get_cpu_info() -> CpuInfo {
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

    let cpu_model = if !cpus.is_empty() {
        cpus[0].brand().to_string()
    } else {
        "Unknown".to_string()
    };

    let cpu_frequency = if !cpus.is_empty() {
        Some(cpus[0].frequency() as f32)
    } else {
        None
    };

    CpuInfo {
        cpu_usage,
        cpu_frequency,
        cpu_cores,
        cpu_model,
    }
}

/// 获取主机信息
///
/// 此函数可以获取主机信息，包括主机名、操作系统名、操作系统版本、操作系统类型、系统启动时间等
/// # 返回值
///
/// * [HostInfo] - 主机信息
///
#[cfg(feature = "host")]
pub fn get_host_info() -> HostInfo {
    let hostname = System::host_name().unwrap();
    let os_name = System::name().unwrap();
    let os_version = System::os_version().unwrap();
    let os_type = env::consts::OS.to_string();
    let boot_time_timestamp = System::boot_time();
    let boot_time = {
        let utc_time = Utc.timestamp_opt(boot_time_timestamp as i64, 0).unwrap();
        let shanghai_offset = FixedOffset::east_opt(8 * 3600).unwrap();
        utc_time.with_timezone(&shanghai_offset).format("%Y-%m-%d %H:%M:%S").to_string()
    };
    HostInfo {
        host_name: hostname,
        os_name,
        os_version,
        os_type,
        boot_time
    }
}
/// 获取GPU信息
///
/// 此函数可以获取GPU信息，包括型号、已用内存、总内存、可用内存、使用率等
/// # 返回值
///
/// * [GpuInfo] - GPU信息
///
#[cfg(feature = "gpu")]
pub fn get_gpu_info() -> Option<GpuInfo> {
    let gpu = active_gpu();
    match gpu {
        Ok(gpu) =>
            {
                let info = gpu.info();
                let gpu_usage = format_to_f32(info.used_vram() as f64 / (1024.0 * 1024.0), 2);
                let gpu_total = format_to_f32(info.total_vram() as f64 / (1024.0 * 1024.0), 2);
                Some(GpuInfo {
                    gpu_model: gpu.model().to_string(),
                    gpu_memory_used: gpu_usage,
                    gpu_memory_total: gpu_total,
                    gpu_memory_free: gpu_total - gpu_usage,
                    gpu_usage: info.load_pct() as u8,
                })
            },
        Err(_) => None,
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
pub fn get_memory_info() -> MemoryInfo {
    let mut system = System::new();
    system.refresh_memory();

    let total_memory = system.total_memory() / 1024 / 1024;
    let used_memory = system.used_memory() / 1024 / 1024;
    let free_memory = system.free_memory() / 1024 / 1024;

    let total_memory_f32 = format_to_f32(total_memory as f32, 2);
    let used_memory_f32 = format_to_f32(used_memory as f32, 2);
    let free_memory_f32 = format_to_f32(free_memory as f32, 2);

    let memory_usage = Some(((used_memory as f32 / total_memory as f32) * 100.0) as u8);

    MemoryInfo {
        total_memory: total_memory_f32,
        used_memory: used_memory_f32,
        free_memory: free_memory_f32,
        memory_usage,
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
pub fn get_disk_info() -> DiskInfo {
    let disks = Disks::new_with_refreshed_list();

    let mut total_disk_space = 0f32;
    let mut total_used_space = 0f32;
    let mut total_free_space = 0f32;
    let mut disk_details = Vec::new();

    for disk in disks.list() {
        let total_space = disk.total_space() as f32 / (1024.0 * 1024.0 * 1024.0);
        let free_space = disk.available_space() as f32 / (1024.0 * 1024.0 * 1024.0);
        let used_space = total_space - free_space;

        let usage = if disk.total_space() > 0 {
            (used_space / total_space) * 100.0
        } else {
            0.0
        };

        let disk_detail = DiskDetail {
            name: disk.name().to_string_lossy().to_string(),
            total_space: format_to_f32(total_space, 2),
            used_space: format_to_f32(used_space, 2),
            free_space: format_to_f32(free_space, 2),
            usage: format_to_f32(usage, 2),
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
        total_disk_space: format_to_f32(total_disk_space, 2),
        total_used_space: format_to_f32(total_used_space, 2),
        total_free_space: format_to_f32(total_free_space, 2),
        total_usage: format_to_f32(total_usage, 2),
        disks: disk_details,
    }
}


/// 获取进程信息
/// 此函数可以获取进程信息，包括进程ID、进程名称、CPU使用率、内存使用率、已用内存等
/// # 返回值
///
/// * [ProcessInfo] - 进程信息
#[cfg(feature = "process")]
pub fn get_procss_info() -> ProcessInfo{
    let current_pid = Pid::from_u32(process::id());
    let mut system = System::new();
    system.refresh_processes(ProcessesToUpdate::Some(&[current_pid]), true);
    let process = system.process(current_pid);

    let name = if let Some(process) = process {
        process.name().to_string_lossy().into_owned()
    } else {
        "Unknown".to_string()
    };

    let cpu_usage = process.map(|p| format_to_f32(p.cpu_usage(), 2) as u8);

    let memory_usage = process.map(|p| {
        format_to_f32(p.memory() as f64 / (system.total_memory() as f64) * 100.0, 2) as u8
    });

    let used_memory = match process {
        Some(process) => {
            process.memory() as f32 / 1024.0 / 1024.0
        }
        None => 0.0
    };

    ProcessInfo {
        pid: current_pid,
        name,
        cpu_usage,
        memory_usage,
        used_memory,
    }
}


fn format_to_f32<T>(value: T, decimals: u32) -> f32
where
    T: Into<f64>,
{
    let decimal_value = Decimal::from_f64(value.into()).unwrap_or(Decimal::ZERO);
    let rounded = decimal_value.round_dp(decimals);
    rounded.to_f32().unwrap_or(0.0)
}

