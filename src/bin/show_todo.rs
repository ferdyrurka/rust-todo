extern crate rust_todo_cli;

use diesel::prelude::*;
use rust_todo_cli::database;
use rust_todo_cli::models::Todo;

fn main() {
    use rust_todo_cli::schema::todo::dsl::*;

    let connection = database::establish_connection();
    let results = todo.load::<Todo>(&connection)
        .expect("Error loading todo")
    ;

    println!("Displaying {} todo", results.len());

    if results.len() != 0 {
        for single_todo in results {
            println!("================== Task id: {} ==================", single_todo.todo_id);
            println!("Task: {}Priority: {}", single_todo.task, single_todo.priority);
        }
    } else {
        println!("Don't found TODO. Have a nice day.");
    }
}