use std::fs::File;
use std::io::{Error as IoError, Read};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("I/O 错误: {0}")]
    Io(#[from] IoError),

    #[error("文件不是有效的 GGUF LoRA 文件")]
    InvalidGguf,

    #[error("版本不兼容: 要求 3.0+")]
    VersionIncompatible,

    #[error("文件损坏或格式错误")]
    CorruptedFile,
}

pub struct LoraMetadata {
    pub tensor_count: usize,
    pub version: f32,
    pub estimated_vram: usize,
}

pub struct LoraLoader;

impl LoraLoader {
    pub fn validate(path: &Path) -> Result<LoraMetadata, ModelError> {
        let mut file = File::open(path)?;
        let mut header = [0u8; 1024];
        let bytes_read = file.read(&mut header)?;

        if bytes_read < 16 {
            return Err(ModelError::CorruptedFile);
        }

        // 校验 GGUF 魔数 0x46554747
        let magic = u32::from_le_bytes([header[0], header[1], header[2], header[3]]);
        if magic != 0x46554747 {
            return Err(ModelError::InvalidGguf);
        }

        // 校验版本号
        let version = f32::from_le_bytes([header[4], header[5], header[6], header[7]]);
        if version < 3.0 {
            return Err(ModelError::VersionIncompatible);
        }

        // 估算张量数量（粗略检查）
        let tensor_count =
            u32::from_le_bytes([header[8], header[9], header[10], header[11]]) as usize;

        // 获取文件大小
        let file_size = file.metadata()?.len() as usize;

        // 估算 VRAM
        let estimated_vram = Self::estimate_vram(file_size);

        Ok(LoraMetadata {
            tensor_count,
            version,
            estimated_vram,
        })
    }

    pub fn estimate_vram(file_size: usize) -> usize {
        (file_size as f64 * 1.2) as usize
    }
}
