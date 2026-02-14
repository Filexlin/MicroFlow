use std::sync::Arc;
use std::time::{Instant};
use std::collections::HashMap;

use crate::types::{ModelId, Error as MicroFlowError};

// 模型结构（简化版，实际实现可能更复杂）
pub struct Model {
    pub id: ModelId,
    pub name: String,
    pub size: usize, // 模型大小（字节）
    pub version: String,
}

// 槽位结构
pub struct Slot {
    pub model_id: Option<ModelId>,
    pub model: Option<Arc<Model>>,
    pub last_access: Instant,
}

// LRU缓存结构（简化版）
pub struct LruCache<K, V> {
    items: HashMap<K, (V, Instant)>,
    capacity: usize,
}

impl<K: std::hash::Hash + Eq + Clone, V> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            items: HashMap::new(),
            capacity,
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some((value, _)) = self.items.get_mut(key) {
            // 更新访问时间
            self.items.insert(key.clone(), (value.clone(), Instant::now()));
            Some(value)
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // 如果键已存在，更新值和访问时间
        if self.items.contains_key(&key) {
            self.items.insert(key, (value, Instant::now()));
            None
        } else {
            // 如果缓存已满，淘汰最久未使用的项
            if self.items.len() >= self.capacity {
                self.evict_oldest();
            }
            // 插入新项
            self.items.insert(key, (value, Instant::now()));
            None
        }
    }

    pub fn evict_oldest(&mut self) -> Option<K> {
        // 找到最久未使用的项
        let oldest_key = self.items.iter()
            .min_by_key(|(_, (_, time))| time)
            .map(|(key, _)| key.clone());

        // 淘汰该项
        if let Some(key) = oldest_key {
            self.items.remove(&key);
            Some(key)
        } else {
            None
        }
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.items.contains_key(key)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

// VRAM池结构
pub struct VramPool {
    capacity: usize, // = 2 (MVP)
    slots: Vec<Slot>,
    lru: LruCache<ModelId, Arc<Model>>,
}

impl VramPool {
    /// 创建新的VRAM池
    pub fn new(capacity: usize) -> Self {
        let mut slots = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            slots.push(Slot {
                model_id: None,
                model: None,
                last_access: Instant::now(),
            });
        }

        Self {
            capacity,
            slots,
            lru: LruCache::new(capacity),
        }
    }

    /// 获取模型
    pub fn get(&mut self, model_id: &ModelId) -> Option<Arc<Model>> {
        // 尝试从LRU缓存获取
        if let Some(model) = self.lru.get(model_id) {
            // 更新槽位的最后访问时间
            self.update_slot_access_time(model_id);
            Some(model.clone())
        } else {
            None
        }
    }

    /// 插入模型
    pub fn insert(&mut self, model_id: ModelId, model: Arc<Model>) -> Result<(), MicroFlowError> {
        // 如果模型已存在，更新即可
        if self.lru.contains_key(&model_id) {
            self.lru.insert(model_id, model);
            self.update_slot_access_time(&model_id);
            return Ok(());
        }

        // 如果缓存已满，淘汰最久未使用的模型
        if self.lru.len() >= self.capacity {
            if let Some(evicted_id) = self.evict() {
                // 从槽位中移除淘汰的模型
                self.remove_from_slot(&evicted_id);
            }
        }

        // 插入新模型
        self.lru.insert(model_id.clone(), model.clone());

        // 找到空槽位或最久未使用的槽位
        let slot_index = self.find_empty_slot()
            .unwrap_or_else(|| self.find_oldest_slot());

        // 更新槽位
        self.slots[slot_index] = Slot {
            model_id: Some(model_id),
            model: Some(model),
            last_access: Instant::now(),
        };

        Ok(())
    }

    /// 淘汰最久未使用的模型
    pub fn evict(&mut self) -> Option<ModelId> {
        self.lru.evict_oldest()
    }

    /// 从槽位中移除模型
    fn remove_from_slot(&mut self, model_id: &ModelId) {
        for slot in &mut self.slots {
            if slot.model_id.as_ref() == Some(model_id) {
                slot.model_id = None;
                slot.model = None;
                slot.last_access = Instant::now();
                break;
            }
        }
    }

    /// 更新槽位的最后访问时间
    fn update_slot_access_time(&mut self, model_id: &ModelId) {
        for slot in &mut self.slots {
            if slot.model_id.as_ref() == Some(model_id) {
                slot.last_access = Instant::now();
                break;
            }
        }
    }

    /// 找到空槽位
    fn find_empty_slot(&self) -> Option<usize> {
        self.slots.iter().position(|slot| slot.model_id.is_none())
    }

    /// 找到最久未使用的槽位
    fn find_oldest_slot(&self) -> usize {
        self.slots.iter()
            .enumerate()
            .min_by_key(|(_, slot)| &slot.last_access)
            .unwrap()
            .0
    }

    /// 获取当前槽位使用情况
    pub fn get_slot_status(&self) -> Vec<Option<ModelId>> {
        self.slots.iter().map(|slot| slot.model_id.clone()).collect()
    }

    /// 获取当前容量
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// 获取当前使用量
    pub fn usage(&self) -> usize {
        self.lru.len()
    }
}
