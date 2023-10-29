#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use db::Database;
use tauri::{Manager, Size}; 
use std::sync::Mutex;
use window_shadows::set_shadow;
mod db;

pub struct DatabaseState(pub Mutex<db::Database>);

fn setup<'a>(app: &'a mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let window = app.get_window("main").unwrap();         
    window.set_size(Size::Logical(tauri::LogicalSize { width: 1280.0, height: 800.0})).unwrap();
    set_shadow(&window, true).unwrap();
    
    
    let path_manager = app.path().clone();


    let db = app.state::<DatabaseState>().clone();
    
    db.0.lock().unwrap().resources_dir = path_manager.resolve("resources/database.db", tauri::path::BaseDirectory::Resource).expect("Failed to find resources directory");
    db.0.lock().unwrap().data_dir = path_manager.resolve("data", tauri::path::BaseDirectory::AppData).expect("Failed to find data directory");
    tauri::async_runtime::spawn(async move {
        db.0.lock().unwrap().db_setup().await.expect("Failed to setup database");  
    });
    

    Ok(())
}

fn main() {    
    tauri::Builder::default()
        .manage(DatabaseState(Mutex::new(Database::default())))
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![add_app_queue, get_app_queue])
        .setup(setup)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn add_app_queue(_app_handle: tauri::AppHandle, _id: String, _path: String, db_state: tauri::State<'_, DatabaseState>) -> Result<String, String> {

    let mut db_pool = db_state.0.lock().unwrap();

    let asdas = db_pool.get_database().await;


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
    );
        //.add_db(db_pool).await;

    // match result {
    //     Ok(_) => println!("add ok"),
    //     Err(e) => println!("error: {}", e)
    // }

    println!("ping ok");
    Ok("Success".into())
}

#[tauri::command]
async fn get_app_queue(_app_handle: tauri::AppHandle) -> Result<String, String> {    
    Ok("".into())
}
