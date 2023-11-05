#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use db::Database;
use tauri::{Manager, Size}; 
use tokio::sync::Mutex;
mod db;

fn setup<'a>(app: &'a mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let window = app.get_window("main").unwrap();         
    window.set_size(Size::Logical(tauri::LogicalSize { width: 1280.0, height: 800.0})).unwrap();
    
    let db = app.state::<Mutex<Database>>();
    
    match tokio::runtime::Runtime::new() {
        Ok(runtime) => runtime.block_on(async {
            db.lock().await.resources_dir = app.path().resolve("resources/database.db", tauri::path::BaseDirectory::Resource).expect("Failed to find resources directory");
            db.lock().await.data_dir = app.path().resolve("data", tauri::path::BaseDirectory::AppData).expect("Failed to find data directory");
            db.lock().await.db_setup().await.expect("Failed to setup database");  
        }),
        Err(_) => panic!("error creating resources"),
    };

    Ok(())
}

fn main() {    
    let db_mng = Mutex::new(Database::default());

    tauri::Builder::default()
        .manage(db_mng)
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![add_app_queue])
        .setup(setup)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn add_app_queue(_app_handle: tauri::AppHandle, _id: String, _path: String, db_state: tauri::State<'_, Mutex<Database>>) -> Result<String, String> {
    
    let db_pool = db_state.lock().await.get_database().await;

    let result = db::games::Games::new(
        1, 
        "id".to_string(), 
        "platform".to_string(), 
        "name".to_string(), 
        "version".to_string(), 
        "sceneGroup".to_string(), 
        "magnet".to_string(), 
        1.0, 
        "installer".to_string(), 
        "page".to_string()
    ).add_db(db_pool).await;

    match result {
        Ok(_) => println!("add ok"),
        Err(e) => println!("error: {}", e)
    }

    
    println!("ping ok");
    Ok("Success".into())
}