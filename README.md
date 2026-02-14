# MicroFlow

MicroFlow是一个基于Rust和Tauri的现代化工作流引擎，旨在提供高性能、可扩展的数据流处理能力。

## 技术栈

- **Rust**: 1.75.0
- **Tauri**: 2.0-beta
- **llama-cpp-rs**: 0.3

## 项目结构

```
microflow/
├── .github/            # GitHub配置文件
├── docs/               # 文档
├── abi/                # 应用程序二进制接口
├── core/               # 核心功能
├── nodes/              # 节点定义
├── python_runtime/     # Python运行时
├── distributed/        # 分布式功能（预留）
├── desktop/            # 桌面应用
├── cli/                # 命令行工具
├── tests/              # 测试
├── benchmarks/         # 基准测试
├── scripts/            # 脚本
└── Cargo.toml          # 工作区配置
```

## 快速开始

### 环境要求

- Rust 1.75.0
- Cargo
- Node.js (用于Tauri)

### 安装

```bash
# 克隆仓库
git clone <repository-url>
cd microflow

# 安装依赖
cargo build
```

### 运行测试

```bash
cargo test
```

## 开发流程

1. **Week 1**: 核心架构实现
2. **Week 2**: 节点系统和Python运行时
3. **Week 3**: 桌面应用和CLI
4. **Week 4**: 测试和优化

## 许可证

本项目采用MIT和Apache 2.0双许可证。
