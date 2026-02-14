use super::{VramPool, Model, ModelId};
use std::sync::Arc;

#[cfg(test)]
mod tests {
    use super::*;

    // 创建测试模型
    fn create_test_model(id: &str, name: &str) -> (ModelId, Arc<Model>) {
        let model_id = ModelId(id.to_string());
        let model = Arc::new(Model {
            id: model_id.clone(),
            name: name.to_string(),
            size: 1024 * 1024, // 1MB
            version: "1.0.0".to_string(),
        });
        (model_id, model)
    }

    #[test]
    fn test_vram_pool_empty() {
        // 创建容量为2的VRAM池
        let mut pool = VramPool::new(2);

        // 验证初始状态
        assert_eq!(pool.capacity(), 2);
        assert_eq!(pool.usage(), 0);
        assert_eq!(pool.get_slot_status(), vec![None, None]);

        // 从空池中获取模型
        let model_id = ModelId("non-existent".to_string());
        assert!(pool.get(&model_id).is_none());
    }

    #[test]
    fn test_vram_pool_basic_insert() {
        // 创建容量为2的VRAM池
        let mut pool = VramPool::new(2);

        // 插入第一个模型
        let (model_id1, model1) = create_test_model("model1", "Model 1");
        assert!(pool.insert(model_id1.clone(), model1).is_ok());

        // 验证状态
        assert_eq!(pool.usage(), 1);
        let slot_status = pool.get_slot_status();
        assert!(slot_status.contains(&Some(model_id1.clone())));

        // 获取模型
        assert!(pool.get(&model_id1).is_some());
    }

    #[test]
    fn test_vram_pool_full() {
        // 创建容量为2的VRAM池
        let mut pool = VramPool::new(2);

        // 插入第一个模型
        let (model_id1, model1) = create_test_model("model1", "Model 1");
        assert!(pool.insert(model_id1.clone(), model1).is_ok());

        // 插入第二个模型
        let (model_id2, model2) = create_test_model("model2", "Model 2");
        assert!(pool.insert(model_id2.clone(), model2).is_ok());

        // 验证状态
        assert_eq!(pool.usage(), 2);
        let slot_status = pool.get_slot_status();
        assert!(slot_status.contains(&Some(model_id1.clone())));
        assert!(slot_status.contains(&Some(model_id2.clone())));

        // 获取两个模型
        assert!(pool.get(&model_id1).is_some());
        assert!(pool.get(&model_id2).is_some());
    }

    #[test]
    fn test_vram_pool_lru_eviction() {
        // 创建容量为2的VRAM池
        let mut pool = VramPool::new(2);

        // 插入第一个模型
        let (model_id1, model1) = create_test_model("model1", "Model 1");
        assert!(pool.insert(model_id1.clone(), model1).is_ok());

        // 插入第二个模型
        let (model_id2, model2) = create_test_model("model2", "Model 2");
        assert!(pool.insert(model_id2.clone(), model2).is_ok());

        // 访问第一个模型（更新LRU顺序）
        assert!(pool.get(&model_id1).is_some());

        // 插入第三个模型（应该淘汰model2）
        let (model_id3, model3) = create_test_model("model3", "Model 3");
        assert!(pool.insert(model_id3.clone(), model3).is_ok());

        // 验证状态
        assert_eq!(pool.usage(), 2);
        let slot_status = pool.get_slot_status();
        assert!(slot_status.contains(&Some(model_id1.clone())));
        assert!(slot_status.contains(&Some(model_id3.clone())));
        assert!(!slot_status.contains(&Some(model_id2.clone())));

        // 验证model1和model3可以获取，model2不可获取
        assert!(pool.get(&model_id1).is_some());
        assert!(pool.get(&model_id3).is_some());
        assert!(pool.get(&model_id2).is_none());
    }

    #[test]
    fn test_vram_pool_duplicate_insert() {
        // 创建容量为2的VRAM池
        let mut pool = VramPool::new(2);

        // 插入第一个模型
        let (model_id1, model1) = create_test_model("model1", "Model 1");
        assert!(pool.insert(model_id1.clone(), model1).is_ok());

        // 插入第二个模型
        let (model_id2, model2) = create_test_model("model2", "Model 2");
        assert!(pool.insert(model_id2.clone(), model2).is_ok());

        // 重复插入第一个模型
        let (_, model1_new) = create_test_model("model1", "Model 1 Updated");
        assert!(pool.insert(model_id1.clone(), model1_new).is_ok());

        // 验证状态（应该仍然有两个模型，没有淘汰）
        assert_eq!(pool.usage(), 2);
        let slot_status = pool.get_slot_status();
        assert!(slot_status.contains(&Some(model_id1.clone())));
        assert!(slot_status.contains(&Some(model_id2.clone())));

        // 验证两个模型都可以获取
        assert!(pool.get(&model_id1).is_some());
        assert!(pool.get(&model_id2).is_some());
    }

    #[test]
    fn test_vram_pool_evict() {
        // 创建容量为2的VRAM池
        let mut pool = VramPool::new(2);

        // 插入两个模型
        let (model_id1, model1) = create_test_model("model1", "Model 1");
        assert!(pool.insert(model_id1.clone(), model1).is_ok());

        let (model_id2, model2) = create_test_model("model2", "Model 2");
        assert!(pool.insert(model_id2.clone(), model2).is_ok());

        // 手动淘汰最久未使用的模型
        let evicted_id = pool.evict();
        assert!(evicted_id.is_some());

        // 验证状态
        assert_eq!(pool.usage(), 1);
        let slot_status = pool.get_slot_status();
        assert_eq!(slot_status.iter().filter(|&&id| id.is_some()).count(), 1);
    }

    #[test]
    fn test_vram_pool_slot_management() {
        // 创建容量为2的VRAM池
        let mut pool = VramPool::new(2);

        // 插入第一个模型
        let (model_id1, model1) = create_test_model("model1", "Model 1");
        assert!(pool.insert(model_id1.clone(), model1).is_ok());

        // 验证槽位状态
        let slot_status = pool.get_slot_status();
        assert!(slot_status.contains(&Some(model_id1.clone())));

        // 插入第二个模型
        let (model_id2, model2) = create_test_model("model2", "Model 2");
        assert!(pool.insert(model_id2.clone(), model2).is_ok());

        // 验证槽位状态
        let slot_status = pool.get_slot_status();
        assert!(slot_status.contains(&Some(model_id1.clone())));
        assert!(slot_status.contains(&Some(model_id2.clone())));

        // 插入第三个模型（淘汰一个）
        let (model_id3, model3) = create_test_model("model3", "Model 3");
        assert!(pool.insert(model_id3.clone(), model3).is_ok());

        // 验证槽位状态
        let slot_status = pool.get_slot_status();
        assert!(!slot_status.contains(&Some(model_id1.clone()))); // model1被淘汰
        assert!(slot_status.contains(&Some(model_id2.clone())));
        assert!(slot_status.contains(&Some(model_id3.clone())));
    }
}
