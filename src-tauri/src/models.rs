use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::queue)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Queue {
    pub id: i32,
    pub game: String,
    pub path: String,
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::queue)]
pub struct InsertQueue<'a> {
    pub game: &'a str,
    pub path: &'a str,
}