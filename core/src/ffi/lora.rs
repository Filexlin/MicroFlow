//! LoRA 状态管理

use std::path::Path;
use std::time::Duration;

/// LoRA应用状态
#[derive(Debug, Default)]
pub struct LoRAState {
    pub active_lora: Option<String>,
    pub apply_time: Duration,
}

/// 验证LoRA文件头（简单版）
pub fn validate_lora_header(path: &Path) -> Result<(), crate::ffi::FfiError> {
    // 简单检查文件存在且非空
    if !path.exists() {
        return Err(crate::ffi::FfiError::ModelNotFound(path.to_path_buf()));
    }
    // TODO: 添加真正的GGUF LoRA文件头校验
    Ok(())
}

/// 估算LoRA显存使用量（简化版）
pub fn estimate_lora_vram(_path: &Path) -> Result<usize, crate::ffi::FfiError> {
    // 简单估算：假设每个LoRA层使用10MB
    Ok(10 * 1024 * 1024) // 10MB
}
