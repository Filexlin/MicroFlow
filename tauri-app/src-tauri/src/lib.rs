use microflow_core::python_library::{PythonLibrary, PythonScript};
use std::sync::Arc;
use std::path::PathBuf;
use std::collections::HashMap;
use tauri::State;
use tokio::sync::Mutex;

pub struct AppState {
    pub python_library: Arc<Mutex<PythonLibrary>>,
}

#[tauri::command]
async fn get_system_info() -> Result<String, String> {
    Ok("MicroFlow Ready".to_string())
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
        python_library: Arc::new(Mutex::new(PythonLibrary::new(library_path))),
    };
    
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            get_system_info,
            scan_python_library
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app, _event| {
        });
}
