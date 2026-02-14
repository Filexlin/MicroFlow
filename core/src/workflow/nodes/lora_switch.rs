use crate::ffi::FfiError;
use crate::parameter::{DynamicPorts, Port};
use crate::types::DataType;
use crate::workflow::context::ExecutionContext;
use std::path::PathBuf;

pub struct LoRASwitchNode {
    pub model_id: String,
    pub lora_path: PathBuf,
    pub ports: DynamicPorts,
}

impl LoRASwitchNode {
    pub fn new(model_id: &str, lora_path: &str) -> Self {
        let mut ports = DynamicPorts::new();

        // 输入端口
        ports.add_input(Port {
            id: "model_id".to_string(),
            data_type: DataType::Text,
            multiple: false,
        });

        ports.add_input(Port {
            id: "lora_path".to_string(),
            data_type: DataType::Text,
            multiple: false,
        });

        // 输出端口
        ports.add_output(Port {
            id: "success".to_string(),
            data_type: DataType::Boolean,
            multiple: false,
        });

        ports.add_output(Port {
            id: "error".to_string(),
            data_type: DataType::Text,
            multiple: false,
        });

        Self {
            model_id: model_id.to_string(),
            lora_path: PathBuf::from(lora_path),
            ports,
        }
    }

    pub fn execute(&self, ctx: &ExecutionContext) -> Result<bool, FfiError> {
        let mut vram_pool = ctx
            .vram_pool
            .lock()
            .map_err(|_| FfiError::Internal("锁中毒".into()))?;

        vram_pool.switch_lora(&self.model_id, self.lora_path.clone())?;

        Ok(true)
    }
}
