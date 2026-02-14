use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum FfiError {
    #[error("模型文件不存在: {0}")]
    ModelNotFound(PathBuf),
    
    #[error("无效的GGUF文件: {0}")]
    InvalidGguf(String),
    
    #[error("GPU初始化失败: {reason}")]
    GpuInitFailed {
        reason: String,
        #[cfg(debug_assertions)]
        raw_code: Option<i32>,
    },
    
    #[error("内存不足: {0}")]
    OutOfMemory(String),
    
    #[error("内部错误: {0}")]
    Internal(String),
}

// 从llama_cpp_rs::Error转换（暂时使用todo!()占位）
impl From<llama_cpp_rs::Error> for FfiError {
    fn from(_: llama_cpp_rs::Error) -> Self {
        todo!("实现llama_cpp_rs::Error到FfiError的转换")
    }
}
