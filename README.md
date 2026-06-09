# system-info

跨平台系统信息采集库，提供 **Rust 核心库** 与 **Node.js 原生绑定** 两种使用方式。可获取主机、CPU、内存、磁盘、网络、进程及 GPU 等运行时信息，适用于系统监控面板、运维工具、桌面应用等场景。

> 本项目 fork 自 [puniyu/system-info](https://github.com/puniyu/system-info)。

## 特性

- **跨平台**：支持 Windows、macOS（Intel / Apple Silicon）、Linux（x86_64 / aarch64）
- **模块化设计**：通过 Cargo Feature 按需启用，减少编译体积
- **统一 API**：Rust 与 Node.js 暴露一致的数据结构
- **进程树**：支持查询指定 PID 的进程及其子进程
- **GPU 检测**：Windows / Linux 优先通过 NVML 获取显存与利用率；macOS 通过 IOKit 读取型号与显存
- **CPU 温度**：在硬件与驱动支持时自动读取（无法读取时返回 `None`）

## 支持平台

| 平台 | Rust 核心库 | Node.js 绑定 |
|------|:-----------:|:------------:|
| Windows x86_64 | ✅ | ✅ |
| macOS x86_64 | ✅ | ✅ |
| macOS Apple Silicon | ✅ | ✅ |
| Linux x86_64 | ✅ | ✅ |
| Linux aarch64 | ✅ | ✅ |

## 项目结构

```
system-info/
├── system_info_core/    # Rust 核心库（crate: puniyu_system_info）
├── system_info_node/    # Node.js N-API 绑定（npm: @puniyu/system-info）
└── .github/workflows/   # CI 构建与发布流程
```

## 安装

### Rust

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu_system_info = "1.5.6"
```

按需启用 Feature（默认已包含 host、cpu、memory、disk、network、process）：

```toml
puniyu_system_info = { version = "1.5.6", features = ["full"] }
```

### Node.js

```bash
npm install @puniyu/system-info
# 或
pnpm add @puniyu/system-info
```

## 快速开始

### Rust

```rust
use puniyu_system_info::SystemInfo;

fn main() {
    // 主机信息
    let host = SystemInfo::host();
    println!("主机名: {}", host.host_name);
    println!("系统: {} {}", host.os_name, host.os_version);
    println!("运行时间: {} 秒", host.uptime);

    // CPU 信息（含使用率与温度）
    let cpu = SystemInfo::cpu();
    println!("CPU: {} ({} 核 / {} 线程)", cpu.model_name, cpu.physical_cores, cpu.logical_cores);
    if let Some(temp) = cpu.temperature {
        println!("温度: {} °C", temp);
    }

    // 内存信息
    let mem = SystemInfo::memory();
    println!("内存: {}/{} MB ({:.1}%)", mem.used, mem.total, mem.usage);

    // 磁盘信息
    let disk = SystemInfo::disk();
    println!("磁盘总用量: {:.1}%", disk.total_usage);
    for d in &disk.disks {
        println!("  {} [{}] {:.1}%", d.name, d.mount, d.usage);
    }

    // 网络信息
    let networks = SystemInfo::network();
    for net in &networks {
        println!("网卡 {}: ↑ {:.1} KB/s  ↓ {:.1} KB/s", net.name, net.upload, net.download);
    }
    let current = SystemInfo::current_network();
    println!("当前网卡: {}", current.name);

    // 进程信息
    let proc = SystemInfo::process();
    println!("当前进程: {} (PID {})", proc.name, proc.pid);
    let all = SystemInfo::process_all();
    println!("顶层进程数: {}", all.len());

    // GPU 信息（需启用 gpu feature）
    if let Some(gpu) = SystemInfo::gpu() {
        println!("GPU: {}", gpu.model);
    }
}
```

### Node.js

```javascript
import {
  getHostInfo,
  getCpuInfo,
  getMemoryInfo,
  getDiskInfo,
  getNetworkInfo,
  getCurrentNetwork,
  getCurrentProcessInfo,
  getProcessInfo,
  getGpuInfo,
} from '@puniyu/system-info'

const host = getHostInfo()
console.log(`${host.hostName} · ${host.osName} ${host.osVersion}`)

const cpu = getCpuInfo()
console.log(`${cpu.modelName} · ${cpu.usage?.toFixed(1)}%`)

const memory = getMemoryInfo()
console.log(`内存 ${memory.used}/${memory.total} MB`)

const disk = getDiskInfo()
console.log(`磁盘使用率 ${disk.totalUsage.toFixed(1)}%`)

const networks = getNetworkInfo()
console.log(`网卡数量: ${networks.length}`)

const process = getCurrentProcessInfo()
console.log(`当前进程: ${process.name} (PID ${process.pid})`)

const gpu = getGpuInfo()
if (gpu) console.log(`GPU: ${gpu.model}`)
```

## API 参考

所有入口均通过 `SystemInfo`（Rust）或对应的 `get*Info` 函数（Node.js）访问。

### 主机信息 — `HostInfo`

| 字段 | 类型 | 说明 |
|------|------|------|
| `host_name` | `String` | 主机名 |
| `os_name` | `String` | 操作系统名称 |
| `os_version` | `String` | 操作系统版本 |
| `os_type` | `String` | 操作系统类型（如 `linux`、`windows`、`macos`） |
| `arch` | `String` | 系统架构 |
| `time_zone` | `String` | IANA 时区 |
| `boot_time` | `DateTime<Utc>` | 系统启动时间（UTC） |
| `uptime` | `u64` | 系统运行时间（秒） |

### CPU 信息 — `CpuInfo`

| 字段 | 类型 | 说明 |
|------|------|------|
| `model_name` | `String` | CPU 型号 |
| `physical_cores` | `u32` | 物理核心数 |
| `logical_cores` | `u32` | 逻辑核心数（线程数） |
| `frequency` | `f32` | 基础频率（GHz） |
| `usage` | `Option<f32>` | 全局 CPU 使用率（%） |
| `temperature` | `Option<f32>` | CPU 温度（°C），不支持时为 `None` |

> CPU 使用率采样需要短暂等待（遵循 `sysinfo` 最小刷新间隔），首次调用会有约 200ms 延迟。

### 内存信息 — `MemoryInfo`

| 字段 | 类型 | 说明 |
|------|------|------|
| `total` | `u64` | 总内存（MB） |
| `used` | `u64` | 已用内存（MB） |
| `free` | `u64` | 可用内存（MB） |
| `usage` | `f32` | 内存使用率（%） |
| `swap_total` | `Option<u64>` | 交换分区总量（MB） |
| `swap_used` | `Option<u64>` | 已用交换分区（MB） |
| `swap_free` | `Option<u64>` | 可用交换分区（MB） |
| `swap_usage` | `Option<f32>` | 交换分区使用率（%） |

### 磁盘信息 — `DiskInfo` / `DiskDetail`

**汇总字段（`DiskInfo`）**

| 字段 | 类型 | 说明 |
|------|------|------|
| `total_space` | `u64` | 总磁盘空间（GB） |
| `total_used_space` | `u64` | 总已用空间（GB） |
| `total_free_space` | `u64` | 总可用空间（GB） |
| `total_usage` | `f64` | 总体使用率（%） |
| `read_speed` | `f32` | 磁盘读取速度（KB/s） |
| `write_speed` | `f32` | 磁盘写入速度（KB/s） |
| `disks` | `Vec<DiskDetail>` | 各分区详情 |

**分区字段（`DiskDetail`）**

| 字段 | 类型 | 说明 |
|------|------|------|
| `name` | `String` | 磁盘名称 |
| `mount` | `String` | 挂载点 |
| `total_space` | `u64` | 总空间（GB） |
| `used_space` | `u64` | 已用空间（GB） |
| `free_space` | `u64` | 可用空间（GB） |
| `usage` | `f32` | 使用率（%） |

### 网络信息 — `NetworkInfo` / `IpInfo`

| 字段 | 类型 | 说明 |
|------|------|------|
| `name` | `String` | 网卡名称 |
| `ip_info` | `Vec<IpInfo>` | IP 地址列表 |
| `upload` | `f64` | 上传速度（KB/s） |
| `download` | `f64` | 下载速度（KB/s） |
| `total_upload` | `f64` | 累计上传流量（MB） |
| `total_download` | `f64` | 累计下载流量（MB） |
| `mac_addr` | `MacAddr` / `String` | MAC 地址 |

**`IpInfo`**

| 字段 | 类型 | 说明 |
|------|------|------|
| `ip_address` | `IpAddr` / `String` | IP 地址 |
| `netmask` | `u8` | 子网前缀长度 |

相关方法：

- `SystemInfo::network()` — 获取所有网卡信息
- `SystemInfo::current_network()` — 获取当前活跃网卡（优先选择有 IPv4 且有流量的网卡）

### 进程信息 — `ProcessInfo`

| 字段 | 类型 | 说明 |
|------|------|------|
| `pid` | `Pid` / `u32` | 进程 ID |
| `name` | `String` | 进程名称 |
| `sub_list` | `Option<Vec<ProcessInfo>>` | 子进程列表 |
| `start_time` | `u64` | 启动时间（Unix 时间戳，秒） |
| `run_time` | `u64` | 运行时长（秒） |
| `cpu_usage` | `Option<f32>` | CPU 使用率（%） |
| `memory_usage` | `Option<f32>` | 内存占用率（%，相对系统总内存） |
| `used_memory` | `f64` | 已用内存（MB） |

相关方法：

- `SystemInfo::process()` — 当前进程
- `SystemInfo::process_with_pid(pid)` — 指定 PID（含子进程树）
- `SystemInfo::process_all()` — 所有顶层进程（不含已被父进程包含的子进程）

### GPU 信息 — `GpuInfo`

| 字段 | 类型 | 说明 |
|------|------|------|
| `model` | `String` | GPU 型号 |
| `memory_total` | `Option<f32>` | 总显存（MB） |
| `memory_used` | `Option<f32>` | 已用显存（MB） |
| `memory_free` | `Option<f32>` | 可用显存（MB） |
| `usage` | `Option<u8>` | GPU 利用率（%） |

检测策略：

| 平台 | 优先级 |
|------|--------|
| Windows | NVML → WMI → DXGI |
| Linux | NVML → lspci |
| macOS | IOKit（型号与显存，无实时利用率） |

## Feature Flags

在 `system_info_core` 中通过 Cargo Feature 控制模块编译：

| Feature | 说明 | 默认 |
|---------|------|:----:|
| `host` | 主机信息 | ✅ |
| `cpu` | CPU 信息 | ✅ |
| `memory` | 内存信息 | ✅ |
| `disk` | 磁盘信息 | ✅ |
| `network` | 网络信息 | ✅ |
| `process` | 进程信息 | ✅ |
| `gpu` | GPU 信息 | ❌ |
| `full` | 包含以上全部（含 gpu） | — |

示例：仅启用 CPU 与内存：

```toml
puniyu_system_info = { version = "1.5.6", default-features = false, features = ["cpu", "memory"] }
```

## 开发

### 环境要求

- Rust ≥ 1.88（项目 toolchain 锁定为 1.96.0，见 `rust-toolchain.toml`）
- Node.js ≥ 14.17
- pnpm ≥ 10（构建 Node 绑定时）

### 构建与测试

```bash
# Rust 核心库 — 运行测试
cargo test -p puniyu_system_info

# 启用 GPU 特性测试
cargo test -p puniyu_system_info --features full

# Node.js 绑定 — 本地构建
cd system_info_node
pnpm install
pnpm build

# 指定目标平台交叉编译
pnpm build --target x86_64-apple-darwin
```

### 代码格式化与 Lint

```bash
cargo fmt --all
cargo clippy --all-targets --all-features
```

## 依赖说明

核心库基于以下项目构建：

- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) — 跨平台系统信息采集
- [nvml-wrapper](https://github.com/Cldfire/nvml-wrapper) — NVIDIA GPU 管理（Windows / Linux）
- [napi-rs](https://napi.rs/) — Node.js 原生模块绑定

## 许可证

[MIT License](./LICENSE)
