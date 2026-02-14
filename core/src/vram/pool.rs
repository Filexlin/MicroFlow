use std::sync::Arc;
use std::time::{Instant};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::ffi::{LlamaModel, FfiError, LoadParams};
use crate::ffi::lora::estimate_lora_vram;
use crate::model::{LoraLoader, ModelError};

// 槽位结构
pub struct Slot {
    pub model_id: String,
    pub model: Arc<LlamaModel>,
    pub last_access: Instant,
    pub current_lora: Option<String>,
    pub current_lora_size: usize,
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
            current_lora: None,
            current_lora_size: 0,
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
            // FIX: 先尝试卸载LoRA（如果有）
            if let Some(slot) = self.slots.get(&oldest_id) {
                if slot.current_lora.is_some() {
                    if let Err(e) = slot.model.unload_lora() {
                        eprintln!("警告: 淘汰模型{}时卸载LoRA失败: {:?}", oldest_id, e);
                    }
                }
            }
            
            // FIX: 从LRU列表中移除
            self.lru.remove(0);
            
            // FIX: 从槽位中移除（触发Drop，自动释放VRAM）
            if let Some(slot) = self.slots.remove(&oldest_id) {
                // 显式drop模型，确保GPU内存立即释放
                drop(slot.model);
                println!("已淘汰模型，释放VRAM: {}", oldest_id);
            }
            
            Ok(())
        } else {
            Ok(())
        }
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

    pub fn load_lora(&mut self, model_id: &str, lora_path: &Path) -> Result<(), FfiError> {
        let slot = self.slots.get_mut(model_id).ok_or_else(|| FfiError::ModelNotFound(PathBuf::from(model_id)))?;
        
        // 校验 LoRA 文件
        let metadata = LoraLoader::validate(lora_path)
            .map_err(|e| FfiError::Internal(format!("LoRA 验证失败: {:?}", e)))?;
        
        // 检查空闲显存是否足够
        let available = self.available_vram()?;
        if metadata.estimated_vram > available {
            return Err(FfiError::OutOfMemory { 
                requested: metadata.estimated_vram/1024/1024, 
                available: available/1024/1024 
            });
        }
        
        // 加载 LoRA
        slot.model.apply_lora(lora_path)
            .map_err(|e| FfiError::Internal(format!("LoRA 加载失败: {:?}", e)))?;
        
        slot.current_lora = Some(lora_path.to_string_lossy().to_string());
        slot.current_lora_size = metadata.estimated_vram;
        
        Ok(())
    }
    
    pub fn unload_lora(&mut self, model_id: &str) -> Result<(), FfiError> {
        let slot = self.slots.get_mut(model_id).ok_or_else(|| FfiError::ModelNotFound(PathBuf::from(model_id)))?;
        
        // 卸载 LoRA
        slot.model.unload_lora()
            .map_err(|e| FfiError::Internal(format!("LoRA 卸载失败: {:?}", e)))?;
        
        slot.current_lora = None;
        slot.current_lora_size = 0;
        
        Ok(())
    }
    
    pub fn switch_lora(&mut self, model_id: &str, lora_path: PathBuf) -> Result<(), FfiError> {
        // 先尝试卸载当前 LoRA
        if let Err(e) = self.unload_lora(model_id) {
            // 卸载失败不影响切换，继续尝试加载新 LoRA
            eprintln!("卸载旧 LoRA 失败: {:?}", e);
        }
        
        // 加载新 LoRA
        self.load_lora(model_id, &lora_path)
    }
    
    fn available_vram(&self) -> Result<usize, FfiError> {
        let total: usize = 6 * 1024 * 1024 * 1024;
        let used: usize = self.slots.values().map(|s| s.model.size_bytes() + s.current_lora_size).sum::<Result<usize, FfiError>>()?;
        Ok(total.saturating_sub(used))
    }
}
