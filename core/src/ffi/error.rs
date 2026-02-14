use std::path::PathBuf;
use thiserror::Error;

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
    
    #[error("无效参数: {0}")]
    InvalidParameter(String),
}

// 从llama_cpp_rs::Error转换
impl From<llama_cpp_rs::Error> for FfiError {
    fn from(err: llama_cpp_rs::Error) -> Self {
        use llama_cpp_rs::ErrorKind;
        match err.kind() {
            ErrorKind::FileNotFound => Self::ModelNotFound(err.path().into()),
            ErrorKind::InvalidFormat => Self::InvalidGguf(err.to_string()),
            ErrorKind::GpuError => Self::GpuInitFailed {
                reason: err.to_string(),
                #[cfg(debug_assertions)]
                raw_code: err.raw_code(),
            },
            _ => Self::Internal(err.to_string()),
        }
    }
}
