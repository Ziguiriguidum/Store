#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, Size}; 
use window_shadows::set_shadow;
mod db;

fn setup<'a>(app: &'a mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let window = app.get_window("main").unwrap();         
    window.set_size(Size::Logical(tauri::LogicalSize { width: 1280.0, height: 800.0})).unwrap();
    set_shadow(&window, true).unwrap();


    println!("{}", app.path_resolver().resource_dir().unwrap().display());

    tauri::async_runtime::spawn(async move {
      db::db_setup(app.path_resolver()).await.unwrap();    
    });       



   




    Ok(())
}

fn main() {    
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![add_app_queue, get_app_queue])
        .setup(setup)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn add_app_queue(_app_handle: tauri::AppHandle, _id: String, _path: String) -> Result<String, String> {        
    //db::create_database().await;
    let result = db::games::Games::new(1, "id".to_string(), "platform".to_string(), "name".to_string(), "version".to_string(), "sceneGroup".to_string(), "magnet".to_string(), 1.0, "installer".to_string(), "page".to_string()).add_db().await;
    match result {
        Ok(_) => println!("add ok"),
        Err(E) => println!("error: {}", E)
    }

    println!("ping ok");
    Ok("Success".into())
}

#[tauri::command]
async fn get_app_queue(_app_handle: tauri::AppHandle) -> Result<String, String> {    
    Ok("".into())
}
