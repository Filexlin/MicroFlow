use microflow_core::vram::pool::VramPool;
use microflow_core::workflow::{
    detect_cycles, EdgeData, ExecutionContext, NodeData, WorkflowData, WorkflowExecutor,
};
use microflow_core::python_library::{PythonLibrary, PythonScript};
use std::sync::Arc;
use std::path::PathBuf;
use std::collections::HashMap;
use tauri::{command, AppHandle, Runtime, State};
use tokio::sync::Mutex;

pub struct AppState {
    pub vram_pool: Arc<Mutex<VramPool>>,
    pub executor: Arc<Mutex<WorkflowExecutor>>,
    pub python_library: Arc<Mutex<PythonLibrary>>,
}

#[derive(Debug)]
pub enum CommandError {
    Dialog(String),
    Io(String),
    Cancelled,
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::Dialog(s) => write!(f, "Dialog error: {}", s),
            CommandError::Io(s) => write!(f, "IO error: {}", s),
            CommandError::Cancelled => write!(f, "User cancelled"),
        }
    }
}

impl std::error::Error for CommandError {}

#[tauri::command]
async fn get_system_info() -> Result<String, String> {
    Ok("MicroFlow Ready".to_string())
}

#[tauri::command]
async fn execute_node(node_type: String, inputs: String) -> Result<String, String> {
    Ok(format!("Executed {} with {}", node_type, inputs))
}

#[tauri::command]
async fn save_workflow(nodes: Vec<NodeData>, edges: Vec<EdgeData>) -> Result<String, String> {
    let edge_pairs: Vec<(String, String)> = edges
        .iter()
        .map(|e| (e.source.clone(), e.target.clone()))
        .collect();

    detect_cycles(&edge_pairs)?;

    let workflow_data = WorkflowData {
        version: "1.0".to_string(),
        nodes,
        edges,
    };

    Ok(workflow_data.to_json())
}

#[tauri::command]
async fn load_workflow(json: String) -> Result<WorkflowData, String> {
    WorkflowData::from_json(&json)
}

#[tauri::command]
async fn execute_workflow(
    state: tauri::State<'_, AppState>,
    workflow_json: String,
) -> Result<String, String> {
    let workflow =
        WorkflowData::from_json(&workflow_json).map_err(|e| format!("解析失败: {}", e))?;

    let edge_pairs: Vec<(String, String)> = workflow
        .edges
        .iter()
        .map(|e| (e.source.clone(), e.target.clone()))
        .collect();
    detect_cycles(&edge_pairs).map_err(|e| e.to_string())?;

    let executor = state.executor.lock().await;
    let result = executor
        .execute_workflow(&workflow)
        .await
        .map_err(|e| format!("执行失败: {}", e))?;

    Ok(format!("执行成功: {:?}", result.final_outputs))
}

#[tauri::command]
async fn save_workflow_file(
    app: AppHandle<impl Runtime>,
    workflow: String,
) -> Result<String, CommandError> {
    let file_path = app.dialog()
        .file()
        .add_filter("MicroFlow", &["mflow"])
        .save_file()
        .await
        .map_err(|e| CommandError::Dialog(e.to_string()))?;
    
    let path: PathBuf = file_path.ok_or(CommandError::Cancelled)?;
    tokio::fs::write(&path, workflow).await
        .map_err(|e| CommandError::Io(e.to_string()))?;
    
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
async fn load_workflow_file(
    app: AppHandle<impl Runtime>,
) -> Result<String, CommandError> {
    let file_path = app.dialog()
        .file()
        .add_filter("MicroFlow", &["mflow"])
        .pick_file()
        .await
        .map_err(|e| CommandError::Dialog(e.to_string()))?;
    
    let path: PathBuf = file_path.ok_or(CommandError::Cancelled)?;
    let content = tokio::fs::read_to_string(&path).await
        .map_err(|e| CommandError::Io(e.to_string()))?;
    
    Ok(content)
}

#[tauri::command]
async fn scan_python_library(
    state: State<'_, AppState>,
) -> Result<HashMap<String, Vec<PythonScript>>, String> {
    let mut lib = state.python_library.lock().await;
    lib.scan().map_err(|e| e.to_string())?;
    
    let by_cat = lib.get_by_category();
    let result: HashMap<String, Vec<PythonScript>> = by_cat
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().cloned().collect()))
        .collect();
    
    Ok(result)
}

pub fn run() {
    let library_path = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".microflow")
        .join("python_library");
    
    let state = AppState {
        vram_pool: Arc::new(Mutex::new(VramPool::new(2))),
        executor: Arc::new(Mutex::new(WorkflowExecutor::new(ExecutionContext::new()))),
        python_library: Arc::new(Mutex::new(PythonLibrary::new(library_path))),
    };
    
    tauri::Builder::default()
        .manage(state)
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_system_info,
            execute_node,
            save_workflow,
            load_workflow,
            execute_workflow,
            save_workflow_file,
            load_workflow_file,
            scan_python_library
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app, _event| {
        });
}
