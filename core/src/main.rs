use microflow_core::vram::VramPool;
use microflow_core::workflow::{ExecutionContext, WorkflowExecutor};

fn main() {
    println!("MicroFlow MVP - Sprint 2");
    let vram_pool = VramPool::new(2);
    let ctx = ExecutionContext::new(vram_pool);
    let executor = WorkflowExecutor::new(ctx);
    executor.run_simple_workflow("你好，世界!", "test_model");
}