
use sqlx::{migrate::MigrateDatabase, Row, Sqlite, SqlitePool};
use tauri::{api::path::{data_dir, resource_dir}, PathResolver};
pub mod games;

pub async fn db_setup<'a>(path_resolver: PathResolver) -> Result<bool, String> {
    if db_exists().await{
        return Ok(true); 
    }

    let db_dir = {
        let path = format!("sqlite:{}{}com.zig.store/database.db", data_dir().unwrap().display(), std::path::MAIN_SEPARATOR);
        Box::leak(path.into_boxed_str())
    };

    let resourceDir = path_resolver.resolve_resource("database.db").expect("Failed to find database resource directory");
    println!("{}", resourceDir.display());


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

pub async fn copy_template_database() -> bool {
    return true;
}

pub async fn get_database()-> sqlx::Pool<Sqlite>{
     let db_dir = {
        let path = format!("sqlite:{}{}com.zig.store/database.db", data_dir().unwrap().display(), std::path::MAIN_SEPARATOR);
        Box::leak(path.into_boxed_str())
    };
    let db = SqlitePool::connect(db_dir).await.unwrap();
    return db;
}