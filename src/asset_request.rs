use crate::constants::{
    Asset, SOURCE_CODE_PRO_OTF, SOURCE_CODE_PRO_TTF, SOURCE_CODE_PRO_WOFF, SOURCE_CODE_PRO_WOFF2,
};
use lambda_http::{http::StatusCode, Body, Response};
use lambda_runtime::error::HandlerError;

pub fn handle_asset_request(asset: Asset) -> Result<Response<Body>, HandlerError> {
    Response::builder()
        .header(
            "content-type",
            match asset {
                Asset::SourceCodeProOtf => "font/otf",
                Asset::SourceCodeProTtf => "font/ttf",
                Asset::SourceCodeProWoff => "font/woff",
                Asset::SourceCodeProWoff2 => "font/woff2",
            },
        )
        .header("cache-control", "public, must-revalidate, max-age=86400")
        .header("x-content-type-options", "nosniff")
        .header("x-download-options", "noopen")
        .status(StatusCode::OK)
        .body(Body::from(match asset {
            Asset::SourceCodeProOtf => SOURCE_CODE_PRO_OTF,
            Asset::SourceCodeProTtf => SOURCE_CODE_PRO_TTF,
            Asset::SourceCodeProWoff => SOURCE_CODE_PRO_WOFF,
            Asset::SourceCodeProWoff2 => SOURCE_CODE_PRO_WOFF2,
        }))
        .map_err(crate::util::map_http_err)
}
