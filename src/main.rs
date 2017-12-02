extern crate futures;
extern crate finchers;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use futures::future::{FutureResult, ok};
use finchers::{Context, Endpoint, Server};
use finchers::endpoint::EndpointResult;
use finchers::endpoint::method::get;

#[derive(Debug, StructOpt)]
#[structopt(name = "finchers-todo", about = "Todo app, using finchers")]
struct Options {
    #[structopt(short = "h", long = "host", default_value = "0.0.0.0")]
    host: String,

    #[structopt(short = "p", long = "port", default_value = "7878")]
    port: u16,
}

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
    let options: Options = structopt::StructOpt::from_args();

    let endpoint = |_: &_| get(empty()).map(|_| "Hello, Heroku!");

    Server::new(endpoint)
        .bind(format!("{}:{}", options.host, options.port))
        .run_http();
}
