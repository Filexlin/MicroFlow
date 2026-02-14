use crate::parameter::{DynamicPorts, Port};
use crate::types::DataType;
use crate::workflow::context::ExecutionContext;

pub struct LLMNode {
    pub model_id: String,
    pub ports: DynamicPorts,
}

impl LLMNode {
    pub fn new(model_id: &str) -> Self {
        let mut ports = DynamicPorts::new();
        ports.add_input(Port {
            id: "prompt".to_string(),
            data_type: DataType::Text,
            multiple: false,
        });
        ports.add_output(Port {
            id: "response".to_string(),
            data_type: DataType::Text,
            multiple: false,
        });
        Self {
            model_id: model_id.to_string(),
            ports,
        }
    }
    pub fn execute(&self, prompt: &str, _ctx: &ExecutionContext) -> String {
        format!("[模型{}响应] 你输入了: {}", self.model_id, prompt)
    }
}
