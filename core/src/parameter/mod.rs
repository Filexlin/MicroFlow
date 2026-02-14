use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::{DataValue, Error as MicroFlowError, DataType};

// UI状态（用于前端）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiState {
    pub visible: bool,
    pub enabled: bool,
    pub value_source: ValueSource,
    pub error_message: Option<String>,
}

// 值来源（用于UI状态）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValueSource {
    UserInput,
    Connection,
    Default,
    Computed,
}

// 参数模式枚举
#[derive(Debug, Clone, PartialEq)]
pub enum ParamMode<T> {
    Constant(T),
    Connection {
        port_id: String,
        cached_value: Option<T>,
    },
}

// 执行上下文（用于解析参数）
pub struct ExecutionContext {
    pub node_id: String,
    pub port_values: HashMap<String, DataValue>,
    pub global_vars: HashMap<String, DataValue>,
    pub execution_time: std::time::Instant,
}

// 参数实例结构
#[derive(Debug, Clone)]
pub struct ParameterInstance {
    pub def_id: String,
    pub mode: ParamMode<DataValue>,
    #[serde(skip)]
    pub ui_state: UiState,
}

impl ParameterInstance {
    /// 创建新的参数实例
    pub fn new(def_id: String, initial_value: DataValue) -> Self {
        Self {
            def_id,
            mode: ParamMode::Constant(initial_value),
            ui_state: UiState {
                visible: true,
                enabled: true,
                value_source: ValueSource::Default,
                error_message: None,
            },
        }
    }

    /// 解析参数值
    pub fn resolve(&self, ctx: &ExecutionContext) -> Result<DataValue, MicroFlowError> {
        match &self.mode {
            ParamMode::Constant(value) => Ok(value.clone()),
            ParamMode::Connection { port_id, cached_value } => {
                // 尝试从上下文获取值
                if let Some(value) = ctx.port_values.get(port_id) {
                    Ok(value.clone())
                } else if let Some(cached) = cached_value {
                    // 使用缓存值
                    Ok(cached.clone())
                } else {
                    Err(MicroFlowError::TypeMismatch(format!("Port '{}' not found in context", port_id)))
                }
            }
        }
    }

    /// 切换模式
    pub fn switch_mode(&mut self, new_mode: ParamMode<DataValue>) {
        self.mode = new_mode;
        
        // 更新UI状态
        self.ui_state.value_source = match &self.mode {
            ParamMode::Constant(_) => ValueSource::UserInput,
            ParamMode::Connection { .. } => ValueSource::Connection,
        };
    }

    /// 从常量模式切换到连接模式
    pub fn switch_to_connection(&mut self, port_id: String) {
        let new_mode = ParamMode::Connection {
            port_id,
            cached_value: None,
        };
        self.switch_mode(new_mode);
    }

    /// 从连接模式切换到常量模式
    pub fn switch_to_constant(&mut self, value: DataValue) {
        let new_mode = ParamMode::Constant(value);
        self.switch_mode(new_mode);
    }

    /// 更新缓存值（用于连接模式）
    pub fn update_cached_value(&mut self, value: DataValue) {
        if let ParamMode::Connection { ref mut cached_value, .. } = self.mode {
            *cached_value = Some(value);
        }
    }

    /// 获取参数类型
    pub fn get_type(&self) -> DataType {
        match &self.mode {
            ParamMode::Constant(value) => value.data_type(),
            ParamMode::Connection { cached_value, .. } => {
                if let Some(value) = cached_value {
                    value.data_type()
                } else {
                    // 默认为Number类型
                    DataType::Number
                }
            }
        }
    }

    /// 检查类型兼容性
    pub fn is_type_compatible(&self, expected_type: &DataType) -> bool {
        let actual_type = self.get_type();
        self.is_types_compatible(&actual_type, expected_type)
    }

    /// 检查两种类型是否兼容
    fn is_types_compatible(&self, actual: &DataType, expected: &DataType) -> bool {
        match (actual, expected) {
            // 相同类型
            (a, b) if a == b => true,
            
            // Number可以与Boolean转换
            (DataType::Number, DataType::Boolean) => true,
            (DataType::Boolean, DataType::Number) => true,
            
            // Number可以与Text转换
            (DataType::Number, DataType::Text) => true,
            (DataType::Text, DataType::Number) => true,
            
            // Boolean可以与Text转换
            (DataType::Boolean, DataType::Text) => true,
            (DataType::Text, DataType::Boolean) => true,
            
            // 列表类型
            (DataType::List(a), DataType::List(b)) => self.is_types_compatible(a, b),
            
            // 流式类型
            (DataType::Stream(a), DataType::Stream(b)) => self.is_types_compatible(a, b),
            
            // 其他类型不兼容
            _ => false,
        }
    }
}
