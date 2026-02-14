use tauri::Manager;
use std::sync::Arc;
use tokio::sync::Mutex;
use microflow_core::vram::pool::VramPool;

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            vram_pool: Arc::new(Mutex::new(VramPool::new(2)))
        })
        .invoke_handler(tauri::generate_handler![
            get_system_info,
            execute_node
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
