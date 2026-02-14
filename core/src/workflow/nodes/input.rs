use crate::parameter::{DynamicPorts, Port};
use crate::types::DataType;

pub struct TextInputNode {
    pub text: String,
    pub ports: DynamicPorts,
}

impl TextInputNode {
    pub fn new(text: &str) -> Self {
        let mut ports = DynamicPorts::new();
        ports.add_output(Port {
            id: "text".to_string(),
            data_type: DataType::Text,
            multiple: false,
        });
        Self { text: text.to_string(), ports }
    }
    pub fn execute(&self) -> String { self.text.clone() }
}