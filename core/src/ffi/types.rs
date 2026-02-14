// 加载参数结构体
#[derive(Debug, Clone, Default)]
pub struct LoadParams {
    pub n_gpu_layers: i32,
    pub main_gpu: i32,
    pub use_mmap: bool,
    pub use_mlock: bool,
}

// 上下文参数结构体
#[derive(Debug, Clone, Default)]
pub struct ContextParams {
    pub n_ctx: usize,
    pub n_batch: usize,
    pub n_threads: usize,
}

// 从LoadParams转换为llama_cpp_rs::LlamaModelParams
impl From<LoadParams> for llama_cpp_rs::LlamaModelParams {
    fn from(params: LoadParams) -> Self {
        let mut model_params = Self::default();
        model_params.set_n_gpu_layers(params.n_gpu_layers);
        model_params.set_main_gpu(params.main_gpu);
        model_params.set_use_mmap(params.use_mmap);
        model_params.set_use_mlock(params.use_mlock);
        model_params
    }
}

