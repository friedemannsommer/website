#[macro_use]
extern crate lazy_static;

mod asset_request;
mod constants;
mod routes;
mod site_request;
mod util;

use crate::routes::simple_router;
use lambda_http::lambda;

fn main() {
    lambda!(simple_router)
}
