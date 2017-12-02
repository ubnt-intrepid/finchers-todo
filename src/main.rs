extern crate finchers;
extern crate hyper;
#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use finchers::{Endpoint, Json};
use finchers::endpoint::method::{delete, get, post, put};
use finchers::endpoint::{json_body, PathExt};
use finchers::response::{Created, Responder, Response};
use finchers::server::Server;
use finchers::util::NoReturn;
use finchers::util::either::Either6;
use hyper::StatusCode;


mod todo {
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
}


enum ApiError {
    ParseBody,
    Unknown,
}

impl Responder for ApiError {
    type Error = NoReturn;
    fn respond(self) -> Result<Response, Self::Error> {
        use ApiError::*;
        match self {
            ParseBody => Ok(Response::new().with_status(StatusCode::BadRequest)),
            Unknown => unreachable!(),
        }
    }
}


fn main() {
    let endpoint = |_: &_| {
        use todo::{NewTodo, Todo};

        let todos = || "todos".map_err(|_| ApiError::Unknown);
        let todos_id = || "todos".with(u64::PATH).map_err(|_| ApiError::Unknown);

        // GET /todos/:id
        let get_todo = get(todos_id()).map(|id| Json(todo::get(id)));

        // GET /todos
        let get_todos = get(todos()).map(|()| Json(todo::list()));

        // DELETE /todos/:id
        let delete_todo = delete(todos_id()).map(|id| { todo::delete(id); });

        // DELETE /todos
        let delete_todos = delete(todos()).map(|()| { todo::clear(); });

        // PUT /todos/:id
        let put_todo = put(todos_id())
            .join(json_body::<Todo>().map_err(|_| ApiError::ParseBody))
            .map(|(id, Json(new_todo))| { todo::set(id, new_todo); });

        // POST /todos
        let post_todo = post(todos())
            .with(json_body::<NewTodo>().map_err(|_| ApiError::ParseBody))
            .map(|Json(new_todo)| Created(Json(todo::save(new_todo))));

        (get_todo.map(Either6::E1))
            .or(get_todos.map(Either6::E2))
            .or(delete_todo.map(Either6::E3))
            .or(delete_todos.map(Either6::E4))
            .or(put_todo.map(Either6::E5))
            .or(post_todo.map(Either6::E6))
    };

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);
    Server::new(endpoint)
        .bind(format!("0.0.0.0:{}", port))
        .run_http();
}
