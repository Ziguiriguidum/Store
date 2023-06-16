#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::PathBuf;

use serde_json::json;
use tauri::{Manager, Size, State, Wry};
use tauri_plugin_store::{with_store, StoreCollection};
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

#[derive(serde::Deserialize)]
struct ItemQueue {
    id: i32,
    path: String,
}

#[derive(serde::Deserialize)]
struct Queue {
    queue: Vec<ItemQueue>,
}

#[tauri::command]
async fn add_app_queue(app_handle: tauri::AppHandle, stores: State<'_, StoreCollection<Wry>>) -> Result<(), tauri_plugin_store::Error>  {

    let path = app_handle.path_resolver().app_data_dir().unwrap().join(".settings.dat");

    with_store(app_handle, stores, path, |store| {
        store.insert("queue".into(), json!("trial"))
    })?;

    Ok(())   
}
