use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Todo {
    id: u64,
    title: String,
    completed: bool,
    order: usize,
}

#[derive(Deserialize)]
pub struct NewTodo {
    title: String,
    completed: Option<bool>,
    order: Option<usize>,
}

#[derive(Default)]
struct Todos {
    db: HashMap<u64, Todo>,
    counter: u64,
}

lazy_static! {
        static ref TODOS: RwLock<Todos> = RwLock::new(Todos::default());
    }

pub fn get(id: u64) -> Option<Todo> {
    let todos = TODOS.read().unwrap();
    todos.db.get(&id).cloned()
}

pub fn set(id: u64, new_todo: Todo) {
    let mut todos = TODOS.write().unwrap();
    if let Some(todo) = todos.db.get_mut(&id) {
        *todo = new_todo;
    }
}

pub fn list() -> Vec<Todo> {
    let todos = TODOS.read().unwrap();
    todos.db.iter().map(|i| i.1.clone()).collect()
}

pub fn save(new_todo: NewTodo) -> Todo {
    let mut todos = TODOS.write().unwrap();
    todos.counter += 1;
    let todo = Todo {
        id: todos.counter,
        title: new_todo.title,
        completed: new_todo.completed.unwrap_or(false),
        order: new_todo.order.unwrap_or(0),
    };

    todos.db.insert(todo.id, todo.clone());
    todo
}

pub fn delete(id: u64) {
    let mut todos = TODOS.write().unwrap();
    todos.db.remove(&id);
}

pub fn clear() {
    let mut todos = TODOS.write().unwrap();
    todos.db.clear();
}
