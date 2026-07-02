mod todos;
mod db_config;

use db_config::establish_connection;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug)]
enum TodoOperations {
    Add(String),
    List,
    Get(u32),
    Update(UpdateTodo),
    Done(u32),
    Delete(u32),
}

#[derive(Debug)]
struct UpdateTodo {
    id: u32,
    title: String,
}

fn process_env_args(args: &Vec<String>) -> Result<TodoOperations, String> {
    if args.len() < 2 {
        return Err("No operation parameters provided. Please provide either add, get, list, update, done or delete.".to_string());
    }

    match args[1].as_str() {
        "add" => match args.get(2) {
            Some(title) => Ok(TodoOperations::Add(title.clone())),
            _ => Err("Failed to add the todo.".to_string()),
        },
        "list" => Ok(TodoOperations::List),
        "get" => match args.get(2).and_then(|s| s.parse().ok()) {
            Some(id) => Ok(TodoOperations::Get(id)),
            _ => Err("Failed to get the todo.".to_string()),
        },
        "update" => match (args.get(2).and_then(|s| s.parse().ok()), args.get(3)) {
            (Some(id), Some(title)) => Ok(TodoOperations::Update(UpdateTodo {
                id,
                title: title.clone(),
            })),
            _ => Err("Failed to update todo.".to_string()),
        },
        "done" => match args.get(2).and_then(|s| s.parse().ok()) {
            Some(id) => Ok(TodoOperations::Done(id)),
            _ => Err("Failed to mark todo as done.".to_string()),
        },
        "delete" => match args.get(2).and_then(|s| s.parse().ok()) {
            Some(id) => Ok(TodoOperations::Delete(id)),
            _ => Err("Failed to delete todo.".to_string()),
        },
        _ => Err("Please provide either add, get, list, update, done or delete.".to_string()),
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let todo_operation = process_env_args(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        std::process::exit(1);
    });

    let connection_pool = db_config::establish_connection().await.unwrap_or_else(|err| {
        eprintln!("{err}");
        std::process::exit(1);
    });

    match todo_operation {
        TodoOperations::Add(title) => {
            let id = todos::add_todo(&connection_pool, title).await.unwrap_or_else(|err| {
                eprintln!("{err}");
                std::process::exit(1);
            });
            println!("Added the todo with ID: {id}");
        },
        TodoOperations::Get(id) => {
            let todo = todos::get_todo(&connection_pool, id).await.unwrap_or_else(|err| {
                eprintln!("{err}");
                std::process::exit(1);
            });
            println!("{todo:#?}")
        },
        TodoOperations::List => {
            let todos_list = todos::list_todos(&connection_pool).await.unwrap_or_else(|err| {
                eprintln!("{err}");
                std::process::exit(1);
            });
            println!("{todos_list:#?}")
        },
        TodoOperations::Update(update_todo) => {},
        TodoOperations::Done(id) => {},
        TodoOperations::Delete(id) => {},
    }
}
