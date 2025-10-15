# Changelog

## [0.5.8](https://github.com/puniyu/system-info/compare/v0.5.7...v0.5.8) (2025-10-15)


### 🔧 其他更新

* **system:** 修改系统和进程运行时间单位为秒 ([5653a5e](https://github.com/puniyu/system-info/commit/5653a5ee74b692a57077ad7728b6176fd1c9c8cb))

## [0.5.7](https://github.com/puniyu/system-info/compare/v0.5.6...v0.5.7) (2025-10-14)


### ♻️ 代码重构

* **lib:** 修改系统和进程时间字段类型为u64 ([74078ec](https://github.com/puniyu/system-info/commit/74078ec1d84353fd9aab846e062b171f3f2247a1))

## [0.5.6](https://github.com/puniyu/system-info/compare/v0.5.5...v0.5.6) (2025-10-13)


### 🔧 其他更新

* **deps:** update puniyu system_info packages ([78fe851](https://github.com/puniyu/system-info/commit/78fe851fccec30a2d7b12df382a849bc1c910497))
* **release:** 更新发布工作流配置 ([b021745](https://github.com/puniyu/system-info/commit/b0217457e01b14cc6dc37202e8598642f071cf18))


### ♻️ 代码重构

* **system:** 重构系统信息获取逻辑 ([6c57aaa](https://github.com/puniyu/system-info/commit/6c57aaabb9662f2020cb9d3e97ab1f73282a5cc3))

## [0.5.5](https://github.com/Puniyu/system-info/compare/v0.5.4...v0.5.5) (2025-09-24)


### ♻️ 代码重构

* **process:** 优化运行时间计算逻辑 ([d3218eb](https://github.com/Puniyu/system-info/commit/d3218ebbedded369095b509a32018515c2263281))

## [0.5.4](https://github.com/Puniyu/system-info/compare/v0.5.3...v0.5.4) (2025-09-24)


### 🔧 其他更新

* **config:** 添加 Rust 项目配置文件 ([a7d5c5d](https://github.com/Puniyu/system-info/commit/a7d5c5df90dd3120b023009d4875842cd8098daf))

## [0.5.3](https://github.com/Puniyu/system-info/compare/v0.5.2...v0.5.3) (2025-09-23)


### 🐛 错误修复

* **system:** 为所有信息结构体添加 Clone 派生 ([e39a09c](https://github.com/Puniyu/system-info/commit/e39a09c85b438dd76859f6cc9042238e4f7f5bcd))

## [0.5.2](https://github.com/Puniyu/system_info/compare/v0.5.1...v0.5.2) (2025-08-15)


### 🔧 其他更新

* **arch:** 添加系统架构信息 ([6c18183](https://github.com/Puniyu/system_info/commit/6c18183c360fbc65b02df141d3f1a7fe67d7dfc2))

## [0.5.1](https://github.com/Puniyu/system_info/compare/v0.5.0...v0.5.1) (2025-08-15)


### 🔧 其他更新

* **uptime:** 添加系统运行时间字段 ([107e8c2](https://github.com/Puniyu/system_info/commit/107e8c25781dd4561fa0182ad8497b0f3ec395d9))

## [0.5.0](https://github.com/Puniyu/system_info/compare/v0.4.0...v0.5.0) (2025-08-15)


### ✨ 新功能

* **memory:** 增加交换内存信息监控 ([a288fba](https://github.com/Puniyu/system_info/commit/a288fba8423d17df2f9dc00a087a959e9643b339))

## [0.4.0](https://github.com/Puniyu/system_info/compare/v0.3.1...v0.4.0) (2025-08-12)


### ✨ 新功能

* **network:** 添加网络信息获取功能 ([4a6a14f](https://github.com/Puniyu/system_info/commit/4a6a14ff8ec61238a6f5e9a97afe4459e1b53550))


### 🎡 持续集成

* 优化 GitHub Actions 工作流并修复代码 ([b867939](https://github.com/Puniyu/system_info/commit/b8679399454c57c6021f86fe144d1f86a4cfde44))

## [0.3.1](https://github.com/Puniyu/system_info/compare/v0.3.0...v0.3.1) (2025-08-12)


### 🔧 其他更新

* **process:** 添加进程启动时间和运行时间字段 ([ed197a2](https://github.com/Puniyu/system_info/commit/ed197a25c24117d491eb8ce054b1760f1f0561ab))

## [0.3.0](https://github.com/Puniyu/system_info/compare/v0.2.1...v0.3.0) (2025-08-12)


### ✨ 新功能

* **process:** 添加进程信息功能 ([ca61305](https://github.com/Puniyu/system_info/commit/ca6130544341e1ad1110d8d4bc3e7c565f5d2f3e))


### 📦️ 构建系统

* **Cargo.toml:** 使 gfxinfo 依赖项可选并更新相关功能 ([44c8f5a](https://github.com/Puniyu/system_info/commit/44c8f5a02b761bb37e2aead36e6f3634933f7c9e))

## [0.2.1](https://github.com/Puniyu/system_info/compare/v0.2.0...v0.2.1) (2025-08-12)


### ♻️ 代码重构

* 为 GpuInfo 结构体添加 cfg 属性 ([e7885b1](https://github.com/Puniyu/system_info/commit/e7885b1e87cc402c84d28e93bd5196345e844b6a))


### 🎡 持续集成

* 移除获取用户ID的步骤 ([e5e1831](https://github.com/Puniyu/system_info/commit/e5e1831565c4b276fb04987ee972a3bc7d05418c))

## [0.2.0](https://github.com/Puniyu/system_info/compare/v0.1.0...v0.2.0) (2025-08-12)


### ✨ 新功能

* 添加系统信息获取功能 ([2fcad40](https://github.com/Puniyu/system_info/commit/2fcad40b77b098601c62784985c0670758990c84))
