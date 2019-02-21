extern crate actix;
extern crate actix_web;
extern crate tera;
#[macro_use]
extern crate lazy_static;

mod assets;
mod constants;
mod render;

use actix_web::{http, server, App};
use constants::{APP_NAME, DEFAULT_HOST, DEFAULT_PORT, ENV_HOST, ENV_PATH, ENV_PORT};
use std::env;

fn create_app() -> App {
    App::new()
        .middleware(
            actix_web::middleware::DefaultHeaders::new()
                .header("content-security-policy", "block-all-mixed-content; upgrade-insecure-requests; sandbox allow-scripts allow-popups")
                .header("cache-control", "public, must-revalidate, max-age=86400")
                .header("feature-policy", "autoplay 'none';camera 'none';fullscreen 'none';geolocation 'none';microphone 'none';midi 'none';payment 'none';sync-xhr 'none';usb 'none';vr 'none'")
                .header("referrer-policy", "strict-origin")
                .header("strict-transport-security", "max-age=86400; includeSubDomains; preload")
                .header("x-content-type-options", "nosniff")
                .header("x-download-options", "noopen")
                .header("x-xss-protection", "1; mode=block")
        )
        .route("/", http::Method::GET, render::index)
        .route("/contact", http::Method::GET, render::contact)
        .route("/assets/{name}", http::Method::GET, assets::handle_request)
        .default_resource(|r| r.with(render::not_found))
}

fn get_socket_path() -> Option<String> {
    let path = env::var_os(ENV_PATH);

    if path.is_none() {
        return None
    }

    let path_os = path.unwrap();
    let path_str = path_os.to_str();

    if path_str.is_none() {
        return None
    }

    Some(String::from(path_str.unwrap()))
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
    let http_server = server::new(|| create_app());
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
