#[derive(Debug, Clone)]
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


impl Default for MemoryInfo {
	fn default() -> Self {
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
		
		Self {
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
}

impl MemoryInfo {
	pub fn new() -> Self {
		Self::default()
	}
}