use crate::database;
use crate::models::{NewTodo};
use diesel::insert_into;
use diesel::prelude::*;

pub fn create_task(new_todo: &NewTodo) -> bool {
    use crate::schema::todo::dsl::*;

    let conn = database::establish_connection();

    match insert_into(todo).values(new_todo)
        .execute(&conn) {
        Ok(_i) => return true,
        Err(..) => return false
    };
}