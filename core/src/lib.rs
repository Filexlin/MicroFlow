//! MicroFlow - Core模块
//! 
//! 架构版本: v3.4
//! 冻结日期: 2026-02-14

pub mod types;
pub mod engine;
pub mod parameter;
pub mod vram;
pub mod ffi;
pub mod workflow;
pub mod model;

pub use types::{DataType, DataValue, ModelId, Error};
pub use engine::*;
pub use parameter::*;
pub use vram::*;
pub use ffi::*;
pub use workflow::*;
pub use model::*;

// TODO: Week 1 implementation