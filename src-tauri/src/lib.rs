#[tauri::command]
fn execute_workflow(nodes: serde_json::Value) -> Result<String, String> {
    println!("执行: {:?}", nodes);
    Ok("成功".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![execute_workflow])
        .run(tauri::generate_context!())
        .expect("error");
}