//! FFI层：安全的llama.cpp Rust绑定
//! 
//! # 安全保证
//! - 所有unsafe代码限制在此模块内
//! - LlamaModel线程安全，LlamaContext绑定单线程
//! - RAII确保C资源正确释放

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Once;

pub mod error;
pub mod types;
pub mod wrapper;
pub mod lora;

pub use error::FfiError;
pub use types::{LoadParams, ContextParams};
pub use wrapper::{LlamaModel, LlamaContext};
pub use lora::{LoRAState, validate_lora_header, estimate_lora_vram};

pub(crate) static LLAMA_BACKEND_INITIALIZED: AtomicBool = AtomicBool::new(false);
pub(crate) static INIT_ONCE: Once = Once::new();

pub fn initialize_backend() -> Result<(), FfiError> {
    INIT_ONCE.call_once(|| unsafe {
        llama_cpp_rs::llama_backend_init(false);
        LLAMA_BACKEND_INITIALIZED.store(true, Ordering::SeqCst);
    });
    Ok(())
}

pub fn is_backend_initialized() -> bool {
    LLAMA_BACKEND_INITIALIZED.load(Ordering::SeqCst)
}
