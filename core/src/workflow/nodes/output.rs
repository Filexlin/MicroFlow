use crate::parameter::{DynamicPorts, Port};
use crate::types::DataType;

pub struct TextOutputNode {
    pub ports: DynamicPorts,
    pub result: Option<String>,
}

impl Default for TextOutputNode {
    fn default() -> Self {
        Self::new()
    }
}

impl TextOutputNode {
    pub fn new() -> Self {
        let mut ports = DynamicPorts::new();
        ports.add_input(Port {
            id: "text".to_string(),
            data_type: DataType::Text,
            multiple: false,
        });
        Self {
            ports,
            result: None,
        }
    }
    pub fn execute(&mut self, input: &str) {
        self.result = Some(input.to_string());
        println!("输出结果: {}", input);
    }
}
