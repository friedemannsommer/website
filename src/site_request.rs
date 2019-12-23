use crate::constants::{Site, TEMPLATE_CACHE};
use lambda_http::{http::StatusCode, Body, Response};
use lambda_runtime::error::HandlerError;

pub fn handle_site_request(site: Site) -> Result<Response<Body>, HandlerError> {
    Response::builder()
    .header(
        "content-security-policy",
        format!("block-all-mixed-content; upgrade-insecure-requests; sandbox allow-scripts allow-popups allow-popups-to-escape-sandbox; frame-ancestors 'none'; form-action 'none'; base-uri 'none'; default-src 'none'; script-src 'self'; style-src 'unsafe-inline' 'self' 'sha256-{}'; font-src 'self'", &TEMPLATE_CACHE.style_sha256)
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
    .header("content-type","text/html; charset=utf-8")
    .status(
        match site {
            Site::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::OK
        }
    )
    .body(
        Body::from(
            match site {
                Site::Contact => TEMPLATE_CACHE.contact.to_string(),
                Site::Index => TEMPLATE_CACHE.index.to_string(),
                _ => TEMPLATE_CACHE.not_found.to_string()
            }
        )
    ).map_err(crate::util::map_http_err)
}
