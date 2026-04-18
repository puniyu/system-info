# Changelog

## [1.5.6](https://github.com/puniyu/system-info/compare/core-v1.5.5...core-v1.5.6) (2026-04-18)


### 🔧 其他更新

* **deps:** update rust crate sysinfo to 0.38.0 ([c6a788c](https://github.com/puniyu/system-info/commit/c6a788cdbb4b4de501e0be455f0ad2c2ceab66af))
* **deps:** update rust crates ([32a038b](https://github.com/puniyu/system-info/commit/32a038bf524d1fa21b3707f137963610e935d6bb))

## [1.5.5](https://github.com/puniyu/system-info/compare/core-v1.5.4...core-v1.5.5) (2026-01-24)


### 🔧 其他更新

* **deps:** 修正依赖项 ([0381f7e](https://github.com/puniyu/system-info/commit/0381f7e06526a7bd40188857dcfd1f54302b0dce))

## [1.5.4](https://github.com/puniyu/system-info/compare/core-v1.5.3...core-v1.5.4) (2025-12-21)


### 🔧 其他更新

* **system-info:** 添加时区信息并更新系统启动时间说明 ([449c711](https://github.com/puniyu/system-info/commit/449c7119460767151944c97b5f109cb7b1b21f10))

## [1.5.3](https://github.com/puniyu/system-info/compare/core-v1.5.2...core-v1.5.3) (2025-12-02)


### 🐛 错误修复

* **network:** 修复未能获取全部网卡速率信息 ([3fd274e](https://github.com/puniyu/system-info/commit/3fd274e0ab2be7d5ce97b5c7fb92d1554ef68123))

## [1.5.2](https://github.com/puniyu/system-info/compare/core-v1.5.1...core-v1.5.2) (2025-12-02)


### ♻️ 代码重构

* **gpu:** 重构GPU信息获取逻辑以提高跨平台兼容性 ([fd5a954](https://github.com/puniyu/system-info/commit/fd5a954215a49caa2e2ffa9c32f245f0cf80c975))

## [1.5.1](https://github.com/puniyu/system-info/compare/core-v1.5.0...core-v1.5.1) (2025-12-01)


### 🐛 错误修复

* **gpu:** 添加GPU模块的条件编译支持 ([8504ac2](https://github.com/puniyu/system-info/commit/8504ac26c86f2cdcb398733e5432f8d8964f3ed3))

## [1.5.0](https://github.com/puniyu/system-info/compare/core-v1.4.0...core-v1.5.0) (2025-12-01)


### ✨ 新功能

* **gpu:** 改进 macOS GPU 信息获取逻辑 ([1c27ba7](https://github.com/puniyu/system-info/commit/1c27ba783501bf1bf5586b6b4327003ef7b501c2))

## [1.4.0](https://github.com/puniyu/system-info/compare/core-v1.3.0...core-v1.4.0) (2025-12-01)


### ✨ 新功能

* **system_info_core:** 重构系统信息获取模块 ([cb16e7a](https://github.com/puniyu/system-info/commit/cb16e7a7a7dec27b824c4d605388dc40ec55379d))

## [1.3.0](https://github.com/puniyu/system-info/compare/core-v0.8.3...core-v1.3.0) (2025-12-01)


### ✨ 新功能

* **system_info:** 增强网络和进程信息功能 ([ea00cff](https://github.com/puniyu/system-info/commit/ea00cff2fce129f048d316f6a66aa1b284fdca8d))


### 🐛 错误修复

* **network:** 修正网络上传下载数据计算错误 ([9f9f7a8](https://github.com/puniyu/system-info/commit/9f9f7a8f306cdda416b5089897df24deb9b1f7d5))

## [0.8.3](https://github.com/puniyu/system-info/compare/core-v0.8.2...core-v0.8.3) (2025-11-14)


### 🐛 错误修复

* **system_info_core:** 限制GPU信息获取仅支持Windows系统 ([ceeb138](https://github.com/puniyu/system-info/commit/ceeb138a253b74ae28da56bdf0bb4fc651266d7e))

## [0.8.2](https://github.com/puniyu/system-info/compare/core-v0.8.1...core-v0.8.2) (2025-11-13)


### 🐛 错误修复

* **system_info_core:** 修复磁盘使用率计算精度问题 ([39fd5ac](https://github.com/puniyu/system-info/commit/39fd5aca68721a0ceedc137a7173a2a1453b36c7))

## [0.8.1](https://github.com/puniyu/system-info/compare/core-v0.8.0...core-v0.8.1) (2025-11-13)


### ♻️ 代码重构

* **cpu:** 移除对 num_cpus crate 的依赖 ([71cd214](https://github.com/puniyu/system-info/commit/71cd21451db6dcc3cdad30644d858841fecc78d6))
* **cpu:** 重构CPU信息获取逻辑以提高准确性 ([f5f08a9](https://github.com/puniyu/system-info/commit/f5f08a9eefcb2d90d4cce88f7723579e33f2d686))
* **system_info:** 重构系统信息数据结构和实现 ([3b67b05](https://github.com/puniyu/system-info/commit/3b67b05563f94d945d760e5c226cf6571eb25925))

## [0.8.0](https://github.com/puniyu/system-info/compare/core-v0.7.0...core-v0.8.0) (2025-11-13)


### ✨ 新功能

* **system_info:** 添加磁盘挂载点信息 ([e06ddc2](https://github.com/puniyu/system-info/commit/e06ddc22d1ed97d4776f5490ac270b53082b5fe1))

## [0.7.0](https://github.com/puniyu/system-info/compare/core-v0.6.1...core-v0.7.0) (2025-11-10)


### ✨ 新功能

* **system_info_node:** 初始化Node.js绑定模块 ([8596b43](https://github.com/puniyu/system-info/commit/8596b4321e80c7d86a406a46b0e305493aa65400))
