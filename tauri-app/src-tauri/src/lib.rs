use tauri::Manager;
use std::sync::Arc;
use tokio::sync::Mutex;
use microflow_core::vram::pool::VramPool;
use microflow_core::workflow::{detect_cycles, WorkflowData, NodeData, EdgeData};
use serde::{Deserialize, Serialize};

pub struct AppState {
    pub vram_pool: Arc<Mutex<VramPool>>,
}

#[tauri::command]
async fn get_system_info() -> Result<String, String> {
    Ok("MicroFlow Ready".to_string())
}

#[tauri::command]
async fn execute_node(node_type: String, inputs: String) -> Result<String, String> {
    // 先返回mock数据，验证IPC通信
    Ok(format!("Executed {} with {}", node_type, inputs))
}

#[tauri::command]
async fn save_workflow(nodes: Vec<NodeData>, edges: Vec<EdgeData>) -> Result<String, String> {
    // 1. 检测循环依赖
    let edge_pairs: Vec<(String, String)> = edges
        .iter()
        .map(|e| (e.source.clone(), e.target.clone()))
        .collect();
    
    detect_cycles(&edge_pairs)?;
    
    // 2. 构建工作流数据
    let workflow_data = WorkflowData {
        version: "1.0".to_string(),
        nodes,
        edges,
    };
    
    // 3. 转JSON返回
    Ok(workflow_data.to_json())
}

#[tauri::command]
async fn load_workflow(json: String) -> Result<WorkflowData, String> {
    // 从JSON加载工作流数据
    WorkflowData::from_json(&json)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            vram_pool: Arc::new(Mutex::new(VramPool::new(2)))
        })
        .invoke_handler(tauri::generate_handler![
            get_system_info,
            execute_node,
            save_workflow,
            load_workflow
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
