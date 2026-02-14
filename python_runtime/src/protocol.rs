use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteRequest {
    pub node_id: String,
    pub code: String,
    pub inputs: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExecuteResponse {
    pub success: bool,
    pub outputs: HashMap<String, serde_json::Value>,
    pub error: Option<String>,
}

// JSON-RPC 2.0 请求
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String, // "2.0"
    pub method: String,  // "execute_python"
    pub params: serde_json::Value,
    pub id: u64,
}

// JSON-RPC 2.0 响应
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub result: Option<serde_json::Value>,
    pub error: Option<JsonRpcError>,
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

// 执行结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExecutionResult {
    pub outputs: HashMap<String, serde_json::Value>,
    pub success: bool,
    pub error: Option<String>,
}
