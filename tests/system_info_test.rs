use puniyu_system_info::{SystemInfo};


#[cfg(feature = "host")]
#[test]
fn test_host_info() {
    let host_info = SystemInfo::host();

    println!("主机信息: {:#?}", host_info);

    assert!(!host_info.host_name.is_empty());
    assert!(!host_info.os_name.is_empty());
    assert!(!host_info.os_version.is_empty());
    assert!(!host_info.os_type.is_empty());
    assert!(!host_info.boot_time.is_empty());

    println!("主机名: {}", host_info.host_name);
    println!("操作系统: {}", host_info.os_name);
    println!("系统版本: {}", host_info.os_version);
    println!("系统类型: {}", host_info.os_type);
    println!("启动时间: {}", host_info.boot_time);
}

#[cfg(feature = "cpu")]
#[test]
fn test_cpu_info() {
    let cpu_info = SystemInfo::cpu();

    println!("CPU信息: {:#?}", cpu_info);

    assert!(!cpu_info.cpu_model.is_empty());
    assert!(cpu_info.cpu_cores > 0);

    println!("CPU型号: {}", cpu_info.cpu_model);
    println!("CPU核心数: {}", cpu_info.cpu_cores);

    if let Some(frequency) = cpu_info.cpu_frequency {
        println!("CPU频率: {} GHz", frequency);
    }

    if let Some(usage) = cpu_info.cpu_usage {
        println!("CPU使用率: {}%", usage);
    }
}

#[cfg(feature = "process")]
#[test]
fn test_process_info() {
    let process_info = SystemInfo::process();

    println!("进程信息: {:#?}", process_info);

    assert!(process_info.pid.as_u32() > 0);
    assert!(!process_info.name.is_empty());

    println!("进程ID: {}", process_info.pid);
    println!("进程名称: {}", process_info.name);
    println!("进程启动时间: {}", process_info.start_time);
    println!("进程运行时间: {}", process_info.run_time);

    if let Some(cpu_usage) = process_info.cpu_usage {
        println!("进程CPU使用率: {}%", cpu_usage);
    }

    if let Some(memory_usage) = process_info.memory_usage {
        println!("进程内存使用率: {}%", memory_usage);
    }

    println!("进程已用内存: {:.2} MB", process_info.used_memory);
}

#[cfg(feature = "memory")]
#[test]
fn test_memory_info() {
    let memory_info = SystemInfo::memory();

    println!("内存信息: {:#?}", memory_info);

    assert!(memory_info.total_memory > 0.0);
    assert!(memory_info.used_memory >= 0.0);
    assert!(memory_info.free_memory >= 0.0);

    println!("总内存: {} MB", memory_info.total_memory);
    println!("已用内存: {} MB", memory_info.used_memory);
    println!("可用内存: {} MB", memory_info.free_memory);

    if let Some(usage) = memory_info.memory_usage {
        println!("内存使用率: {}%", usage);
    }
}

#[cfg(feature = "disk")]
#[test]
fn test_disk_info() {
    let disk_info = SystemInfo::disk();

    println!("硬盘信息: {:#?}", disk_info);

    assert!(disk_info.total_disk_space > 0.0);
    assert!(disk_info.total_used_space >= 0.0);
    assert!(disk_info.total_free_space >= 0.0);

    println!("总磁盘空间: {} GB", disk_info.total_disk_space);
    println!("已用磁盘空间: {} GB", disk_info.total_used_space);
    println!("可用磁盘空间: {} GB", disk_info.total_free_space);
    println!("磁盘使用率: {}%", disk_info.total_usage);

    for disk in &disk_info.disks {
        assert!(!disk.name.is_empty());
        println!("  磁盘名称: {}", disk.name);
        println!("  总空间: {} GB", disk.total_space);
        println!("  已用空间: {} GB", disk.used_space);
        println!("  可用空间: {} GB", disk.free_space);
        println!("  使用率: {}%", disk.usage);
    }
}

#[cfg(feature = "gpu")]
#[test]
fn test_gpu_info() {
    let gpu_info = SystemInfo::gpu();

    if let Some(gpu) = gpu_info {
        assert!(!gpu.gpu_model.is_empty());
        assert!(gpu.gpu_memory_total > 0.0);
        assert!(gpu.gpu_memory_used >= 0.0);
        assert!(gpu.gpu_memory_free >= 0.0);
        assert!(gpu.gpu_usage <= 100);
    } else {
        dbg!("未检测到GPU");
    }
}
