
use sqlx::{migrate::MigrateDatabase, Row, Sqlite, SqlitePool};
use tauri::api::path::data_dir;


pub async fn create_database() {
    let db_dir = {
        let path = format!("sqlite:{}{}/com.zig.store/database.db", data_dir().unwrap().display(), std::path::MAIN_SEPARATOR);
        Box::leak(path.into_boxed_str())
    };


    if !Sqlite::database_exists(db_dir).await.unwrap_or(false) {
        println!("Creating database {}", db_dir);
        match Sqlite::create_database(db_dir).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
    let db = SqlitePool::connect(db_dir).await.unwrap();
    let result = sqlx::query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY NOT NULL, name VARCHAR(250) NOT NULL);").execute(&db).await.unwrap();
    println!("Create user table result: {:?}", result);
    let result = sqlx::query(
        "SELECT name
         FROM sqlite_schema
         WHERE type ='table' 
         AND name NOT LIKE 'sqlite_%';",
    )
    .fetch_all(&db)
    .await
    .unwrap();
    for (idx, row) in result.iter().enumerate() {
        println!("[{}]: {:?}", idx, row.get::<String, &str>("name"));
    }
}

pub async fn get_database(){
    // if !data_dir().unwrap().as_path().join("/com.zig.store/database.db").exists() { 
    //    match Sqlite::create_database(DB_URL).await {
    //         Ok(_) => println!("Create db success"),
    //         Err(error) => panic!("error: {}", error),
    //     }
    // }
    
    
    // let db = SqliteConnection::connect(&format!("sqlite::{}", data_dir().unwrap().as_path().join("/com.zig.store/database.db").as_path().display().to_string())).await.unwrap();
    
    // return db;
}