#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::PathBuf;

use serde_json::json;
use tauri::{Manager, Size};
use tauri_plugin_store::StoreCollection;
use window_shadows::set_shadow;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![add_app_queue])
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

// #[derive(serde::Deserialize)]
// struct ItemQueue {
//     id: i32,
//     path: String,
// }

// #[derive(serde::Deserialize)]
// struct Queue {
//     queue: Vec<ItemQueue>,
// }

#[tauri::command]
async fn add_app_queue(app: tauri::AppHandle, _id: i32, _path: String) {
    use tauri::{Manager, Wry};
    use tauri_plugin_store::with_store;

    let stores = app.state::<StoreCollection<Wry>>();
    let path = PathBuf::from(".settings.dat");
    with_store(app.app_handle(), stores, path, |store| {
        store.insert("a".to_string(), json!("b"));
        Ok(())
    });
}
