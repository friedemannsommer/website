use crate::constants::{
    Asset, SOURCE_CODE_PRO_OTF, SOURCE_CODE_PRO_TTF, SOURCE_CODE_PRO_WOFF, SOURCE_CODE_PRO_WOFF2,
};
use lambda_http::{
    http::{HeaderMap, HeaderValue, StatusCode},
    Body, Response,
};
use lambda_runtime::error::HandlerError;

fn build_forbidden_response() -> Result<Response<Body>, HandlerError> {
    Response::builder()
        .header("content-type", "text/plain; charset=utf-8")
        .status(StatusCode::FORBIDDEN)
        .body(Body::from("ERR_DIRECT_ACCESS"))
        .map_err(crate::util::map_http_err)
}

pub fn handle_asset_request(
    req_headers: &HeaderMap<HeaderValue>,
    asset: Asset,
) -> Result<Response<Body>, HandlerError> {
    if !crate::util::referrer_validation(req_headers) {
        return build_forbidden_response();
    }

    Response::builder()
        .header(
            "content-type",
            match asset {
                Asset::FontOtf => "font/otf",
                Asset::FontTtf => "font/ttf",
                Asset::FontWoff => "font/woff",
                Asset::FontWoff2 => "font/woff2",
            },
        )
        .header("cache-control", "public, must-revalidate, max-age=86400")
        .header("x-content-type-options", "nosniff")
        .header("x-download-options", "noopen")
        .status(StatusCode::OK)
        .body(Body::from(match asset {
            Asset::FontOtf => SOURCE_CODE_PRO_OTF,
            Asset::FontTtf => SOURCE_CODE_PRO_TTF,
            Asset::FontWoff => SOURCE_CODE_PRO_WOFF,
            Asset::FontWoff2 => SOURCE_CODE_PRO_WOFF2,
        }))
        .map_err(crate::util::map_http_err)
}
