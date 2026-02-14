use crate::vram::VramPool;
use std::sync::{Arc, Mutex};

pub struct ExecutionContext {
    pub vram_pool: Arc<Mutex<VramPool>>,
}

impl ExecutionContext {
    pub fn new(pool: VramPool) -> Self {
        Self { vram_pool: Arc::new(Mutex::new(pool)) }
    }
}