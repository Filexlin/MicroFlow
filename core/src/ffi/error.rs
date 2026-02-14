use thiserror::Error;
use std::path::PathBuf;

#[derive(Error, Debug, Clone)]
pub enum FfiError {
    #[error("模型文件不存在: {0}")]
    ModelNotFound(PathBuf),
    
    #[error("GGUF格式无效: {0}")]
    InvalidGguf(String),
    
    #[error("GPU初始化失败: {reason}")]
    GpuInitFailed {
        reason: String,
        #[cfg(debug_assertions)]
        raw_code: Option<i32>,
    },
    
    #[error("内存不足: 请求{requested}MB，可用{available}MB")]
    OutOfMemory { requested: usize, available: usize },
    
    #[error("FFI内部错误: {0}")]
    Internal(String),
    
    #[error("参数无效: {0}")]
    InvalidParameter(String),
    
    #[error("Backend未初始化")]
    BackendNotInitialized,
}

impl FfiError {
    pub fn from_llama_error(err: String) -> Self {
        Self::Internal(err)
    }
}
