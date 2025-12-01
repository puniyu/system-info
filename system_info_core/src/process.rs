use std::time::{SystemTime, UNIX_EPOCH};
use sysinfo::{Pid, Process, System};

use crate::round;

#[derive(Debug, Clone)]
pub struct ProcessInfo {
	/// 进程ID
	pub pid: Pid,
	/// 进程名称
	pub name: String,
	/// 子进程信息
	pub sub_list: Option<Vec<ProcessInfo>>,
	/// 进程启动时间
	pub start_time: u64,
	/// 进程运行时间，单位：秒
	pub run_time: u64,
	/// 进程CPU使用率
	pub cpu_usage: Option<f32>,
	/// 进程内存使用率
	pub memory_usage: Option<f32>,
	/// 进程已用内存(单位: MB)
	pub used_memory: f64,
}

impl Default for ProcessInfo {
	fn default() -> Self {
		Self::new(std::process::id())
	}
}

impl ProcessInfo {
	pub fn new(pid: u32) -> Self {
		use sysinfo::ProcessesToUpdate;
		let mut system = System::new();
		let pid = Pid::from_u32(pid);
		system.refresh_processes(ProcessesToUpdate::All, true);
		system.refresh_memory();

		let total_memory = system.total_memory();

		system
			.process(pid)
			.map_or_else(|| Self::unknown(pid), |p| Self::from_process(p, &system, total_memory))
	}

	pub fn all() -> Vec<ProcessInfo> {
		use std::collections::HashSet;
		use sysinfo::ProcessesToUpdate;

		let mut system = System::new();
		system.refresh_processes(ProcessesToUpdate::All, true);
		system.refresh_memory();
		let total_memory = system.total_memory();

		let all_pids: HashSet<Pid> = system.processes().keys().copied().collect();

		all_pids
			.iter()
			.filter_map(|&pid| {
				let process = system.process(pid)?;
				if process.parent().is_some_and(|parent_pid| all_pids.contains(&parent_pid)) {
					return None;
				}
				Some(Self::from_process(process, &system, total_memory))
			})
			.collect()
	}

	fn unknown(pid: Pid) -> Self {
		Self {
			pid,
			name: "Unknown".to_string(),
			sub_list: None,
			start_time: 0,
			run_time: 0,
			cpu_usage: None,
			memory_usage: None,
			used_memory: 0.0,
		}
	}

	fn from_process(process: &Process, system: &System, total_memory: u64) -> Self {
		let pid = process.pid();
		let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

		Self {
			pid,
			name: process.name().to_string_lossy().to_string(),
			sub_list: Self::build_children(system, pid, total_memory),
			start_time: process.start_time(),
			run_time: current_time.saturating_sub(process.start_time()),
			cpu_usage: Self::calc_cpu_usage(process),
			memory_usage: Self::calc_memory_usage(process, total_memory),
			used_memory: process.memory() as f64 / 1024.0 / 1024.0,
		}
	}

	fn build_children(system: &System, parent_pid: Pid, total_memory: u64) -> Option<Vec<Self>> {
		let children: Vec<Self> = system
			.processes()
			.values()
			.filter(|p| p.parent() == Some(parent_pid))
			.map(|p| Self::from_process(p, system, total_memory))
			.collect();

		if children.is_empty() { None } else { Some(children) }
	}

	fn calc_cpu_usage(process: &Process) -> Option<f32> {
		let usage = process.cpu_usage();
		if usage > 0.0 { Some(round(usage as f64) as f32) } else { None }
	}

	fn calc_memory_usage(process: &Process, total_memory: u64) -> Option<f32> {
		if total_memory > 0 {
			Some(round(process.memory() as f64 / total_memory as f64 * 100.0) as f32)
		} else {
			None
		}
	}
}
