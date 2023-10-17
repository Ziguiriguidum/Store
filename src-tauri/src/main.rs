#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, Size}; 
use window_shadows::set_shadow;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![add_app_queue, get_app_queue])
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            

            window
                .set_size(Size::Logical(tauri::LogicalSize {
                    width: 1280.0,
                    height: 800.0,
                }))
                .unwrap();

            set_shadow(&window, true).unwrap();            

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn add_app_queue(app_handle: tauri::AppHandle, id: String, path: String) -> Result<String, String> {    
    
    Ok("Success".into())
}

#[tauri::command]
async fn get_app_queue(app_handle: tauri::AppHandle) -> Result<String, String> {

    Ok("".into())
}
