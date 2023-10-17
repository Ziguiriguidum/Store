use sea_orm::{ Database, DatabaseConnection};
use std::fs::File;
use tauri::api::path::data_dir;


pub async fn get_database() -> DatabaseConnection{

    if !data_dir().unwrap().as_path().join("/com.zig.store/database.db").exists() { 
        std::fs::create_dir_all(data_dir().unwrap().as_path().join("/com.zig.store")).expect("Unable to create directory");         
        File::create(data_dir().unwrap().as_path().join("/com.zig.store/database.db")).expect("Unable to create file");
    }

    let db = Database::connect(format!("sqlite://{}", data_dir().unwrap().as_path().join("/com.zig.store/database.db").as_path().display().to_string())).await.unwrap();
    db.ping().await.unwrap();
    return db;
}