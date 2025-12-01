#[derive(Debug, Clone)]
pub struct DiskInfo {
	/// 总磁盘空间(单位: GB)
	pub total_space: u64,
	/// 总已用磁盘空间(单位: GB)
	pub total_used_space: u64,
	/// 总可用磁盘空间(单位: GB)
	pub total_free_space: u64,
	/// 总体磁盘使用率
	pub total_usage: f64,
	/// 磁盘读速度(单位: KB/S)
	pub read_speed: f32,
	/// 磁盘写入速度(单位: KB/S)
	pub write_speed: f32,
	/// 各个磁盘详细信息
	pub disks: Vec<DiskDetail>,
}


#[derive(Debug, Clone)]
#[cfg(feature = "disk")]
pub struct DiskDetail {
	/// 磁盘名称
	pub name: String,
	/// 磁盘挂载点
	pub mount: String,
	/// 总磁盘空间(单位: GB)
	pub total_space: u64,
	/// 已用磁盘空间(单位: GB)
	pub used_space: u64,
	/// 可用磁盘空间(单位: GB)
	pub free_space: u64,
	/// 磁盘使用率
	pub usage: f32,
}

impl Default for DiskInfo {
	fn default() -> Self {
		use sysinfo::{Disks, ProcessesToUpdate, System};
		let mut s = System::new_all();
		s.refresh_processes(ProcessesToUpdate::All, true);
		
		let mut read_speed = 0f32;
		let mut write_speed = 0f32;
		for process in s.processes() {
			let disk_usage = process.1.disk_usage();
			read_speed += disk_usage.read_bytes as f32 / 1024.0;
			write_speed += disk_usage.written_bytes as f32 / 1024.0;
		}
		
		let disks = Disks::new_with_refreshed_list();
		
		let mut total_disk_space = 0u64;
		let mut total_used_space = 0u64;
		let mut total_free_space = 0u64;
		let mut disk_details = Vec::new();
		
		for disk in disks.list() {
			let total_space = disk.total_space() / (1024 * 1024 * 1024);
			let free_space = disk.available_space() / (1024 * 1024 * 1024);
			let used_space = total_space - free_space;
			
			let usage = if total_space > 0 {
				(used_space as f64 / total_space as f64) * 100.0
			} else {
				0.0
			};
			
			let disk_detail = DiskDetail {
				name: disk.name().to_string_lossy().to_string(),
				mount: disk.mount_point().to_string_lossy().trim_end_matches('\\').to_string(),
				total_space,
				used_space,
				free_space,
				usage: usage.round() as f32,
			};
			
			total_disk_space += total_space;
			total_used_space += used_space;
			total_free_space += free_space;
			disk_details.push(disk_detail);
		}
		
		let total_usage = if total_disk_space > 0 {
			(total_used_space / total_disk_space) as f64 * 100.0
		} else {
			0.0
		};
		
		Self {
			total_space: total_disk_space,
			total_used_space,
			total_free_space,
			total_usage,
			read_speed: read_speed.round(),
			write_speed: write_speed.round(),
			disks: disk_details,
		}
	}
}


impl DiskInfo {
	pub fn new() -> Self {
		Self::default()
	}
}

