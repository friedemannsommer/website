use crate::{
    constants::{Site, STYLESHEET_HASH},
    templates::render_site,
};
use lambda_http::{http::StatusCode, lambda_runtime::Error, Body, Response};

pub fn handle_site_request(site: Site) -> Result<Response<Body>, Error> {
    let response = Response::builder()
    .header(
        "content-security-policy",
        format!("block-all-mixed-content; upgrade-insecure-requests; sandbox allow-scripts allow-popups allow-popups-to-escape-sandbox; frame-ancestors 'none'; form-action 'none'; base-uri 'none'; default-src 'none'; script-src 'self'; style-src 'unsafe-inline' 'self' 'sha256-{}'; font-src 'self'", *STYLESHEET_HASH)
    )
    .header(
        "cache-control",
        "public, must-revalidate, max-age=86400",        
    )
    .header(
        "feature-policy",
        "autoplay 'none';camera 'none';fullscreen 'none';geolocation 'none';microphone 'none';midi 'none';payment 'none';sync-xhr 'none';usb 'none';vr 'none'"
    )
    .header(
        "referrer-policy", "strict-origin"
    )
    .header("x-content-type-options","nosniff")
    .header("x-download-options", "noopen")
    .header("x-xss-protection","1; mode=block")
    .header("x-frame-options", "deny")
    .header("content-type","text/html; charset=utf-8");

    match render_site(&site) {
        Ok(html) => response
            .status(match site {
                Site::NotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::OK,
            })
            .body(Body::Text(html)),
        Err(err) => {
            eprintln!("{:?}", err);
            response
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("INTERNAL_RENDER_ERROR"))
        }
    }
    .map_err(crate::util::map_http_err)
}
