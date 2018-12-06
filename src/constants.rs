// generic meta
pub const ENV_HOST: &str = "HOST";
pub const ENV_PORT: &str = "PORT";
pub const APP_NAME: &str = "website";
// defaults
pub const DEFAULT_PORT: &str = "8080";
pub const DEFAULT_HOST: &str = "127.0.0.1";
// http
pub const CONTENT_TYPE_TEXT: &str = "text/plain; charset=utf-8";
pub const CONTENT_TYPE_HTML: &str = "text/html; charset=utf-8";
// templates
pub const TEMPLATE_BASE: &str = "base";
pub const TEMPLATE_INDEX: &str = "index";
pub const TEMPLATE_CONTACT: &str = "contact";
pub const TEMPLATE_NOT_FOUND: &str = "404";

lazy_static! {
    pub static ref TERA: tera::Tera = {
        let mut tera = tera::Tera::default();
        let res = tera.add_raw_templates(vec![
            (TEMPLATE_BASE, include_str!("./templates/base.html.tera")),
            (TEMPLATE_INDEX, include_str!("./templates/index.html.tera")),
            (
                TEMPLATE_CONTACT,
                include_str!("./templates/contact.html.tera"),
            ),
            (
                TEMPLATE_NOT_FOUND,
                include_str!("./templates/404.html.tera"),
            ),
        ]);

        if res.is_err() {
            panic!(res.unwrap());
        }

        tera
    };
}
