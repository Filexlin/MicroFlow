# MicroFlow 项目概述

## 定位
边缘AI时代的ComfyUI - 全场景小模型工作流平台。让6GB显卡本地运行多模型协作，支持文本/图像/音频到硬件控制的全场景。

## 技术栈（纯Rust，零Python依赖）
- Rust 1.75+, Tauri v2, ReactFlow, llama-cpp-rs, tokio, petgraph
- 错误处理: thiserror（禁止unwrap）
- 并发: Mutex单线程策略（单用户本地场景）

## 四层架构
1. 界面层(Tauri+ReactFlow) - Month 2进行中
2. 编排层(DAG执行器+节点系统) - ✅完成
3. 模型层(VramPool+LoRA热切换+.mfl格式) - ✅完成  
4. 引擎层(DataValue+状态机+FFI) - ✅完成

## 已完成功能（Month 1）
- Week1-2: DataValue类型系统(12种类型), 分层状态机(12状态)
- Week3: 动态端口系统, 基础节点(Input/LLM/Output)
- Week4: VramPool(LRU 2槽), 显存精确计算
- Week5-6: FFI层(llama.cpp封装), C ABI, cbindgen
- Week7: 动态端口连接图, DAG执行顺序
- Week8: LoRA热切换(GGUF校验,VRAM估算,失败回滚), LoRASwitchNode
- Week9: JSON-RPC协议, Python进程池, 超时控制

## 目录结构
```
core/src/
  ├── types/          # DataValue/DataType
  ├── engine/         # 状态机
  ├── workflow/nodes/ # 节点实现(LLM/Input/Output/LoRA)
  ├── parameter/      # 动态端口+连接图
  ├── vram/           # VramPool(LRU)
  ├── model/          # LoRA加载器(Week8)
  └── ffi/            # llama.cpp绑定(Week5-6)

python_runtime/src/   # JSON-RPC+进程池(Week9)
  ├── protocol.rs     # JSON-RPC 2.0
  ├── manager.rs      # 进程池管理
  └── executor.rs     # 执行引擎

nodes/                # Tauri前端(Month2)
tauri-app/            # 桌面应用(Week10+)
```

## 关键设计决策
1. **单线程Mutex**: 串行DAG执行,简单安全(MVP阶段)
2. **Python子进程**: 崩溃隔离,突破GIL,50-100ms启动开销(进程池缓解)
3. **显式路径传递**: Python节点通过path_mappings接收路径,禁止隐式文件访问
4. **零Python依赖**: 单文件可执行,Python运行时调用

## Month 2计划(Week10-15)
- Week10: Tauri框架搭建
- Week11: ReactFlow基础画布
- Week12: 自定义节点(UI)
- Week13: 节点连线与保存
- Week14: 运行按钮(后端调用)
- Week15: 工作流持久化

## 快速开始
```bash
cargo check --workspace  # 检查
cargo test --all         # 测试
cd tauri-app && cargo tauri dev  # UI开发
```

## 核心Trait
```rust
trait Node {
    fn execute(&self, inputs: HashMap<PortId, DataValue>) -> Result<HashMap<PortId, DataValue>, NodeError>;
    fn ports(&self) -> DynamicPorts;
}

trait ModelProvider {
    fn load(path: &Path) -> Result<Self, ModelError>;
    fn apply_lora(&self, lora: &Path) -> Result<(), ModelError>;
}
```

## 顾问协作
- DeepSeek: 系统架构(所有权/线程安全)
- Qwen: 工程实现(unsafe/错误处理)
- Trae: 代码执行
- 用户: 整合者

## 状态
Month 1核心引擎功能完成,Month 2UI开发启动。
仓库: `https://github.com/Filexlin/MicroFlow`
