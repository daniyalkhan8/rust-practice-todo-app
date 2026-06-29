use serde::{Deserialize, Serialize};
use std::env;

const TODOS_PATH: &str = "todos.json";

#[derive(Debug)]
struct UpdateTodo {
    id: u32,
    title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todo {
    id: u32,
    title: String,
    done: bool,
}

#[derive(Debug)]
struct JsonTodos {
    todos: Vec<Todo>,
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
        let mut id: u32 = 0;
        if self.todos.len() > 0 {
            if let Some(newest_todo) = self.todos.iter().max_by_key(|todo| todo.id) {
                id = newest_todo.id + 1;
            }
        }

        let done = false;
        let todo = Todo { id, title, done };
        self.todos.push(todo);

        let todos_updated = serde_json::to_string(&self.todos).unwrap();
        std::fs::write(TODOS_PATH, todos_updated).expect("Failed to write to todos.json");
    }

    fn get(self, id: u32) {
        let todo: Vec<Todo> = self
            .todos
            .iter()
            .filter(|todo| todo.id == id)
            .cloned()
            .collect();
        println!("{:#?}", todo);
    }

    fn list(self) {
        println!("{:#?}", self.todos);
    }

    fn update(mut self, update_todo: UpdateTodo) {
        self.todos = self
            .todos
            .into_iter()
            .map(|mut todo| {
                if todo.id == update_todo.id {
                    todo.title = update_todo.title.clone()
                }
                todo
            })
            .collect();

        let todos_updated = serde_json::to_string(&self.todos).unwrap();
        std::fs::write(TODOS_PATH, todos_updated).expect("Failed to write to todos.json");
    }

    fn done(mut self, id: u32) {
        self.todos = self
            .todos
            .into_iter()
            .map(|mut todo| {
                if todo.id == id {
                    todo.done = true;
                }
                todo
            })
            .collect();

        let todos_updated = serde_json::to_string(&self.todos).unwrap();
        std::fs::write(TODOS_PATH, todos_updated).expect("Failed to write to todos.json");
    }

    fn delete(mut self, id: u32) {
        self.todos = self
            .todos
            .into_iter()
            .filter(|todo| todo.id != id)
            .collect();
        let todos_updated = serde_json::to_string(&self.todos).unwrap();
        std::fs::write(TODOS_PATH, todos_updated).expect("Failed to write to todos.json");
    }
}

#[derive(Debug)]
enum TodoOperations {
    Add(String),
    List,
    Get(u32),
    Update(UpdateTodo),
    Done(u32),
    Delete(u32),
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let todo_operation = process_env_args(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        std::process::exit(1);
    });
    let todos = JsonTodos::new();

    match todo_operation {
        TodoOperations::Add(title) => todos.add(title),
        TodoOperations::Get(id) => todos.get(id),
        TodoOperations::List => todos.list(),
        TodoOperations::Update(update_todo) => todos.update(update_todo),
        TodoOperations::Done(id) => todos.done(id),
        TodoOperations::Delete(id) => todos.delete(id),
    }
}
