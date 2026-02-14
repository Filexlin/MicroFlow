use crate::types::DataType;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Port {
    pub id: String,
    pub data_type: DataType,
    pub multiple: bool,
}

pub struct DynamicPorts {
    inputs: HashMap<String, Port>,
    outputs: HashMap<String, Port>,
}

impl DynamicPorts {
    pub fn new() -> Self { Self { inputs: HashMap::new(), outputs: HashMap::new() } }
    pub fn add_input(&mut self, port: Port) { self.inputs.insert(port.id.clone(), port); }
    pub fn add_output(&mut self, port: Port) { self.outputs.insert(port.id.clone(), port); }
    pub fn get_input(&self, id: &str) -> Option<&Port> { self.inputs.get(id) }
    pub fn get_output(&self, id: &str) -> Option<&Port> { self.outputs.get(id) }
}