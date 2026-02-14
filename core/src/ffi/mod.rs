//! FFI层：安全的llama.cpp Rust绑定
//!
//! # 安全保证
//! - 所有unsafe代码限制在此模块内
//! - LlamaModel线程安全，LlamaContext绑定单线程
//! - RAII确保C资源正确释放

#[cfg(feature = "llama")]
use std::sync::Once;

pub mod error;
pub mod lora;
pub mod types;

#[cfg(feature = "llama")]
pub mod wrapper;

pub use error::FfiError;
pub use lora::{estimate_lora_vram, validate_lora_header, LoRAState};
pub use types::{ContextParams, LoadParams};

#[cfg(feature = "llama")]
pub use wrapper::{LlamaContext, LlamaModel};

#[cfg(feature = "llama")]
static BACKEND_INIT: Once = Once::new();

#[cfg(feature = "llama")]
static mut BACKEND_INIT_SUCCESS: bool = false;

#[cfg(feature = "llama")]
pub fn initialize_backend() -> Result<(), FfiError> {
    unsafe {
        BACKEND_INIT.call_once(|| {
            // 检查 llama_backend_init 返回值（false 表示失败）
            let success = llama_cpp_rs::llama_backend_init(false);
            BACKEND_INIT_SUCCESS = success;
        });

        if BACKEND_INIT_SUCCESS {
            Ok(())
        } else {
            Err(FfiError::BackendInit)
        }
    }
}

#[cfg(not(feature = "llama"))]
pub fn initialize_backend() -> Result<(), FfiError> {
    Err(FfiError::BackendInit)
}

#[cfg(feature = "llama")]
pub fn is_backend_initialized() -> bool {
    unsafe { BACKEND_INIT_SUCCESS }
}

#[cfg(not(feature = "llama"))]
pub fn is_backend_initialized() -> bool {
    false
}
