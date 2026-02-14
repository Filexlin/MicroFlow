use crate::workflow::nodes::{TextInputNode, LLMNode, TextOutputNode};
use crate::workflow::context::ExecutionContext;

pub struct WorkflowExecutor {
    ctx: ExecutionContext,
}

impl WorkflowExecutor {
    pub fn new(ctx: ExecutionContext) -> Self { Self { ctx } }
    
    pub fn run_simple_workflow(&self, input_text: &str, model_id: &str) {
        println!("开始执行工作流...");
        let input_node = TextInputNode::new(input_text);
        let prompt = input_node.execute();
        println!("输入: {}", prompt);
        
        let llm_node = LLMNode::new(model_id);
        let response = llm_node.execute(&prompt, &self.ctx);
        println!("推理: {}", response);
        
        let mut output_node = TextOutputNode::new();
        output_node.execute(&response);
        
        println!("工作流完成!");
    }
}