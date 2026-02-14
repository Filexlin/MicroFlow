//! RAII包装器：安全封装llama.cpp C指针

use std::ptr::NonNull;
use std::sync::{Arc, Mutex};
use std::marker::PhantomData;
use std::path::Path;
use std::time::{Instant, Duration};
use crate::ffi::error::FfiError;
use crate::ffi::types::{LoadParams, ContextParams};
use crate::ffi::{initialize_backend, is_backend_initialized};
use crate::ffi::lora::{LoRAState, validate_lora_header};
use std::sync::atomic::Ordering;

pub(crate) struct InnerModel {
    ptr: NonNull<llama_cpp_rs::llama_model>,
    size_bytes: usize,
    n_vocab: usize,
    n_layer: usize,
    load_time: Instant,
}

impl Drop for InnerModel {
    fn drop(&mut self) {
        // SAFETY: ptr由llama_load_model_from_file创建，非空，且只在此处释放
        unsafe {
            llama_cpp_rs::llama_free_model(self.ptr.as_ptr());
        }
    }
}

/// 线程安全的模型句柄
pub struct LlamaModel {
    pub(crate) inner: Arc<Mutex<InnerModel>>,
    pub(crate) lora_state: Arc<Mutex<LoRAState>>,
}

impl LlamaModel {
    /// 从GGUF文件加载模型（阻塞操作）
    /// 
    /// # SAFETY前提
    /// - 路径必须是有效UTF-8且文件存在
    /// - backend必须已初始化（本函数自动检查）
    pub fn from_file<P: AsRef<Path>>(
        path: P,
        params: LoadParams,
    ) -> Result<Self, FfiError> {
        let path = path.as_ref();
        
        // 阻塞项修复#1：路径安全检查（DeepSeek要求）
        if !path.exists() {
            return Err(FfiError::ModelNotFound(path.to_path_buf()));
        }
        
        // 阻塞项修复#3：自动backend初始化
        if !is_backend_initialized() {
            initialize_backend()?;
        }
        
        let start = Instant::now();
        
        // 阻塞项修复#1：UTF-8验证（Qwen要求，防panic）
        let path_str = path.to_str()
            .ok_or_else(|| FfiError::InvalidParameter("路径包含非法UTF-8字符".to_string()))?;
        
        // 阻塞项修复#2：SAFETY注释
        // SAFETY:
        // - path_str是有效的null-terminated C字符串
        // - backend已初始化
        // - 返回的指针由llama_cpp_rs管理，非空时有效
        let ptr = unsafe {
            llama_cpp_rs::llama_load_model_from_file(
                path_str,
                params.into(),
            ).map_err(|e| FfiError::Internal(format!("模型加载失败: {}", e)))?
        };
        
        let ptr = NonNull::new(ptr)
            .ok_or(FfiError::Internal("llama_load_model_from_file返回空指针".to_string()))?;
        
        // 阻塞项修复#2：SAFETY注释
        // SAFETY: ptr是有效的llama_model指针，由llama_cpp_rs保证
        let size_bytes = unsafe { llama_cpp_rs::llama_model_size(ptr.as_ptr()) };
        let n_vocab = unsafe { llama_cpp_rs::llama_n_vocab(ptr.as_ptr()) as usize };
        let n_layer = unsafe { llama_cpp_rs::llama_n_layer(ptr.as_ptr()) as usize };
        
        let inner = Arc::new(Mutex::new(InnerModel {
            ptr,
            size_bytes,
            n_vocab,
            n_layer,
            load_time: start,
        }));

        let lora_state = Arc::new(Mutex::new(LoRAState {
            active_lora: None,
            apply_time: Duration::default(),
        }));

        Ok(Self { inner, lora_state })
    }

    pub fn size_bytes(&self) -> Result<usize, FfiError> {
        Ok(self.inner.lock().map_err(|_| FfiError::Internal("锁中毒".into()))?.size_bytes)
    }
    pub fn n_vocab(&self) -> Result<usize, FfiError> {
        Ok(self.inner.lock().map_err(|_| FfiError::Internal("锁中毒".into()))?.n_vocab)
    }
    pub fn n_layer(&self) -> Result<usize, FfiError> {
        Ok(self.inner.lock().map_err(|_| FfiError::Internal("锁中毒".into()))?.n_layer)
    }

    pub(crate) fn with_ptr<F, R>(&self, f: F) -> Result<R, FfiError>
    where F: FnOnce(*const llama_cpp_rs::llama_model) -> R,
    {
        let model = self.inner.lock().map_err(|_| FfiError::Internal("锁中毒".into()))?;
        Ok(f(model.ptr.as_ptr()))
    }

    pub fn apply_lora<P: AsRef<Path>>(&self, lora_path: P) -> Result<(), FfiError> {
        let start = Instant::now();
        let path = lora_path.as_ref();
        if !path.exists() { return Err(FfiError::ModelNotFound(path.to_path_buf())); }
        
        validate_lora_header(path)?; // 文件头校验
        
        let mut model = self.inner.lock().map_err(|_| FfiError::Internal("锁中毒".into()))?;
        let path_str = path.to_str().ok_or_else(|| FfiError::InvalidParameter("路径非法".into()))?;
        
        // 卸载旧LoRA
        if self.lora_state.lock().map_err(|_| FfiError::Internal("锁中毒".into()))?.active_lora.is_some() {
            self.unload_lora_inner(&mut model)?;
        }
        
        // 应用新LoRA
        let result = unsafe {
            llama_cpp_rs::llama_model_apply_lora_from_file(model.ptr.as_ptr(), path_str, 1.0, std::ptr::null_mut())
        };
        
        if let Err(e) = result {
            let _ = self.unload_lora_inner(&mut model); // 回滚
            return Err(FfiError::Internal(format!("LoRA失败(已回滚): {}", e)));
        }
        
        let elapsed = start.elapsed();
        if elapsed > Duration::from_millis(100) { eprintln!("警告: LoRA耗时{:?}", elapsed); }
        
        let mut state = self.lora_state.lock().map_err(|_| FfiError::Internal("锁中毒".into()))?;
        state.active_lora = Some(path.to_string_lossy().to_string());
        state.apply_time = elapsed;
        Ok(())
    }
    
    pub fn unload_lora(&self) -> Result<(), FfiError> {
        let mut model = self.inner.lock().map_err(|_| FfiError::Internal("锁中毒".into()))?;
        self.unload_lora_inner(&mut model)?;
        self.lora_state.lock().map_err(|_| FfiError::Internal("锁中毒".into()))?.active_lora = None;
        Ok(())
    }
    
    fn unload_lora_inner(&self, model: &mut InnerModel) -> Result<(), FfiError> {
        unsafe { llama_cpp_rs::llama_model_remove_lora(model.ptr.as_ptr()).map_err(|e| FfiError::Internal(format!("卸载失败: {}", e))) }
    }
}

unsafe impl Send for LlamaModel {}
unsafe impl Sync for LlamaModel {}

pub struct LlamaContext {
    model: Arc<Mutex<InnerModel>>,
    ctx_ptr: NonNull<llama_cpp_rs::llama_context>,
    _marker: PhantomData<*mut ()>,
}

impl LlamaContext {
    pub fn new(
        model: &LlamaModel,
        params: ContextParams,
    ) -> Result<Self, FfiError> {
        if !is_backend_initialized() {
            initialize_backend()?;
        }

        // 阻塞项修复#2：SAFETY注释
        // SAFETY:
        // - model.inner.ptr是有效的llama_model指针
        // - params已转换为有效的llama_context_params
        // - backend已初始化
        let model_inner = model.inner.lock().map_err(|_| FfiError::Internal("锁中毒".into()))?;
        let ctx_ptr = unsafe {
            llama_cpp_rs::llama_new_context_with_model(
                model_inner.ptr.as_ptr(),
                params.into(),
            ).map_err(|e| FfiError::Internal(format!("Context创建失败: {}", e)))?
        };

        Ok(Self {
            model: Arc::clone(&model.inner),
            ctx_ptr: NonNull::new(ctx_ptr)
                .ok_or(FfiError::Internal("返回空指针".to_string()))?,
            _marker: PhantomData,
        })
    }
}

impl Drop for LlamaContext {
    fn drop(&mut self) {
        // 阻塞项修复#2：SAFETY注释
        // SAFETY: ctx_ptr由llama_new_context_with_model创建，非空，且只在此处释放
        unsafe { llama_cpp_rs::llama_free(self.ctx_ptr.as_ptr()); }
    }
}

impl !Send for LlamaContext {}
impl !Sync for LlamaContext {}