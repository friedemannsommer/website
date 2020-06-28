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
use lambda_http::lambda;

fn main() {
    lambda!(simple_router)
}
