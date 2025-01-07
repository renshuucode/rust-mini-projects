// src/bin/todo_cli/main.rs
use rust_mini_projects::todo::Todo;

fn main() {
    let mut todo = Todo::new();
    todo.add("Buy milk".to_string());
    todo.list();
}