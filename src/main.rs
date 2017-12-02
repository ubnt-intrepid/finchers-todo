extern crate futures;
extern crate finchers;

use futures::future::{FutureResult, ok};
use finchers::{Context, Endpoint, Server};
use finchers::endpoint::EndpointResult;
use finchers::endpoint::method::get;

struct Empty;

impl Endpoint for Empty {
    type Item = ();
    type Error = finchers::util::NoReturn;
    type Future = FutureResult<Self::Item, Self::Error>;
    fn apply(self, _: &mut Context) -> EndpointResult<Self::Future> {
        Ok(ok(()))
    }
}

fn empty() -> Empty {
    Empty
}

fn main() {
    let endpoint = |_: &_| get(empty()).map(|_| "Hello, Heroku!");

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);
    Server::new(endpoint)
        .bind(format!("0.0.0.0:{}", port))
        .run_http();
}
