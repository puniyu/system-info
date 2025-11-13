use puniyu_system_info::SystemInfo;

#[cfg(feature = "host")]
#[test]
fn test_host_info() {
	let host_info = SystemInfo::host();

	assert!(!host_info.host_name.is_empty());
	assert!(!host_info.os_name.is_empty());
	assert!(!host_info.os_version.is_empty());
	assert!(!host_info.os_type.is_empty());
	assert!(host_info.boot_time.timestamp() > 0);
}

#[cfg(feature = "cpu")]
#[test]
fn test_cpu_info() {
	let cpu_info = SystemInfo::cpu();

	assert!(!cpu_info.model_name.is_empty());
	assert!(cpu_info.physical_cores > 0);
	assert!(cpu_info.logical_cores > 0);

	assert!(cpu_info.frequency >= 0.0);

	if let Some(usage) = cpu_info.usage {
		assert!(usage <= 100.0);
	}
}

#[cfg(feature = "process")]
#[test]
fn test_process_info() {
	let process_info = SystemInfo::process();

	assert!(process_info.pid.as_u32() > 0);
	assert!(!process_info.name.is_empty());

	std::thread::sleep(std::time::Duration::from_secs(1));

	let process_info = SystemInfo::process();
	assert!(process_info.start_time > 0);
	assert!(process_info.run_time > 0);

	assert!(process_info.used_memory >= 0.0);

	if let Some(cpu_usage) = process_info.cpu_usage {
		assert!(cpu_usage <= 100.0);
	}

	if let Some(memory_usage) = process_info.memory_usage {
		assert!(memory_usage <= 100.0);
	}
}

#[cfg(feature = "memory")]
#[test]
fn test_memory_info() {
	let memory_info = SystemInfo::memory();

	assert!(memory_info.total > 0);
	assert!(memory_info.used > 0);
	assert!(memory_info.free > 0);
	assert!(memory_info.usage >= 0.0);
}

#[cfg(feature = "disk")]
#[test]
fn test_disk_info() {
	let disk_info = SystemInfo::disk();

	assert!(disk_info.total_space > 0);
	assert!(disk_info.total_used_space > 0);
	assert!(disk_info.total_free_space > 0);
	assert!(disk_info.read_speed >= 0.0);
	assert!(disk_info.write_speed >= 0.0);

	for disk in &disk_info.disks {
		assert!(!disk.name.is_empty());
		assert!(disk.total_space > 0);
		assert!(disk.used_space > 0);
		assert!(disk.free_space > 0);
		assert!(disk.usage <= 100f32);
	}
}

#[cfg(feature = "gpu")]
#[test]
fn test_gpu_info() {
	let gpu_info = SystemInfo::gpu();

	if let Some(gpu) = gpu_info {
		assert!(!gpu.model.is_empty());
		assert!(gpu.memory_total > 0.0);
		assert!(gpu.memory_used >= 0.0);
		assert!(gpu.memory_free >= 0.0);
		assert!(gpu.usage <= 100);
	} else {
		dbg!("未检测到GPU");
	}
}
