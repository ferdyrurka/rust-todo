use crate::database;
use crate::models::{NewTodo};
use diesel;

pub fn create_task(new_todo: &NewTodo) {
    use crate::schema::todo;

    let conn = database::establish_connection();

    diesel::insert_into(todo::table)
        .values(new_todo)
    ;
}