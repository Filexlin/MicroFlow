use serde_json::{from_value, json, to_value, Value};
use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;

use crate::manager::PythonManager;
use crate::protocol::{ExecutionResult, JsonRpcRequest, JsonRpcResponse};

#[derive(Error, Debug)]
pub enum PythonError {
    #[error("进程启动失败: {0}")]
    ProcessStartError(String),

    #[error("执行超时 (> {0}s)")]
    Timeout(u64),

    #[error("JSON 解析错误: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Python 执行错误: {0}")]
    ExecutionError(String),

    #[error("内存限制超出")]
    MemoryLimitExceeded,

    #[error("管理器错误: {0}")]
    ManagerError(#[from] crate::manager::PythonManagerError),

    #[error("I/O 错误: {0}")]
    IoError(#[from] std::io::Error),
}

pub struct PythonExecutor {
    manager: Arc<std::sync::Mutex<PythonManager>>,
}

impl PythonExecutor {
    pub fn new(max_pool_size: usize, timeout: Duration, python_path: &str) -> Self {
        let manager = PythonManager::new(max_pool_size, timeout, python_path);
        Self {
            manager: Arc::new(std::sync::Mutex::new(manager)),
        }
    }

    pub fn start(&self) -> Result<(), PythonError> {
        let mut manager = self
            .manager
            .lock()
            .map_err(|_| PythonError::ExecutionError("锁中毒".into()))?;

        manager.start().map_err(PythonError::ManagerError)?;
        Ok(())
    }

    pub fn stop(&self) {
        if let Ok(mut manager) = self.manager.lock() {
            manager.stop();
        }
    }

    /// 执行 Python 代码（Static 模式）
    pub fn execute(
        &self,
        code: &str,
        inputs: HashMap<String, Value>,
        _timeout: Duration,
    ) -> Result<HashMap<String, Value>, PythonError> {
        // 构建 JSON-RPC 请求
        let params = json!({
            "code": code,
            "inputs": inputs
        });

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "execute_python".to_string(),
            params,
            id: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
        };

        // 序列化请求
        let request_json = serde_json::to_string(&request).map_err(PythonError::JsonError)?;

        // 获取 Python 进程
        let mut manager = self
            .manager
            .lock()
            .map_err(|_| PythonError::ExecutionError("锁中毒".into()))?;

        let process = manager.get_process().map_err(PythonError::ManagerError)?;
        let process_id = process.id;

        // 发送请求
        writeln!(process.stdin, "{}", request_json).map_err(PythonError::IoError)?;
        process.stdin.flush().map_err(PythonError::IoError)?;

        // 读取响应
        let mut response_json = String::new();
        match process.stdout.read_line(&mut response_json) {
            Ok(_) => {
                // 解析响应
                let response: JsonRpcResponse =
                    serde_json::from_str(&response_json).map_err(PythonError::JsonError)?;

                // 检查是否有错误
                if let Some(error) = response.error {
                    // 释放进程
                    manager.release_process(process_id);
                    return Err(PythonError::ExecutionError(error.message));
                }

                // 处理结果
                if let Some(result) = response.result {
                    let outputs: HashMap<String, Value> =
                        from_value(result).map_err(PythonError::JsonError)?;

                    // 释放进程
                    manager.release_process(process_id);

                    Ok(outputs)
                } else {
                    // 释放进程
                    manager.release_process(process_id);
                    Err(PythonError::ExecutionError("Python 执行无结果".into()))
                }
            }
            Err(e) => {
                // I/O 错误，重启进程
                let _ = manager.restart_process(process_id);
                Err(PythonError::IoError(e))
            }
        }
    }

    /// 路径映射版本（支持显式路径传递）
    pub fn execute_with_paths(
        &self,
        code: String, // Template 模式已替换变量
        path_mappings: HashMap<String, PathBuf>,
        timeout: Duration,
    ) -> Result<ExecutionResult, PythonError> {
        // 将路径映射转换为字符串
        let path_inputs: HashMap<String, Value> = path_mappings
            .into_iter()
            .map(|(k, v)| (k, to_value(v.to_string_lossy().to_string()).unwrap()))
            .collect();

        // 执行代码
        let outputs = self.execute(&code, path_inputs, timeout)?;

        // 构建执行结果
        let result = ExecutionResult {
            outputs,
            success: true,
            error: None,
        };

        Ok(result)
    }
}
