use std::path::PathBuf;
use std::collections::HashMap;
use tokio::sync::mpsc::Sender;
use serde::{Deserialize, Serialize};

use crate::types::data_type::DataType;

// 模型ID类型
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct ModelId(pub String);

// 错误类型
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Error {
    TypeMismatch(String),
    ConversionError(String),
    StreamError(String),
    PathError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TypeMismatch(msg) => write!(f, "Type mismatch: {}", msg),
            Error::ConversionError(msg) => write!(f, "Conversion error: {}", msg),
            Error::StreamError(msg) => write!(f, "Stream error: {}", msg),
            Error::PathError(msg) => write!(f, "Path error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

// DataValue枚举
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum DataValue {
    Number(f64),
    Text(String),
    Boolean(bool),
    Path(PathBuf),
    List(Vec<DataValue>),
    Dict(HashMap<String, DataValue>),
    Model(ModelId),
    Stream(Sender<DataValue>), // 流式通道
}

impl DataValue {
    pub fn data_type(&self) -> DataType {
        match self {
            DataValue::Number(_) => DataType::Number,
            DataValue::Text(_) => DataType::Text,
            DataValue::Boolean(_) => DataType::Boolean,
            DataValue::Path(_) => DataType::Path,
            DataValue::List(items) => {
                if let Some(first) = items.first() {
                    DataType::List(Box::new(first.data_type()))
                } else {
                    // 空列表，使用Number作为默认类型
                    DataType::List(Box::new(DataType::Number))
                }
            }
            DataValue::Dict(_, _) => DataType::Dict("key".to_string(), Box::new(DataType::Number)),
            DataValue::Model(_) => DataType::Model,
            DataValue::Stream(_) => DataType::Stream(Box::new(DataType::Number)),
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            DataValue::Number(n) => Some(*n),
            DataValue::Boolean(b) => Some(if *b { 1.0 } else { 0.0 }),
            DataValue::Text(s) => s.parse().ok(),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<&str> {
        match self {
            DataValue::Text(s) => Some(s),
            _ => None,
        }
    }

    pub fn convert_to(&self, target: DataType) -> Result<DataValue, Error> {
        match (self, &target) {
            (DataValue::Number(n), DataType::Number) => Ok(DataValue::Number(*n)),
            (DataValue::Number(n), DataType::Text) => Ok(DataValue::Text(n.to_string())),
            (DataValue::Number(n), DataType::Boolean) => Ok(DataValue::Boolean(*n != 0.0)),
            (DataValue::Text(s), DataType::Text) => Ok(DataValue::Text(s.clone())),
            (DataValue::Text(s), DataType::Number) => {
                s.parse().map(DataValue::Number).map_err(|_| Error::ConversionError(format!("Cannot convert '{}' to number", s)))
            }
            (DataValue::Text(s), DataType::Boolean) => {
                match s.to_lowercase().as_str() {
                    "true" | "yes" | "1" => Ok(DataValue::Boolean(true)),
                    "false" | "no" | "0" => Ok(DataValue::Boolean(false)),
                    _ => Err(Error::ConversionError(format!("Cannot convert '{}' to boolean", s))),
                }
            }
            (DataValue::Boolean(b), DataType::Boolean) => Ok(DataValue::Boolean(*b)),
            (DataValue::Boolean(b), DataType::Number) => Ok(DataValue::Number(if *b { 1.0 } else { 0.0 })),
            (DataValue::Boolean(b), DataType::Text) => Ok(DataValue::Text(b.to_string())),
            (DataValue::Path(p), DataType::Path) => Ok(DataValue::Path(p.clone())),
            (DataValue::Path(p), DataType::Text) => Ok(DataValue::Text(p.to_string_lossy().to_string())),
            _ => Err(Error::TypeMismatch(format!("Cannot convert {:?} to {:?}", self, target))),
        }
    }
}
