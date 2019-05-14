extern crate rust_todo_cli;

use rust_todo_cli::repository::todo_repository;

fn main() {
    let results = todo_repository::find_all();
    println!("Displaying {} todo, max show is 30", results.len());

    if results.len() != 0 {
        for single_todo in results {
            println!("================== Task id: {} ==================", single_todo.todo_id);
            println!("Task: {}Priority: {}", single_todo.task, single_todo.priority);
        }
    } else {
        println!("Don't found TODO. Have a nice day.");
    }
}