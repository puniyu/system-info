use chrono::{DateTime, TimeZone, Utc};

#[derive(Debug, Clone)]
pub struct HostInfo {
	/// 主机名
	pub host_name: String,
	/// 操作系统名
	pub os_name: String,
	/// 操作系统版本
	pub os_version: String,
	/// 操作系统类型
	pub os_type: String,
	/// 系统架构
	pub arch: String,
	/// 时区
	pub time_zone: String,
	/// 系统启动时间(UTC时间)
	pub boot_time: DateTime<Utc>,
	/// 系统运行时间， 单位：秒
	pub uptime: u64,
}

impl Default for HostInfo {
	fn default() -> Self {
		use std::env;
		use sysinfo::System;
		let hostname = System::host_name().unwrap();
		let os_name = System::name().unwrap();
		let arch = System::cpu_arch();
		let os_version = System::os_version().unwrap();
		let os_type = env::consts::OS.to_string();
		let boot_time_secs = System::boot_time();
		let boot_time = Utc
			.timestamp_opt(boot_time_secs as i64, 0)
			.single()
			.expect("Invalid boot time timestamp");
		let tz_str = iana_time_zone::get_timezone().expect("Invalid timezone");
		let uptime = System::uptime();
		Self {
			host_name: hostname,
			os_name,
			arch,
			os_version,
			os_type,
			boot_time,
			uptime,
			time_zone: tz_str,
		}
	}
}

impl HostInfo {
	pub fn new() -> Self {
		Self::default()
	}
}
