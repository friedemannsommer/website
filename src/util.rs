use crate::constants;
use crypto::{digest::Digest, sha2::Sha256};
use html_minifier::HTMLMinifier;
use serde_json::Value;
use data_encoding::BASE64;

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
    let mut output= vec![0; hasher.output_bytes()];
    hasher.result(&mut output);
    BASE64.encode(&output)
}
