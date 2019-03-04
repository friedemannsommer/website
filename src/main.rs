#[macro_use]
extern crate lazy_static;

use lambda_http::lambda;

mod constants;
mod routes;
mod util;

use crate::routes::simple_router;

fn main() {
    lambda!(simple_router)
}
