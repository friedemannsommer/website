use actix_web::{Error, HttpRequest, HttpResponse};
use constants;
use tera;

const ERROR_MESSAGE: &str = "Something went wrong... :(";

pub fn index(_req: HttpRequest) -> Result<HttpResponse, Error> {
    let context = tera::Context::new();
    let html = constants::TERA.render(constants::TEMPLATE_INDEX, &context);

    if html.is_ok() {
        return Ok(HttpResponse::Ok()
            .content_type(constants::CONTENT_TYPE_HTML)
            .body(html.unwrap()));
    }

    Ok(HttpResponse::InternalServerError()
        .content_type(constants::CONTENT_TYPE_TEXT)
        .body(ERROR_MESSAGE))
}

pub fn contact(_req: HttpRequest) -> Result<HttpResponse, Error> {
    let context = tera::Context::new();
    let html = constants::TERA.render(constants::TEMPLATE_CONTACT, &context);

    if html.is_ok() {
        return Ok(HttpResponse::Ok()
            .content_type(constants::CONTENT_TYPE_HTML)
            .body(html.unwrap()));
    }

    Ok(HttpResponse::InternalServerError()
        .content_type(constants::CONTENT_TYPE_TEXT)
        .body(ERROR_MESSAGE))
}

pub fn not_found(_req: HttpRequest) -> Result<HttpResponse, Error> {
    let context = tera::Context::new();
    let html = constants::TERA.render(constants::TEMPLATE_NOT_FOUND, &context);

    if html.is_ok() {
        return Ok(HttpResponse::NotFound()
            .content_type(constants::CONTENT_TYPE_HTML)
            .body(html.unwrap()));
    }

    Ok(HttpResponse::InternalServerError()
        .content_type(constants::CONTENT_TYPE_TEXT)
        .body(ERROR_MESSAGE))
}
