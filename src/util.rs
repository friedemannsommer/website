use crate::constants;
use html_minifier::HTMLMinifier;
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
