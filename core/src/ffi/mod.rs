//! FFI层：安全的llama.cpp Rust绑定
//! 
//! # 安全保证
//! - 所有unsafe代码限制在此模块内
//! - LlamaModel线程安全，LlamaContext绑定单线程
//! - RAII确保C资源正确释放

use std::sync::Once;

pub mod error;
pub mod types;
pub mod wrapper;
pub mod lora;

pub use error::FfiError;
pub use types::{LoadParams, ContextParams};
pub use wrapper::{LlamaModel, LlamaContext};
pub use lora::{LoRAState, validate_lora_header, estimate_lora_vram};

static BACKEND_INIT: Once = Once::new();
static mut BACKEND_INIT_SUCCESS: bool = false;

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

pub fn is_backend_initialized() -> bool {
    unsafe { BACKEND_INIT_SUCCESS }
}
