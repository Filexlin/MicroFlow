use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

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
#[derive(Debug, Clone)]
pub enum DataValue {
    Number(f64),
    Text(String),
    Boolean(bool),
    Path(PathBuf),
    Binary(Vec<u8>),
    List(Vec<DataValue>),
    Dict(HashMap<String, DataValue>),
    Model(ModelId),
}

// 为 DataValue 实现 PartialEq、Eq 和 Hash
impl PartialEq for DataValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DataValue::Number(a), DataValue::Number(b)) => a == b,
            (DataValue::Text(a), DataValue::Text(b)) => a == b,
            (DataValue::Boolean(a), DataValue::Boolean(b)) => a == b,
            (DataValue::Path(a), DataValue::Path(b)) => a == b,
            (DataValue::Binary(a), DataValue::Binary(b)) => a == b,
            (DataValue::List(a), DataValue::List(b)) => a == b,
            (DataValue::Dict(a), DataValue::Dict(b)) => a == b,
            (DataValue::Model(a), DataValue::Model(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for DataValue {}

impl std::hash::Hash for DataValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            DataValue::Number(n) => {
                0.hash(state);
                n.to_bits().hash(state);
            }
            DataValue::Text(s) => {
                1.hash(state);
                s.hash(state);
            }
            DataValue::Boolean(b) => {
                2.hash(state);
                b.hash(state);
            }
            DataValue::Path(p) => {
                3.hash(state);
                p.hash(state);
            }
            DataValue::Binary(b) => {
                4.hash(state);
                b.hash(state);
            }
            DataValue::List(l) => {
                5.hash(state);
                l.hash(state);
            }
            DataValue::Dict(d) => {
                6.hash(state);
                let mut keys: Vec<_> = d.keys().collect();
                keys.sort();
                for key in keys {
                    key.hash(state);
                    d.get(key).hash(state);
                }
            }
            DataValue::Model(m) => {
                7.hash(state);
                m.hash(state);
            }
        }
    }
}

impl std::fmt::Display for DataValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataValue::Number(n) => write!(f, "{}", n),
            DataValue::Text(s) => write!(f, "{}", s),
            DataValue::Boolean(b) => write!(f, "{}", b),
            DataValue::Path(p) => write!(f, "{}", p.to_string_lossy()),
            DataValue::Binary(_) => write!(f, "<binary data>"),
            DataValue::List(items) => write!(f, "{:?}", items),
            DataValue::Dict(d) => write!(f, "{:?}", d),
            DataValue::Model(id) => write!(f, "Model({})", id.0),
        }
    }
}

impl DataValue {
    pub fn data_type(&self) -> DataType {
        match self {
            DataValue::Number(_) => DataType::Number,
            DataValue::Text(_) => DataType::Text,
            DataValue::Boolean(_) => DataType::Boolean,
            DataValue::Path(_) => DataType::Path,
            DataValue::Binary(_) => DataType::Binary,
            DataValue::List(items) => {
                if let Some(first) = items.first() {
                    DataType::List(Box::new(first.data_type()))
                } else {
                    // 空列表，使用Number作为默认类型
                    DataType::List(Box::new(DataType::Number))
                }
            }
            DataValue::Dict(_) => DataType::Dict("key".to_string(), Box::new(DataType::Number)),
            DataValue::Model(_) => DataType::Model,
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
            (DataValue::Text(s), DataType::Number) => s
                .parse()
                .map(DataValue::Number)
                .map_err(|_| Error::ConversionError(format!("Cannot convert '{}' to number", s))),
            (DataValue::Text(s), DataType::Boolean) => match s.to_lowercase().as_str() {
                "true" | "yes" | "1" => Ok(DataValue::Boolean(true)),
                "false" | "no" | "0" => Ok(DataValue::Boolean(false)),
                _ => Err(Error::ConversionError(format!(
                    "Cannot convert '{}' to boolean",
                    s
                ))),
            },
            (DataValue::Boolean(b), DataType::Boolean) => Ok(DataValue::Boolean(*b)),
            (DataValue::Boolean(b), DataType::Number) => {
                Ok(DataValue::Number(if *b { 1.0 } else { 0.0 }))
            }
            (DataValue::Boolean(b), DataType::Text) => Ok(DataValue::Text(b.to_string())),
            (DataValue::Path(p), DataType::Path) => Ok(DataValue::Path(p.clone())),
            (DataValue::Path(p), DataType::Text) => {
                Ok(DataValue::Text(p.to_string_lossy().to_string()))
            }
            (DataValue::Binary(b), DataType::Binary) => Ok(DataValue::Binary(b.clone())),
            (DataValue::Binary(b), DataType::Text) => {
                Ok(DataValue::Text(general_purpose::STANDARD.encode(b)))
            }
            (DataValue::Text(s), DataType::Binary) => general_purpose::STANDARD
                .decode(s)
                .map(DataValue::Binary)
                .map_err(|_| Error::ConversionError("Cannot convert string to binary".to_string())),
            _ => Err(Error::TypeMismatch(format!(
                "Cannot convert {:?} to {:?}",
                self, target
            ))),
        }
    }
}
