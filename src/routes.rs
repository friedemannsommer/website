use crate::asset_request::handle_asset_request;
use crate::constants;
use crate::site_request::handle_site_request;
use lambda_http::http::{Method, StatusCode};
use lambda_http::{Body, Request, Response};
use lambda_runtime::{error::HandlerError, Context};

pub fn simple_router(request: Request, _: Context) -> Result<Response<Body>, HandlerError> {
    if request.method() != Method::GET {
        return Response::builder()
            .header("content-type", "text/plain; charset=utf-8")
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::from("ERR_METHOD_NOT_ALLOWED"))
            .map_err(crate::util::map_http_err);
    }

    let headers = request.headers();

    match request.uri().path() {
        "/" | "" => handle_site_request(constants::Site::Index),
        "/contact" | "/contact/" => handle_site_request(constants::Site::Contact),
        "/source-code-pro-regular.woff2" => {
            handle_asset_request(headers, constants::Asset::FontWoff2)
        }
        "/source-code-pro-regular.woff" => {
            handle_asset_request(headers, constants::Asset::FontWoff)
        }
        "/source-code-pro-regular.otf" => handle_asset_request(headers, constants::Asset::FontOtf),
        "/source-code-pro-regular.ttf" => handle_asset_request(headers, constants::Asset::FontTtf),
        _ => handle_site_request(constants::Site::NotFound),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::http::HeaderValue;

    const SITE_URI_LIST: [&str; 3] = ["/", "/contact", "/not-found"];
    const ASSET_URI_LIST: [&str; 4] = [
        "/source-code-pro-regular.woff2",
        "/source-code-pro-regular.woff",
        "/source-code-pro-regular.otf",
        "/source-code-pro-regular.ttf",
    ];

    #[test]
    fn handle_invalid_request_method() {
        for method in ["POST", "PATCH", "DELETE", "OPTIONS"].iter() {
            let response = simple_router(
                lambda_http::http::Request::builder()
                    .method(*method)
                    .uri("/")
                    .body(Body::Empty)
                    .unwrap(),
                Context::default(),
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
            Body::from(&*constants::TEMPLATE_CACHE.index)
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
                Body::from(&*constants::TEMPLATE_CACHE.contact)
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
            Body::from(&*constants::TEMPLATE_CACHE.not_found)
        );
    }

    #[test]
    fn force_download() {
        for path in ASSET_URI_LIST.iter() {
            let response = fetch_get_response_with_cors(*path);

            assert_eq!(
                response.headers().get("x-download-options").unwrap(),
                HeaderValue::from_static("noopen")
            );
        }
    }

    #[test]
    fn content_type_detection() {
        for path in ASSET_URI_LIST.iter() {
            let response = fetch_get_response_with_cors(*path);

            assert_eq!(
                response.headers().get("x-content-type-options").unwrap(),
                HeaderValue::from_static("nosniff")
            );
        }
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
                .split(";")
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
                    && csp_value.contains(
                        (format!("sha256-{}", constants::TEMPLATE_CACHE.style_sha256)).as_str()
                    ),
                true
            );
            assert_eq!(csp_value.contains("allow-popups"), true);
            assert_eq!(csp_value.contains("allow-popups-to-escape-sandbox"), true);
        }
    }

    #[test]
    fn asset_cache_control() {
        for path in ASSET_URI_LIST.iter() {
            let response = fetch_get_response_with_cors(*path);

            assert_eq!(
                response.headers().get("cache-control").unwrap(),
                "public, must-revalidate, max-age=86400"
            );
        }
    }

    #[test]
    fn asset_font_otf() {
        let response = fetch_get_response_with_cors("/source-code-pro-regular.otf");

        assert_eq!(response.status(), 200);
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "application/x-font-opentype"
        );
        assert_eq!(
            *response.body(),
            Body::Binary(Vec::from(constants::SOURCE_CODE_PRO_OTF))
        );
    }

    #[test]
    fn asset_font_ttf() {
        let response = fetch_get_response_with_cors("/source-code-pro-regular.ttf");

        assert_eq!(response.status(), 200);
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "application/x-font-ttf"
        );
        assert_eq!(
            *response.body(),
            Body::Binary(Vec::from(constants::SOURCE_CODE_PRO_TTF))
        );
    }

    #[test]
    fn asset_font_woff() {
        let response = fetch_get_response_with_cors("/source-code-pro-regular.woff");

        assert_eq!(response.status(), 200);
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "application/font-woff"
        );
        assert_eq!(
            *response.body(),
            Body::Binary(Vec::from(constants::SOURCE_CODE_PRO_WOFF))
        );
    }

    #[test]
    fn asset_font_woff2() {
        let response = fetch_get_response_with_cors("/source-code-pro-regular.woff2");

        assert_eq!(response.status(), 200);
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "application/font-woff2"
        );
        assert_eq!(
            *response.body(),
            Body::Binary(Vec::from(constants::SOURCE_CODE_PRO_WOFF2))
        );
    }

    #[test]
    fn asset_font_otf_direct_access() {
        let response = fetch_get_response("/source-code-pro-regular.otf");

        assert_eq!(response.status(), 403);
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "text/plain; charset=utf-8"
        );
        assert_eq!(*response.body(), Body::from("ERR_DIRECT_ACCESS"));
    }

    #[test]
    fn asset_font_ttf_direct_access() {
        let response = fetch_get_response("/source-code-pro-regular.ttf");

        assert_eq!(response.status(), 403);
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "text/plain; charset=utf-8"
        );
        assert_eq!(*response.body(), Body::from("ERR_DIRECT_ACCESS"));
    }

    #[test]
    fn asset_font_woff_direct_access() {
        let response = fetch_get_response("/source-code-pro-regular.woff");

        assert_eq!(response.status(), 403);
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "text/plain; charset=utf-8"
        );
        assert_eq!(*response.body(), Body::from("ERR_DIRECT_ACCESS"));
    }

    #[test]
    fn asset_font_woff2_direct_access() {
        let response = fetch_get_response("/source-code-pro-regular.woff2");

        assert_eq!(response.status(), 403);
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "text/plain; charset=utf-8"
        );
        assert_eq!(*response.body(), Body::from("ERR_DIRECT_ACCESS"));
    }

    fn fetch_get_response(path: &str) -> Response<Body> {
        simple_router(
            lambda_http::http::Request::builder()
                .method("GET")
                .uri(path)
                .body(Body::Empty)
                .unwrap(),
            Context::default(),
        )
        .unwrap()
    }

    fn fetch_get_response_with_cors(path: &str) -> Response<Body> {
        simple_router(
            lambda_http::http::Request::builder()
                .method("GET")
                .header("host", "www.example.com")
                .header("origin", "https://www.example.com")
                .header("referer", "https://www.example.com/")
                .uri(path)
                .body(Body::Empty)
                .unwrap(),
            Context::default(),
        )
        .unwrap()
    }
}
