use std::net::IpAddr;

use sysinfo::{MacAddr, NetworkData, Networks};

use crate::round;

#[derive(Debug, Clone)]
pub struct IpInfo {
	/// ip地址
	pub ip_address: IpAddr,
	/// 子网掩码
	pub netmask: u8,
}

#[derive(Debug, Clone)]
pub struct NetworkInfo {
	/// 网卡名称
	pub name: String,
	/// 网卡ip信息
	pub ip_info: Vec<IpInfo>,
	/// 上传速度(单位: KB/S)
	pub upload: f64,
	/// 下载速度(单位: KB/S)
	pub download: f64,
	/// 总上传流量(单位: MB)
	pub total_upload: f64,
	/// 总下载流量(单位: MB)
	pub total_download: f64,
	/// 网卡mac地址
	pub mac_addr: MacAddr,
}

impl Default for NetworkInfo {
	fn default() -> Self {
		use std::thread::sleep;
		use std::time::Duration;

		let mut networks = Networks::new_with_refreshed_list();
		sleep(Duration::from_millis(100));
		networks.refresh(true);
		if let Some(info) = Self::find_active_network(&networks, true) {
			return info;
		}
		if let Some(info) = Self::find_active_network(&networks, false) {
			return info;
		}
		Self::unknown()
	}
}

impl NetworkInfo {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn all() -> Vec<Self> {
		Networks::new_with_refreshed_list()
			.list()
			.iter()
			.map(|(name, data)| Self::from_data(name, data, 0.0, 0.0))
			.collect()
	}

	fn find_active_network(networks: &Networks, require_traffic: bool) -> Option<Self> {
		for (name, data) in networks.list() {
			if Self::is_loopback(name) {
				continue;
			}

			let ip_info = Self::parse_ip_info(data);
			let has_ipv4 = ip_info.iter().any(|ip| ip.ip_address.is_ipv4());

			if !has_ipv4 || ip_info.is_empty() {
				continue;
			}

			let has_traffic = data.received() + data.transmitted() > 0;
			if require_traffic && !has_traffic {
				continue;
			}

			let (upload, download) = if has_traffic {
				(
					round(data.transmitted() as f64 / 1024.0 / 0.1),
					round(data.received() as f64 / 1024.0 / 0.1),
				)
			} else {
				(0.0, 0.0)
			};

			return Some(Self {
				name: name.to_string(),
				ip_info,
				upload,
				download,
				total_upload: Self::bytes_to_mb(data.total_transmitted()),
				total_download: Self::bytes_to_mb(data.total_received()),
				mac_addr: data.mac_address(),
			});
		}
		None
	}

	fn from_data(name: &str, data: &NetworkData, upload: f64, download: f64) -> Self {
		Self {
			name: name.to_string(),
			ip_info: Self::parse_ip_info(data),
			upload,
			download,
			total_upload: Self::bytes_to_mb(data.total_transmitted()),
			total_download: Self::bytes_to_mb(data.total_received()),
			mac_addr: data.mac_address(),
		}
	}

	fn unknown() -> Self {
		Self {
			name: "Unknown".to_string(),
			ip_info: Vec::new(),
			upload: 0.0,
			download: 0.0,
			total_upload: 0.0,
			total_download: 0.0,
			mac_addr: MacAddr([0u8; 6]),
		}
	}

	fn parse_ip_info(data: &NetworkData) -> Vec<IpInfo> {
		data.ip_networks()
			.iter()
			.map(|ip| IpInfo { ip_address: ip.addr, netmask: ip.prefix })
			.collect()
	}

	fn is_loopback(name: &str) -> bool {
		name.starts_with("lo") || name.starts_with("Loopback") || name.contains("loopback")
	}

	fn bytes_to_mb(bytes: u64) -> f64 {
		round(bytes as f64 / 1024.0 / 1024.0)
	}
}
