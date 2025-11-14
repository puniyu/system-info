# Changelog

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
