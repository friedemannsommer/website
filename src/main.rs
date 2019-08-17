#[macro_use]
extern crate lazy_static;

mod constants;
mod routes;
mod util;

use lambda_http::lambda;
use crate::routes::simple_router;

fn main() {
    lambda!(simple_router)
}
