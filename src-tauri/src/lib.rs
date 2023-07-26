use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;

use crate::models::Queue;
pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish("../database.db")
        .unwrap_or_else(|_| panic!("Error connecting to database"))
}

pub fn add_queue(game: &str, path: &str){
    let new_queue = models::InsertQueue{game, path};
    _ = diesel::insert_into(crate::schema::queue::table).values(&new_queue).on_conflict_do_nothing().execute(&mut establish_connection());
}

pub fn get_queue() -> Result<Option<Vec<Queue>>, diesel::result::Error>{
    use self::schema::queue::dsl::queue;
    return queue.select(Queue::as_select()).get_results(&mut establish_connection()).optional();
}