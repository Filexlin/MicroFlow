//! FFI类型定义

/// 模型加载参数
#[derive(Debug, Clone, Copy)]
pub struct LoadParams {
    pub n_gpu_layers: i32,
    pub main_gpu: i32,
    pub use_mmap: bool,
    pub use_mlock: bool,
}

impl Default for LoadParams {
    fn default() -> Self {
        Self {
            n_gpu_layers: 0,
            main_gpu: 0,
            use_mmap: true,
            use_mlock: false,
        }
    }
}

/// 推理上下文参数
#[derive(Debug, Clone, Copy)]
pub struct ContextParams {
    pub n_ctx: u32,
    pub n_batch: u32,
    pub n_threads: u32,
}

impl Default for ContextParams {
    fn default() -> Self {
        Self {
            n_ctx: 4096,
            n_batch: 512,
            n_threads: 4,
        }
    }
}
