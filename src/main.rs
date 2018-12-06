extern crate actix;
extern crate actix_web;
extern crate tera;
#[macro_use]
extern crate lazy_static;

mod assets;
mod constants;
mod render;

use actix_web::{http, server, App};
use constants::{APP_NAME, DEFAULT_HOST, DEFAULT_PORT, ENV_HOST, ENV_PORT};
use std::env;

fn create_app() -> App {
    App::new()
        .route("/", http::Method::GET, render::index)
        .route("/contact", http::Method::GET, render::contact)
        .route("/assets/{name}", http::Method::GET, assets::handle_request)
        .default_resource(|r| r.with(render::not_found))
}

fn get_server_address() -> String {
    env::var(ENV_HOST)
        .unwrap_or(DEFAULT_HOST.to_string())
        .to_owned()
        + ":"
        + &env::var(ENV_PORT).unwrap_or(DEFAULT_PORT.to_string())
}

fn main() {
    let sys = actix::System::new(APP_NAME);
    let address = get_server_address();

    server::new(|| create_app()).bind(&address).unwrap().start();

    println!("Started http server: {}", &address);
    sys.run();
}
