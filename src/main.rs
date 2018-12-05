extern crate actix;
extern crate actix_web;

use actix_web::{http, server, App, Error, HttpRequest, HttpResponse};
use std::env;

// app meta constants
const APP_NAME: &str = "website";
const ENV_HOST: &str = "HOST";
const ENV_PORT: &str = "PORT";
const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "8080";
// generic constants
const CONTENT_TYPE_TEXT: &str = "text/plain; charset=utf-8";
const MESSAGE: &str = "Hello from Actix web a Rust web framework.";

fn index(_req: &HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type(CONTENT_TYPE_TEXT)
        .body(MESSAGE))
}

fn main() {
    let sys = actix::System::new(APP_NAME);
    let address = env::var(ENV_HOST)
        .unwrap_or(DEFAULT_HOST.to_string())
        .to_owned()
        + ":"
        + &env::var(ENV_PORT).unwrap_or(DEFAULT_PORT.to_string());

    server::new(|| App::new().resource("/", |r| r.method(http::Method::GET).f(index)))
        .bind(&address)
        .unwrap()
        .start();

    println!("Started http server: {}", &address);
    sys.run();
}
