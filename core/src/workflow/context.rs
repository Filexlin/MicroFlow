use crate::vram::VramPool;
use crate::types::DataValue;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub struct ExecutionContext {
    pub vram_pool: Arc<Mutex<VramPool>>,
    outputs: HashMap<String, HashMap<String, DataValue>>,
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self {
            vram_pool: Arc::new(Mutex::new(VramPool::new(2))),
            outputs: HashMap::new(),
        }
    }
    
    pub fn set_outputs(&mut self, node_id: String, outputs: HashMap<String, DataValue>) {
        self.outputs.insert(node_id, outputs);
    }
    
    pub fn get_outputs(&self, node_id: &str) -> Option<&HashMap<String, DataValue>> {
        self.outputs.get(node_id)
    }
    
    pub fn get_final_outputs(&self) -> HashMap<String, DataValue> {
        // 简化版：返回所有输出
        let mut final_outputs = HashMap::new();
        for (_, outputs) in &self.outputs {
            for (key, value) in outputs {
                final_outputs.insert(key.clone(), value.clone());
            }
        }
        final_outputs
    }
}