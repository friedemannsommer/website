use crate::constants;
use lambda_http::http::{header::HeaderValue, Method, StatusCode};
use lambda_http::{Body, Request, Response};
use lambda_runtime::{error::HandlerError, Context};

pub fn simple_router(request: Request, _: Context) -> Result<Response<Body>, HandlerError> {
    let mut response = Response::new(Body::from(""));
    let headers = response.headers_mut();

    headers.insert("content-type", HeaderValue::from_static("text/plain; charset=utf-8"));
    headers.insert("content-security-policy", HeaderValue::from_static("block-all-mixed-content; upgrade-insecure-requests; sandbox allow-scripts allow-popups; frame-ancestors 'none'; form-action 'none'; default-src https: 'unsafe-inline'; object-src 'none'"));
    headers.insert(
        "cache-control",
        HeaderValue::from_static("public, must-revalidate, max-age=86400"),
    );
    headers.insert("feature-policy", HeaderValue::from_static("autoplay 'none';camera 'none';fullscreen 'none';geolocation 'none';microphone 'none';midi 'none';payment 'none';sync-xhr 'none';usb 'none';vr 'none'"));
    headers.insert("referrer-policy", HeaderValue::from_static("strict-origin"));
    headers.insert(
        "strict-transport-security",
        HeaderValue::from_static("max-age=86400; includeSubDomains; preload"),
    );
    headers.insert(
        "x-content-type-options",
        HeaderValue::from_static("nosniff"),
    );
    headers.insert("x-download-options", HeaderValue::from_static("noopen"));
    headers.insert(
        "x-xss-protection",
        HeaderValue::from_static("1; mode=block"),
    );
    headers.insert("x-frame-options", HeaderValue::from_static("deny"));

    if request.method() != Method::GET {
        *response.status_mut() = StatusCode::METHOD_NOT_ALLOWED;
        *response.body_mut() = Body::from("ERR_METHOD_NOT_ALLOWED");

        return Ok(response);
    }

    headers.insert(
        "content-type",
        HeaderValue::from_static("text/html; charset=utf-8"),
    );

    *response.status_mut() = StatusCode::OK;
    *response.body_mut() = Body::from(match request.uri().path() {
        "/" | "" => &**constants::TEMPLATE_CACHE.index,
        "/contact" | "/contact/" => &**constants::TEMPLATE_CACHE.contact,
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
            &**constants::TEMPLATE_CACHE.not_found
        }
    });

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_index() {
        let request = Request::default();
        let expected = Body::from(&**constants::TEMPLATE_CACHE.index);
        let response = simple_router(request, Context::default());

        assert_eq!(&*response.unwrap().body(), &expected)
    }

    #[test]
    fn handles_contact() {
        let mut request = Request::default();
        let expected = Body::from(&**constants::TEMPLATE_CACHE.contact);

        *request.uri_mut() = lambda_http::http::Uri::from_static("/contact");

        let response = simple_router(request, Context::default());

        assert_eq!(&*response.unwrap().body(), &expected)
    }

    #[test]
    fn handles_not_found() {
        let mut request = Request::default();
        let expected = Body::from(&**constants::TEMPLATE_CACHE.not_found);

        *request.uri_mut() = lambda_http::http::Uri::from_static("/this-route-doesnt-exist");

        let response = simple_router(request, Context::default());

        assert_eq!(&*response.unwrap().body(), &expected)
    }

    #[test]
    fn force_download() {
        let request = Request::default();
        let response = simple_router(request, Context::default());

        assert_eq!(
            response
                .unwrap()
                .headers()
                .get("x-download-options")
                .unwrap(),
            HeaderValue::from_static("noopen")
        )
    }

    #[test]
    fn frame_deny() {
        let request = Request::default();
        let response = simple_router(request, Context::default());

        assert_eq!(
            response.unwrap().headers().get("x-frame-options").unwrap(),
            HeaderValue::from_static("deny")
        )
    }

    #[test]
    fn xss_protection() {
        let request = Request::default();
        let response = simple_router(request, Context::default());

        assert_eq!(
            response.unwrap().headers().get("x-xss-protection").unwrap(),
            HeaderValue::from_static("1; mode=block")
        )
    }

    #[test]
    fn content_type_detection() {
        let request = Request::default();
        let response = simple_router(request, Context::default());

        assert_eq!(
            response
                .unwrap()
                .headers()
                .get("x-content-type-options")
                .unwrap(),
            HeaderValue::from_static("nosniff")
        )
    }
}
