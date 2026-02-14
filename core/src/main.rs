use microflow_core::workflow::{ExecutionContext, WorkflowExecutor};

fn main() {
    println!("MicroFlow MVP - Sprint 2");
    let ctx = ExecutionContext::new();
    let executor = WorkflowExecutor::new(ctx);
    executor.run_simple_workflow("你好，世界!", "test_model");
}
