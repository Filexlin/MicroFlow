use tauri::Manager;
use std::sync::Arc;
use tokio::sync::Mutex;
use microflow_core::vram::pool::VramPool;

pub struct AppState {
    pub vram_pool: Arc<Mutex<VramPool>>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello {}", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            vram_pool: Arc::new(Mutex::new(VramPool::new(2)))
        })
        .invoke_handler(tauri::generate_handler![
            greet
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
