use handlebars::Handlebars;
use html_minifier::HTMLMinifier;

// generic meta
pub const ENV_HOST: &str = "HOST";
pub const ENV_PORT: &str = "PORT";
pub const ENV_PATH: &str = "SOCKET_PATH";
pub const APP_NAME: &str = "website";
// defaults
pub const DEFAULT_PORT: &str = "8080";
pub const DEFAULT_HOST: &str = "127.0.0.1";
// http
pub const CONTENT_TYPE_HTML: &str = "text/html; charset=utf-8";
// template struct
pub struct TemplateCache {
    pub index: Box<String>,
    pub contact: Box<String>,
    pub not_found: Box<String>,
}

fn minify_html(html: String) -> Result<String, &'static str> {
    let mut html_minifier = HTMLMinifier::new();

    html_minifier.digest(html)?;
    Ok(html_minifier.get_html())
}

lazy_static! {
    pub static ref TEMPLATE_CACHE: TemplateCache = {
        let mut handlebars = Handlebars::new();
        let json_data: serde_json::Value = json!({
            "title": "Friedemann Sommer",
            "description": "Friedemann Sommer Software Engineer at advanced STORE",
            "style": include_str!("./assets/styles.css"),
            "entries": [
                {
                    "url": "/",
                    "name": "Home"
                },
                {
                    "url": "/contact",
                    "name": "Contact"
                }
            ],
            "links": [
                {
                    "alias": "GitHub",
                    "url": "https://github.com/friedemannsommer",
                    "name": "@friedemannsommer"
                },
                {
                    "alias": "GitLab",
                    "url": "https://gitlab.com/friedemannsommer",
                    "name": "@friedemannsommer"
                },
                {
                    "alias": "Twitter",
                    "url": "https://twitter.com/free_some_mem",
                    "name": "@free_some_mem"
                },
                {
                    "alias": "LinkedIn",
                    "url": "https://www.linkedin.com/in/friedemann-sommer-0608b0130",
                    "name": "Friedemann Sommer"
                },
                {
                    "alias": "Mail",
                    "url": "mailto:contact@friedemannsommer.com",
                    "name": "contact@friedemannsommer.com"
                }
            ]
        });

        handlebars.set_strict_mode(true);

        handlebars
            .register_partial("base", include_str!("./templates/base.hbs"))
            .unwrap();
        handlebars
            .register_partial("footer", include_str!("./templates/footer.hbs"))
            .unwrap();
        handlebars
            .register_partial("head", include_str!("./templates/head.hbs"))
            .unwrap();
        handlebars
            .register_partial("menu", include_str!("./templates/menu.hbs"))
            .unwrap();

        TemplateCache {
            contact: Box::new(
                minify_html(
                    handlebars
                        .render_template(include_str!("./templates/contact.hbs"), &json_data)
                        .unwrap(),
                )
                .unwrap(),
            ),
            index: Box::new(
                minify_html(
                    handlebars
                        .render_template(include_str!("./templates/index.hbs"), &json_data)
                        .unwrap(),
                )
                .unwrap(),
            ),
            not_found: Box::new(
                minify_html(
                    handlebars
                        .render_template(include_str!("./templates/404.hbs"), &json_data)
                        .unwrap(),
                )
                .unwrap(),
            ),
        }
    };
}
