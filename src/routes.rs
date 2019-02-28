use actix_web::{Error, HttpRequest, HttpResponse};
use constants;

pub fn index(_req: HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type(constants::CONTENT_TYPE_HTML)
        .body(&*constants::TEMPLATE_CACHE.index))
}

pub fn contact(_req: HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type(constants::CONTENT_TYPE_HTML)
        .body(&*constants::TEMPLATE_CACHE.contact))
}

pub fn not_found(_req: HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type(constants::CONTENT_TYPE_HTML)
        .body(&*constants::TEMPLATE_CACHE.not_found))
}
