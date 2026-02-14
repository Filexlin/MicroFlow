use std::sync::Arc;
use std::ptr::NonNull;
use std::marker::PhantomData;
use std::path::Path;
use std::ffi::c_void;

use crate::ffi::{FfiError, LoadParams, ContextParams};

// 内部模型结构体（包含原始指针）
pub struct InnerModel {
    ptr: NonNull<c_void>,
    size_bytes: usize,
    n_vocab: usize,
}

impl Drop for InnerModel {
    fn drop(&mut self) {
        // SAFETY: 这个unsafe块是安全的，因为：
        // 1. ptr是一个有效的NonNull<c_void>指针
        // 2. 我们在Drop中调用llama_cpp_rs::llama_free_model来释放内存
        // 3. 这个指针在创建时已经被验证为有效
        unsafe {
            llama_cpp_rs::llama_free_model(self.ptr.as_ptr());
        }
    }
}

// 外部模型结构体（安全RAII包装器）
pub struct LlamaModel {
    inner: Arc<InnerModel>,
}

// 实现Send和Sync，因为InnerModel的Drop是线程安全的
unsafe impl Send for LlamaModel {}
unsafe impl Sync for LlamaModel {}

impl LlamaModel {
    /// 从文件加载模型
    pub fn from_file<P: AsRef<Path>>(
        path: P,
        params: LoadParams,
    ) -> Result<Self, FfiError> {
        let path = path.as_ref();
        
        // 检查路径是否存在
        if !path.exists() {
            return Err(FfiError::ModelNotFound(path.to_path_buf()));
        }

        // 暂时使用todo!()占位，实际实现需要调用llama_cpp_rs的API
        // 注意：实际实现时需要检查backend初始化
        todo!("实现从文件加载模型的逻辑")
    }

    /// 获取模型大小
    pub fn size_bytes(&self) -> usize {
        self.inner.size_bytes
    }

    /// 获取词表大小
    pub fn n_vocab(&self) -> usize {
        self.inner.n_vocab
    }

    /// 使用模型指针执行闭包
    pub(crate) fn with_ptr<F, R>(&self, f: F) -> R
    where
        F: FnOnce(*const c_void) -> R,
    {
        f(self.inner.ptr.as_ptr())
    }
}

// 上下文结构体
pub struct LlamaContext {
    model: Arc<InnerModel>,
    ctx_ptr: NonNull<c_void>,
    _marker: PhantomData<*mut ()>,
}

// 实现Drop，先释放上下文，模型由Arc自动管理
impl Drop for LlamaContext {
    fn drop(&mut self) {
        // SAFETY: 这个unsafe块是安全的，因为：
        // 1. ctx_ptr是一个有效的NonNull<c_void>指针
        // 2. 我们在Drop中调用llama_cpp_rs::llama_free来释放上下文
        // 3. 这个指针在创建时已经被验证为有效
        unsafe {
            llama_cpp_rs::llama_free(self.ctx_ptr.as_ptr());
        }
    }
}

// 标记为!Send和!Sync，因为LlamaContext不是线程安全的
impl !Send for LlamaContext {}
impl !Sync for LlamaContext {}

// 全局backend初始化标志
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;

static LLAMA_BACKEND_INITIALIZED: AtomicBool = AtomicBool::new(false);

