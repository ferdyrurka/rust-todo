use crate::database;
use crate::models::{NewTodo, Todo};
use crate::schema::todo::dsl::*;
use std::vec::Vec;
use diesel::prelude::*;

pub fn find_all() -> Vec<Todo> {
    return todo.limit(30)
        .load::<Todo>(&database::establish_connection())
        .expect("Error loading todo")
    ;
}

pub fn create_task(new_todo: NewTodo) -> bool {
    match diesel::insert_into(todo).values(&new_todo)
        .execute(&database::establish_connection()) {
        Ok(..) => return true,
        Err(..) => return false
    };
}

pub fn delete_task(todo_id_delete:i32) -> bool {
    match diesel::delete(todo.filter(todo_id.eq(todo_id_delete)))
        .execute(&database::establish_connection()) {
        Ok(..) => return true,
        Err(..) => return false
    };
}