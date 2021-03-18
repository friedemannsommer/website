use crate::{constants, site_request::handle_site_request};
use lambda_http::{
    http::{Method, StatusCode},
    lambda_runtime::Error,
    Body, Request, Response,
};

pub fn simple_router(request: Request) -> Result<Response<Body>, Error> {
    if request.method() != Method::GET {
        return Response::builder()
            .header("content-type", "text/plain; charset=utf-8")
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::from("ERR_METHOD_NOT_ALLOWED"))
            .map_err(crate::util::map_http_err);
    }

    match request.uri().path() {
        "/" | "" => handle_site_request(constants::Site::Index),
        "/contact" | "/contact/" => handle_site_request(constants::Site::Contact),
        _ => handle_site_request(constants::Site::NotFound),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::templates::render_site;
    use lambda_http::http::HeaderValue;

    const SITE_URI_LIST: [&str; 3] = ["/", "/contact", "/not-found"];

    #[test]
    fn handle_invalid_request_method() {
        for method in ["POST", "PATCH", "DELETE", "OPTIONS"].iter() {
            let response = simple_router(
                lambda_http::http::Request::builder()
                    .method(*method)
                    .uri("/")
                    .body(Body::Empty)
                    .unwrap(),
            )
            .unwrap();

            assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
            assert_eq!(*response.body(), Body::from("ERR_METHOD_NOT_ALLOWED"));
        }
    }

    #[test]
    fn handles_index() {
        let response = fetch_get_response("/");

        assert_eq!(response.status(), 200);
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "text/html; charset=utf-8"
        );
        assert_eq!(
            *response.body(),
            Body::from(render_site(&constants::Site::Index).unwrap())
        );
    }

    #[test]
    fn handles_contact() {
        for path in ["/contact", "/contact/"].iter() {
            let response = fetch_get_response(*path);

            assert_eq!(response.status(), 200);
            assert_eq!(
                response.headers().get("content-type").unwrap(),
                "text/html; charset=utf-8"
            );
            assert_eq!(
                *response.body(),
                Body::from(render_site(&constants::Site::Contact).unwrap())
            );
        }
    }

    #[test]
    fn handles_not_found() {
        let response = fetch_get_response("/404");

        assert_eq!(response.status(), 404);
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "text/html; charset=utf-8"
        );
        assert_eq!(
            *response.body(),
            Body::from(render_site(&constants::Site::NotFound).unwrap())
        );
    }

    #[test]
    fn frame_deny() {
        for path in SITE_URI_LIST.iter() {
            let response = fetch_get_response(*path);

            assert_eq!(
                response.headers().get("x-frame-options").unwrap(),
                HeaderValue::from_static("deny")
            );
        }
    }

    #[test]
    fn xss_protection() {
        for path in SITE_URI_LIST.iter() {
            let response = fetch_get_response(*path);

            assert_eq!(
                response.headers().get("x-xss-protection").unwrap(),
                HeaderValue::from_static("1; mode=block")
            );
        }
    }

    #[test]
    fn feature_policy() {
        for path in SITE_URI_LIST.iter() {
            let response = fetch_get_response(*path);
            let fp_parts: Vec<&str> = response
                .headers()
                .get("feature-policy")
                .unwrap()
                .to_str()
                .unwrap()
                .split(';')
                .collect();

            for part in fp_parts {
                assert_eq!(part.contains("'none'"), true);
            }
        }
    }

    #[test]
    fn referrer_policy() {
        for path in SITE_URI_LIST.iter() {
            let response = fetch_get_response(*path);

            assert_eq!(
                response.headers().get("referrer-policy").unwrap(),
                "strict-origin"
            );
        }
    }

    #[test]
    fn content_security_policy() {
        for path in SITE_URI_LIST.iter() {
            let response = fetch_get_response(*path);
            let csp_value = response
                .headers()
                .get("content-security-policy")
                .unwrap()
                .to_str()
                .unwrap();

            assert_eq!(csp_value.contains("block-all-mixed-content"), true);
            assert_eq!(csp_value.contains("upgrade-insecure-requests"), true);
            assert_eq!(csp_value.contains("base-uri 'none'"), true);
            assert_eq!(csp_value.contains("default-src 'none'"), true);
            assert_eq!(csp_value.contains("script-src 'self'"), true);
            assert_eq!(
                csp_value.contains("style-src")
                    && csp_value
                        .contains((format!("sha256-{}", *constants::STYLESHEET_HASH)).as_str()),
                true
            );
            assert_eq!(csp_value.contains("allow-popups"), true);
            assert_eq!(csp_value.contains("allow-popups-to-escape-sandbox"), true);
        }
    }

    fn fetch_get_response(path: &str) -> Response<Body> {
        simple_router(
            lambda_http::http::Request::builder()
                .method("GET")
                .uri(path)
                .body(Body::Empty)
                .unwrap(),
        )
        .unwrap()
    }
}
