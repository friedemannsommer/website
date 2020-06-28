use crypto::{digest::Digest, sha2::Sha256};
use data_encoding::BASE64;
use lambda_http::http;
use lambda_runtime::error::HandlerError;

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
