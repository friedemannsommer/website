#![warn(clippy::all)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate horrorshow;

mod constants;
mod routes;
mod site_request;
mod templates;
mod util;

use crate::routes::simple_router;
use lambda_http::{handler, lambda_runtime};

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    lambda_runtime::run(handler(|req, _| async { simple_router(req) })).await?;
    Ok(())
}
