use database;
use models::{NewTodo, Todo};
use diesel::insert_into;

pub fn create_task(new_todo: &NewTodo) {
    use schema::todo;

    diesel::insert_into(todo::table)
        .values(&new_todo)
        .expect("Error saving new todo")
    ;
}