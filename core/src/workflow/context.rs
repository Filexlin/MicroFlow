use crate::types::DataValue;
use crate::vram::VramPool;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct ExecutionContext {
    pub vram_pool: Arc<Mutex<VramPool>>,
    outputs: HashMap<String, HashMap<String, DataValue>>,
}

impl Default for ExecutionContext {
    fn default() -> Self {
        Self::new()
    }
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self {
            vram_pool: Arc::new(Mutex::new(VramPool::new(2))),
            outputs: HashMap::new(),
        }
    }

    #[cfg(feature = "llama")]
    pub fn get_model(&self, model_id: &str) -> Option<Arc<crate::ffi::LlamaModel>> {
        let mut pool = self.vram_pool.lock().unwrap();
        pool.get_model(model_id)
    }

    #[cfg(not(feature = "llama"))]
    pub fn get_model(&self, _model_id: &str) -> Option<Arc<()>> {
        None
    }

    pub fn set_outputs(&mut self, node_id: String, outputs: HashMap<String, DataValue>) {
        self.outputs.insert(node_id, outputs);
    }

    pub fn set_node_outputs(&mut self, node_id: String, outputs: HashMap<String, DataValue>) {
        self.set_outputs(node_id, outputs);
    }

    pub fn get_outputs(&self, node_id: &str) -> Option<&HashMap<String, DataValue>> {
        self.outputs.get(node_id)
    }

    pub fn get_node_outputs(&self, node_id: &str) -> Option<&HashMap<String, DataValue>> {
        self.get_outputs(node_id)
    }

    pub fn get_final_outputs(&self) -> HashMap<String, DataValue> {
        // 简化版：返回所有输出
        let mut final_outputs = HashMap::new();
        for outputs in self.outputs.values() {
            for (key, value) in outputs {
                final_outputs.insert(key.clone(), value.clone());
            }
        }
        final_outputs
    }
}
