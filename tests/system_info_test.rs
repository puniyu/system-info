use puniyu_system_info::{get_system_info};

#[test]
fn test_get_system_info() {
    let system_info = get_system_info();

    assert!(true);

    println!("{:#?}", system_info);
}

#[cfg(feature = "host")]
#[test]
fn test_host_info() {
    let system_info = get_system_info();

    // 验证主机信息存在
    assert!(!system_info.host.host_name.is_empty());
    assert!(!system_info.host.os_name.is_empty());
    assert!(!system_info.host.os_version.is_empty());
    assert!(!system_info.host.os_type.is_empty());
    assert!(!system_info.host.boot_time.is_empty());

    println!("主机名: {}", system_info.host.host_name);
    println!("操作系统: {}", system_info.host.os_name);
    println!("系统版本: {}", system_info.host.os_version);
    println!("系统类型: {}", system_info.host.os_type);
    println!("启动时间: {}", system_info.host.boot_time);
}

#[cfg(feature = "cpu")]
#[test]
fn test_cpu_info() {
    let system_info = get_system_info();

    assert!(!system_info.cpu.cpu_model.is_empty());
    assert!(system_info.cpu.cpu_cores > 0);

    println!("CPU型号: {}", system_info.cpu.cpu_model);
    println!("CPU核心数: {}", system_info.cpu.cpu_cores);

    if let Some(frequency) = system_info.cpu.cpu_frequency {
        println!("CPU频率: {} GHz", frequency);
    }

    if let Some(usage) = system_info.cpu.cpu_usage {
        println!("CPU使用率: {}%", usage);
    }
}

#[cfg(feature = "process")]
#[test]
fn test_process_info() {
    let system_info = get_system_info();

    assert!(system_info.process.pid.as_u32() > 0);
    assert!(!system_info.process.name.is_empty());

    println!("进程ID: {}", system_info.process.pid);
    println!("进程名称: {}", system_info.process.name);

    if let Some(cpu_usage) = system_info.process.cpu_usage {
        println!("进程CPU使用率: {}%", cpu_usage);
    }

    if let Some(memory_usage) = system_info.process.memory_usage {
        println!("进程内存使用率: {}%", memory_usage);
    }

    println!("进程已用内存: {:.2} MB", system_info.process.used_memory);
}

#[cfg(feature = "memory")]
#[test]
fn test_memory_info() {
    let system_info = get_system_info();

    assert!(system_info.memory.total_memory > 0.0);
    assert!(system_info.memory.used_memory >= 0.0);
    assert!(system_info.memory.free_memory >= 0.0);

    println!("总内存: {} MB", system_info.memory.total_memory);
    println!("已用内存: {} MB", system_info.memory.used_memory);
    println!("可用内存: {} MB", system_info.memory.free_memory);

    if let Some(usage) = system_info.memory.memory_usage {
        println!("内存使用率: {}%", usage);
    }
}

#[cfg(feature = "disk")]
#[test]
fn test_disk_info() {
    let system_info = get_system_info();

    assert!(system_info.disk.total_disk_space > 0.0);
    assert!(system_info.disk.total_used_space >= 0.0);
    assert!(system_info.disk.total_free_space >= 0.0);

    println!("总磁盘空间: {} GB", system_info.disk.total_disk_space);
    println!("已用磁盘空间: {} GB", system_info.disk.total_used_space);
    println!("可用磁盘空间: {} GB", system_info.disk.total_free_space);
    println!("磁盘使用率: {}%", system_info.disk.total_usage);

    for disk in &system_info.disk.disks {
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
    let system_info = get_system_info();

    // GPU信息可能不存在（例如在没有GPU的系统上）
    if let Some(gpu) = system_info.gpu {
        println!("GPU型号: {}", gpu.gpu_model);
        println!("GPU已用内存: {} MB", gpu.gpu_memory_used);
        println!("GPU总内存: {} MB", gpu.gpu_memory_total);
        println!("GPU可用内存: {} MB", gpu.gpu_memory_free);
        println!("GPU使用率: {}%", gpu.gpu_usage);
    } else {
        println!("未检测到GPU");
    }
}
