use crate::db::Database;
use std::collections::HashMap;

pub async fn update_game_list(){
    let data = reqwest::get("https://store.caiocinel.com/api/list")
        .await.unwrap()
        .json::<HashMap<String, String>>()
        .await.unwrap();

    println!("{:#?}", data);
}