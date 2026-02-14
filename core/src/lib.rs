//! MicroFlow - Core模块
//! 
//! 架构版本: v3.4
//! 冻结日期: 2026-02-14

pub mod types;
pub mod engine;

pub use types::{DataType, DataValue, ModelId, Error};
pub use engine::*;

// TODO: Week 1 implementation