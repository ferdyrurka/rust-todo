extern crate rust_todo_cli;

use rust_todo_cli::repository::todo_repository;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task_id:i32;

    match args[1].trim().parse::<i32>() {
        Ok(i) => task_id = i,
        Err(..) => {
            println!("Task id is not integer!");
            return;
        }
    };

    if !todo_repository::delete_task(task_id) {
        println!("Failed delete todo!");
        return;
    }

    println!("Delete todo successfully!");
}