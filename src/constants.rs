use crate::util::{get_sha256_hash, render_template};
use handlebars::Handlebars;
use serde_json::json;

pub enum Site {
    Index,
    Contact,
    NotFound,
}

pub enum Asset {
    FontOtf,
    FontTtf,
    FontWoff,
    FontWoff2,
}

// template struct
pub struct TemplateCache {
    pub index: String,
    pub contact: String,
    pub not_found: String,
    pub style_sha256: String,
}

pub const SOURCE_CODE_PRO_OTF: &[u8] = include_bytes!("./assets/source-code-pro-regular.otf");
pub const SOURCE_CODE_PRO_TTF: &[u8] = include_bytes!("./assets/source-code-pro-regular.ttf");
pub const SOURCE_CODE_PRO_WOFF: &[u8] = include_bytes!("./assets/source-code-pro-regular.woff");
pub const SOURCE_CODE_PRO_WOFF2: &[u8] = include_bytes!("./assets/source-code-pro-regular.woff2");

lazy_static! {
    pub static ref TEMPLATE_ENGINE: Handlebars<'static> = {
        let mut handlebars = Handlebars::new();

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

        handlebars
    };
    pub static ref TEMPLATE_CACHE: TemplateCache = {
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

        // sadly this workaround is required to get the minified css
        let index_html =
            render_template(include_str!("./templates/index.hbs"), &json_data).unwrap();
        let style_start = index_html.find("<style>");
        let style_end = index_html.find("</style>");
        let style: &str = if style_start.is_some() && style_end.is_some() {
            let parts = index_html.split_at(style_start.unwrap() + 7);

            parts.1.split_at(style_end.unwrap() - parts.0.len()).0
        } else {
            ""
        };

        TemplateCache {
            contact: render_template(include_str!("./templates/contact.hbs"), &json_data).unwrap(),
            index: index_html.clone(),
            not_found: render_template(include_str!("./templates/404.hbs"), &json_data).unwrap(),
            style_sha256: get_sha256_hash(style),
        }
    };
}
