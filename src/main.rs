extern crate finchers;
extern crate hyper;
#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod todo;
mod errors;

use finchers::{Endpoint, Json};
use finchers::endpoint::method::{delete, get, post, put};
use finchers::endpoint::path::segment;
use finchers::endpoint::{json_body, PathExt};
use finchers::response::{Created, NoContent};
use finchers::server::Server;
use finchers::util::either::Either6;

use todo::{NewTodo, Todo};
use errors::ApiError;

fn main() {
    let endpoint = |_: &_| {
        // GET /todos/:id
        let get_todo = get(segment("todos").with(u64::PATH))
            .map(|id| Json(todo::get(id)));

        // GET /todos
        let get_todos = get(segment("todos"))
            .map(|()| Json(todo::list()));

        // DELETE /todos/:id
        let delete_todo = delete(segment("todos").with(u64::PATH))
            .map(|id| {
                todo::delete(id);
                NoContent
            });

        // DELETE /todos
        let delete_todos = delete(segment("todos"))
            .map(|()| {
                todo::clear();
            });

        // PUT /todos/:id
        let put_todo = put(segment("todos").with(u64::PATH))
            .join(json_body::<Todo, ApiError>())
            .map(|(id, Json(todo))| {
                todo::set(id, todo.clone());
                Json(todo)
            });

        // POST /todos
        let post_todo = post(segment("todos"))
            .with(json_body::<NewTodo, ApiError>())
            .map(|Json(new_todo)| {
                let todo = todo::save(new_todo);
                Created(Json(todo))
            });

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
