use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteRequest {
    pub node_id: String,
    pub code: String,
    pub inputs: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteResponse {
    pub success: bool,
    pub outputs: HashMap<String, serde_json::Value>,
    pub error: Option<String>,
}