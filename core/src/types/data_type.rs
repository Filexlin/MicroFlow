use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum DataType {
    Number,      // f64
    Text,        // String
    Boolean,     // bool
    Path,        // PathBuf
    List(Box<DataType>), // 递归列表
    Dict(String, Box<DataType>), // 字典
    Model,       // 模型引用
    Stream(Box<DataType>), // 流式数据
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::Number => write!(f, "Number"),
            DataType::Text => write!(f, "Text"),
            DataType::Boolean => write!(f, "Boolean"),
            DataType::Path => write!(f, "Path"),
            DataType::List(inner) => write!(f, "List({})", inner),
            DataType::Dict(key, inner) => write!(f, "Dict({}, {})", key, inner),
            DataType::Model => write!(f, "Model"),
            DataType::Stream(inner) => write!(f, "Stream({})", inner),
        }
    }
}
