use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use core::time::Duration;
use std::{path::Path, fs::File};
use log::LevelFilter;


pub async fn get_database() -> DatabaseConnection{
    if !Path::new("database.db").exists() {
        File::create("database.db").expect("Unable to create file");
    }


    let mut opt = ConnectOptions::new("sqlite://database.db");
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(LevelFilter::Info);

    let db = Database::connect(opt).await.unwrap();
    return db;
}