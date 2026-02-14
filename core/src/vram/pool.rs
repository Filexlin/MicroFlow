use std::sync::Arc;
use std::time::{Instant};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::ffi::{LlamaModel, FfiError, LoadParams};

// 槽位结构
pub struct Slot {
    pub model_id: String,
    pub model: Arc<LlamaModel>,
    pub last_access: Instant,
}

// VRAM池结构
pub struct VramPool {
    capacity: usize, // = 2 (MVP)
    slots: HashMap<String, Slot>,
    lru: Vec<String>,
}

impl VramPool {
    /// 创建新的VRAM池
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            slots: HashMap::new(),
            lru: Vec::new(),
        }
    }

    /// 加载模型
    pub fn load_model(&mut self, id: String, path: PathBuf, params: LoadParams)
        -> Result<Arc<LlamaModel>, FfiError>
    {
        // 如果缓存已满，淘汰最久未使用的模型
        if self.slots.len() >= self.capacity {
            self.evict_lru()?;
        }

        // 加载新模型
        let model = Arc::new(LlamaModel::from_file(&path, params)?);

        // 插入到槽位中
        self.slots.insert(id.clone(), Slot {
            model_id: id.clone(),
            model: Arc::clone(&model),
            last_access: Instant::now(),
        });

        // 更新LRU列表
        self.lru.push(id);

        Ok(model)
    }

    /// 获取模型
    pub fn get_model(&mut self, id: &str) -> Option<Arc<LlamaModel>> {
        self.slots.get_mut(id).map(|s| {
            // 更新最后访问时间
            s.last_access = Instant::now();
            Arc::clone(&s.model)
        })
    }

    /// 淘汰最久未使用的模型
    pub fn evict_lru(&mut self) -> Result<(), FfiError> {
        if let Some(oldest_id) = self.lru.first().cloned() {
            // 从LRU列表中移除
            self.lru.remove(0);
            // 从槽位中移除
            self.slots.remove(&oldest_id);
        }
        Ok(())
    }

    /// 获取当前槽位使用情况
    pub fn get_slot_status(&self) -> Vec<String> {
        self.slots.keys().cloned().collect()
    }

    /// 获取当前容量
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// 获取当前使用量
    pub fn usage(&self) -> usize {
        self.slots.len()
    }
}
