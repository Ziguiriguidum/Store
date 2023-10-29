
use std::{path::PathBuf, fs};

use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tauri::api::path::data_dir;
pub mod games;

pub async fn db_setup<'a>(resources_dir: PathBuf) -> Result<bool, String> {
    if db_exists().await{
        return Ok(true); 
    }

    let db_dir = {
        let path = format!("{}{}com.zig.store/database.db", data_dir().unwrap().display(), std::path::MAIN_SEPARATOR);
        Box::leak(path.into_boxed_str())
    };
        
    println!("db_dir: {}", db_dir);
    println!("resource_dir: {}", resources_dir.to_str().unwrap().replace("\\\\?\\", ""));

    fs::copy(resources_dir.to_str().unwrap().replace("\\\\?\\", ""), db_dir).expect("Failed to copy database");

    return Ok(true);    
}

pub async fn db_exists() -> bool {
    let db_dir = {
        let path = format!("sqlite:{}{}com.zig.store/database.db", data_dir().unwrap().display(), std::path::MAIN_SEPARATOR);
        Box::leak(path.into_boxed_str())
    };

    match Sqlite::database_exists(db_dir).await {
        Ok(true) => return true,
        Ok(false) => return false,
        Err(_) => return false
    }
       
}

pub async fn get_database()-> sqlx::Pool<Sqlite>{
     let db_dir = {
        let path = format!("sqlite:{}{}com.zig.store/database.db", data_dir().unwrap().display(), std::path::MAIN_SEPARATOR);
        Box::leak(path.into_boxed_str())
    };
    let db = SqlitePool::connect(db_dir).await.unwrap();
    return db;
}