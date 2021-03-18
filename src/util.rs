use crypto::{digest::Digest, sha2::Sha256};
use lambda_http::{http, lambda_runtime::Error};

pub fn get_sha256_hash(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(value);
    let mut output = vec![0; hasher.output_bytes()];
    hasher.result(&mut output);
    base64::encode(&output)
}

pub fn map_http_err(err: http::Error) -> Error {
    Error::from(err.to_string().as_str())
}
