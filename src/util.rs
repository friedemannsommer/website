use crate::constants;
use crypto::{digest::Digest, sha2::Sha256};
use data_encoding::BASE64;
use html_minifier::HTMLMinifier;
use lambda_http::http;
use lambda_runtime::error::HandlerError;
use serde_json::Value;

pub fn minify_html(html: String) -> Result<String, &'static str> {
    let mut html_minifier = HTMLMinifier::new();

    html_minifier.digest(html)?;
    Ok(html_minifier.get_html())
}

pub fn render_template(template: &str, data: &Value) -> Result<String, &'static str> {
    minify_html(
        constants::TEMPLATE_ENGINE
            .render_template(template, data)
            .unwrap(),
    )
}

pub fn get_sha256_hash(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(value);
    let mut output = vec![0; hasher.output_bytes()];
    hasher.result(&mut output);
    BASE64.encode(&output)
}

pub fn map_http_err(err: http::Error) -> HandlerError {
    HandlerError::from(err.to_string().as_str())
}

pub fn referrer_validation(req_headers: &http::HeaderMap<http::HeaderValue>) -> bool {
    let referrer = req_headers.get("referer");
    let host = req_headers.get("host");

    if referrer.is_none() || host.is_none() {
        return false;
    }

    let referrer_str = referrer.unwrap().to_str();
    let host_str = host.unwrap().to_str();

    if referrer_str.is_err() || host_str.is_err() {
        return false;
    }

    referrer_str
        .unwrap()
        .starts_with(format!("https://{}", host_str.unwrap()).as_str())
}
