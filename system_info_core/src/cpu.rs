#[derive(Debug, Clone)]
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


impl Default for CpuInfo {
	fn default() -> Self {
		use std::thread::sleep;
		use sysinfo::System;
		let mut system = System::new();
		system.refresh_cpu_all();
		
		sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
		system.refresh_cpu_usage();
		let cpu = &system.cpus()[0];
		
		Self {
			model_name: cpu.brand().to_string(),
			physical_cores: num_cpus::get_physical() as u32,
			logical_cores: num_cpus::get() as u32,
			frequency: cpu.frequency() as f32 / 1000.0,
			usage: Some(system.global_cpu_usage().round()),
		}
	}
}


impl CpuInfo {
	pub fn new() -> Self {
		Self::default()
	}
}