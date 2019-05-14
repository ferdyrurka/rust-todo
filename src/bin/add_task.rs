extern crate rust_todo_cli;

use rust_todo_cli::repository::todo_repository;
use rust_todo_cli::models::NewTodo;
use std::io::{self};
use htmlescape;

fn main() {
    let stdin = io::stdin();

    println!("Write your task message: ");
    let mut task_args:String = String::new();
    stdin.read_line(&mut task_args).ok().expect("Failed to read task");
    let task = htmlescape::encode_minimal(&task_args.to_owned());

    println!("Write priority task: ");
    let mut priority_args:String = String::new();
    stdin.read_line(&mut priority_args).ok().expect("Failed to read priority");

    let priority:i16;
    match priority_args.trim().parse::<i16>() {
        Ok(i) => priority = i,
        Err(..) => {
            println!("Priority is not integer!");
            return;
        }
    };

    if priority < 1 || priority > 3 {
        println!("Minimum priority is 1 but maximum priority is 3!");
        return;
    }

    let new_todo:NewTodo = NewTodo {
        task: &task,
        priority: &priority
    };

    if !todo_repository::create_task(new_todo) {
        println!("Something went wrong");
        return;
    }

    println!("Saved successfully");
}