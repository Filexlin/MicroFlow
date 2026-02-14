//! FFI层：安全的llama.cpp Rust绑定
//! 
//! # 安全保证
//! - 所有unsafe代码限制在此模块内
//! - LlamaModel线程安全，LlamaContext绑定单线程
//! - RAII确保C资源正确释放

use std::sync::atomic::{AtomicBool, Ordering};

pub mod error;
pub mod types;
pub mod wrapper;

pub use error::FfiError;
pub use types::{LoadParams, ContextParams};
pub use wrapper::{LlamaModel, LlamaContext};

pub(crate) static LLAMA_BACKEND_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub fn initialize_backend() -> Result<(), FfiError> {
    if LLAMA_BACKEND_INITIALIZED.load(Ordering::SeqCst) {
        return Ok(());
    }
    
    LLAMA_BACKEND_INITIALIZED.store(true, Ordering::SeqCst);
    Ok(())
}

pub(crate) fn is_backend_initialized() -> bool {
    LLAMA_BACKEND_INITIALIZED.load(Ordering::SeqCst)
}
