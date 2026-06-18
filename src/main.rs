use std::env;

#[derive(Debug)]
struct UpdateTodo {
    id: u32,
    title: String
}

#[derive(Debug)]
enum TodoOperations {
    Add(String),
    List,
    Get(u32),
    Update(UpdateTodo),
    Done(u32),
    Delete(u32),
    Unknown
}

fn process_env_args(args: &Vec<String>) -> TodoOperations {
    if args.len() < 2 {
        return TodoOperations::Unknown;
    }

    match args[1].as_str() {
        "add" => {
            match args.get(2) {
                Some(title) => TodoOperations::Add(title.clone()),
                _ => TodoOperations::Unknown
            }
        },
        "list" => TodoOperations::List,
        "get" => {
            match args.get(2).and_then(|s| s.parse().ok()) {
                Some(id) => TodoOperations::Get(id),
                _ => TodoOperations::Unknown
            }
        }
        "update" => {
            match (args.get(2).and_then(|s| s.parse().ok()), args.get(3)) {
                (Some(id), Some(title)) => TodoOperations::Update(UpdateTodo { id, title: title.clone() }),
                _ => TodoOperations::Unknown
            }
        }
        "done" => {
            match args.get(2).and_then(|s| s.parse().ok()) {
                Some(id) => TodoOperations::Done(id),
                _ => TodoOperations::Unknown
            }
        }
        "delete" => {
            match args.get(2).and_then(|s| s.parse().ok()) {
                Some(id) => TodoOperations::Delete(id),
                _ => TodoOperations::Unknown
            }
        }
        _ => TodoOperations::Unknown
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let todo_operation: TodoOperations = process_env_args(&args);

    println!("{todo_operation:?}")
}