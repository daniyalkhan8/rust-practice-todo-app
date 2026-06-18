use std::env;
use serde::{Deserialize, Serialize};

const TODOS_PATH: &str = "todos.json";

#[derive(Debug)]
struct UpdateTodo {
    id: u32,
    title: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: u32,
    title: String,
    done: bool
}

#[derive(Debug)]
struct JsonTodos {
    todos: Vec<Todo>
}

impl JsonTodos {
    fn new() -> JsonTodos {
        if !std::path::Path::new(TODOS_PATH).exists() {
            std::fs::write(TODOS_PATH, "[]").expect("Failed to create todos.json");
        }

        let content = std::fs::read_to_string(TODOS_PATH).expect("Failed to read todos.json");
        let todos: Vec<Todo> = serde_json::from_str(&content).expect("Failed to parse todos.json");

        JsonTodos { todos }
    }

    fn add(mut self, title: String) {
        let id: u32;
        if self.todos.len() > 0 {
            id = self.todos[self.todos.len() - 1].id + 1
        } else {
            id = 0
        }

        let done = false;
        let todo = Todo {id, title, done };
        self.todos.push(todo);

        let todos_updated = serde_json::to_string(&self.todos).unwrap();
        std::fs::write(TODOS_PATH, todos_updated).expect("Failed to write to todos.json");
    }

    fn get(self, id: u32) {}

    fn list(self) {
        println!("{:#?}", self.todos);
    }
    fn update(self, todo: UpdateTodo) {}
    fn done(self, id: u32) {}
    fn delete(self, id: u32) {}
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
    let todo_operation = process_env_args(&args);
    let todos = JsonTodos::new();

    match todo_operation {
        TodoOperations::Add(title) => todos.add(title),
        TodoOperations::List => todos.list(),
        _ => println!("debug")
    }
}