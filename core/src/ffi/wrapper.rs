//! RAII包装器：安全封装llama.cpp C指针

use std::ptr::NonNull;
use std::sync::Arc;
use std::marker::PhantomData;
use std::path::Path;
use crate::ffi::error::FfiError;
use crate::ffi::types::{LoadParams, ContextParams};
use crate::ffi::LLAMA_BACKEND_INITIALIZED;
use std::sync::atomic::Ordering;

pub(crate) struct InnerModel {
    ptr: NonNull<std::ffi::c_void>,
    size_bytes: usize,
    n_vocab: usize,
}

impl Drop for InnerModel {
    fn drop(&mut self) {
        // SAFETY: ptr由加载函数创建，且只在此处释放
        // Week 6: unsafe { llama_cpp_rs::llama_free_model(self.ptr.as_ptr()) };
    }
}

pub struct LlamaModel {
    pub(crate) inner: Arc<InnerModel>,
}

impl LlamaModel {
    pub fn from_file<P: AsRef<Path>>(
        path: P,
        _params: LoadParams,
    ) -> Result<Self, FfiError> {
        let path = path.as_ref();
        
        if !LLAMA_BACKEND_INITIALIZED.load(Ordering::SeqCst) {
            return Err(FfiError::BackendNotInitialized);
        }
        
        if !path.exists() {
            return Err(FfiError::ModelNotFound(path.to_path_buf()));
        }

        let inner = Arc::new(InnerModel {
            ptr: NonNull::dangling(),
            size_bytes: 0,
            n_vocab: 32000,
        });

        Ok(Self { inner })
    }

    pub fn size_bytes(&self) -> usize {
        self.inner.size_bytes
    }

    pub fn n_vocab(&self) -> usize {
        self.inner.n_vocab
    }

    pub(crate) fn with_ptr<F, R>(&self, f: F) -> R
    where
        F: FnOnce(*const std::ffi::c_void) -> R,
    {
        f(self.inner.ptr.as_ptr())
    }
}

unsafe impl Send for LlamaModel {}
unsafe impl Sync for LlamaModel {}

pub struct LlamaContext {
    model: Arc<InnerModel>,
    ctx_ptr: NonNull<std::ffi::c_void>,
    _marker: PhantomData<*mut ()>,
}

impl LlamaContext {
    pub fn new(
        model: &LlamaModel,
        _params: ContextParams,
    ) -> Result<Self, FfiError> {
        if !LLAMA_BACKEND_INITIALIZED.load(Ordering::SeqCst) {
            return Err(FfiError::BackendNotInitialized);
        }

        Ok(Self {
            model: Arc::clone(&model.inner),
            ctx_ptr: NonNull::dangling(),
            _marker: PhantomData,
        })
    }
}

impl Drop for LlamaContext {
    fn drop(&mut self) {
        // SAFETY: 先释放Context，Model由Arc自动管理
        // Week 6: unsafe { llama_cpp_rs::llama_free(self.ctx_ptr.as_ptr()) };
    }
}

impl !Send for LlamaContext {}
impl !Sync for LlamaContext {}
