use crate::database;
use crate::models::{NewTodo};
use diesel::prelude::*;

pub fn create_task(new_todo: NewTodo) -> bool {
    use crate::schema::todo::dsl::*;

    let conn = database::establish_connection();

    match diesel::insert_into(todo).values(&new_todo)
        .execute(&conn) {
        Ok(..) => return true,
        Err(..) => return false
    };
}

pub fn delete_task(todo_id_delete:i32) -> bool {
    use crate::schema::todo::dsl::*;

    let conn= database::establish_connection();

    match diesel::delete(todo.filter(todo_id.eq(todo_id_delete)))
        .execute(&conn) {
        Ok(..) => return true,
        Err(..) => return false
    };
}