pub mod lora_loader;
pub use lora_loader::{LoraLoader, ModelError, LoraMetadata};

pub trait ModelProvider: Send + Sync {
    /// 热切换 LoRA（0.1 秒目标）
    fn apply_lora(&self, lora_path: &std::path::Path) -> Result<(), ModelError>;
    
    /// 卸载当前 LoRA，恢复 Base
    fn remove_lora(&self) -> Result<(), ModelError>;
    
    /// 获取当前 LoRA 状态
    fn current_lora(&self) -> Option<std::path::PathBuf>;
}
