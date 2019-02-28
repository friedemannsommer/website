extern crate actix;
extern crate actix_web;
extern crate handlebars;
extern crate html_minifier;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_json;

mod constants;
mod routes;

use actix_web::{http, server, App};
use constants::{APP_NAME, DEFAULT_HOST, DEFAULT_PORT, ENV_HOST, ENV_PATH, ENV_PORT};
use std::env;

fn create_app() -> App {
    App::new()
        .route("/", http::Method::GET, routes::index)
        .route("/contact", http::Method::GET, routes::contact)
        .default_resource(|r| r.with(routes::not_found))
}

fn get_socket_path() -> Option<String> {
    let path = env::var_os(ENV_PATH)?;

    Some(String::from(path.to_str()?))
}

fn get_server_address() -> String {
    env::var(ENV_HOST)
        .unwrap_or_else(|_| DEFAULT_HOST.to_string())
        .to_owned()
        + ":"
        + &env::var(ENV_PORT).unwrap_or_else(|_| DEFAULT_PORT.to_string())
}

fn main() {
    let sys = actix::System::new(APP_NAME);
    let http_server = server::new(create_app);
    let socket = get_socket_path();

    if socket.is_some() {
        let path = socket.unwrap();

        http_server.bind(&path).unwrap().start();
        println!("Started http server: {}", &path);
    } else {
        let address = get_server_address();

        http_server.bind(&address).unwrap().start();
        println!("Started http server: {}", &address);
    }

    sys.run();
}
