use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use crate::ffi::error::FfiError;

pub struct LoRAState {
    pub active_lora: Option<String>,
    pub apply_time: Duration,
}

pub fn validate_lora_header(path: &Path) -> Result<(), FfiError> {
    let mut file = File::open(path).map_err(|e| FfiError::InvalidParameter(format!("无法打开: {}", e)))?;
    let mut header = [0u8; 8];
    file.read_exact(&mut header).map_err(|_| FfiError::InvalidGguf("文件损坏".into()))?;
    let is_gguf = &header[0..4] == b"GGUF";
    let is_safe = &header[0..8] == b"__PK\x03\x04";
    if !is_gguf && !is_safe && !path.extension().map_or(false, |e| e == "bin") {
        return Err(FfiError::InvalidGguf("不支持格式".into()));
    }
    Ok(())
}

pub fn estimate_lora_vram(path: &Path) -> Result<usize, FfiError> {
    let meta = std::fs::metadata(path).map_err(|_| FfiError::ModelNotFound(path.to_path_buf()))?;
    Ok((meta.len() as f64 * 1.2) as usize)
}