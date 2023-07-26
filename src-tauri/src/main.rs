#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::add_queue;
use tauri::{Manager, Size}; 
use window_shadows::set_shadow;
mod use_store;
mod game_list;
use app::get_queue;

static mut APP_DIR: Option<String> = None;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![add_app_queue, get_app_queue, fetch_games])
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
async fn fetch_games(app_handle: tauri::AppHandle) -> Result<String, String> {        
    Ok("Success".into())
}

#[tauri::command]
async fn add_app_queue(app_handle: tauri::AppHandle, id: String, path: String) -> Result<String, String> {    
    add_queue(id.as_str(), path.as_str());
    Ok("Success".into())
}

#[tauri::command]
async fn get_app_queue(app_handle: tauri::AppHandle) -> Result<Vec<app::models::Queue>, String> {
    if get_queue().unwrap().is_none() {
        return Err("No queue found".into());
    };
    Ok(get_queue().unwrap().unwrap())
}
