use actix_web::{Error, HttpRequest, HttpResponse};
use constants;

/**
 * @todo(fs) implement asset file lookup (don't use static file serving)
 */
pub fn handle_request(req: HttpRequest) -> Result<HttpResponse, Error> {
    println!("{}", req.path());
    Ok(HttpResponse::Ok()
        .content_type(constants::CONTENT_TYPE_TEXT)
        .body(""))
}
