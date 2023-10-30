
use std::{path::PathBuf, fs};

use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
pub mod games;

pub struct Database{
    pub resources_dir: PathBuf,
    pub db_dir: String,
    pub data_dir: PathBuf
}

impl Database{
    pub async fn db_setup<'a>(&mut self) -> Result<bool, String> {
        if self.db_exists().await{
            return Ok(true); 
        } 

        self.db_dir = format!("{}{}/database.db", self.data_dir.display(), std::path::MAIN_SEPARATOR);
            
        println!("db_dir: {}", self.db_dir);
        println!("resources_dir: {}", self.resources_dir.to_str().unwrap().replace("\\\\?\\", ""));

        fs::create_dir(self.db_dir.clone().replace("database.db", "")).expect("Failed to create database directory");
        fs::copy(self.resources_dir.to_str().unwrap().replace("\\\\?\\", ""), self.db_dir.clone()).expect("Failed to copy database");

        return Ok(true);    
    }

    pub async fn db_exists(&mut self) -> bool {
        let db_dir = {
            let path = format!("sqlite:{}{}/database.db",self.data_dir.display(), std::path::MAIN_SEPARATOR);
            Box::leak(path.into_boxed_str())
        };

        match Sqlite::database_exists(db_dir).await {
            Ok(true) => return true,
            Ok(false) => return false,
            Err(_) => return false
        }
        
    }

    pub async fn get_database(&mut self)-> sqlx::Pool<Sqlite>{
        let db_dir = {
            let path = format!("sqlite:{}{}com.zig.store/database.db", self.data_dir.display(), std::path::MAIN_SEPARATOR);
            Box::leak(path.into_boxed_str())
        };
        let db = SqlitePool::connect(db_dir).await.unwrap();
        return db;
    }

    pub fn default() -> Database {
        Database { 
            resources_dir: PathBuf::from(""),
            db_dir: String::from(""),
            data_dir: PathBuf::from(""),
        }
    }

}