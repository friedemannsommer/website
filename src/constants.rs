use crate::util::get_sha256_hash;

pub enum Site {
    Index,
    Contact,
    NotFound,
}

pub struct LinkEntry {
    pub name: String,
    pub url: String,
    pub label: String,
}

pub struct MenuEntry {
    pub name: String,
    pub url: String,
}

pub const SITE_TITLE: &str = "Friedemann Sommer";
pub const SITE_DESCRIPTION: &str = "Friedemann Sommer Software Engineer at advanced STORE";
pub const STYLESHEET: &str = include_str!("./assets/styles.min.css");

lazy_static! {
    pub static ref MENU_ENTRIES: Vec<MenuEntry> = {
        vec![
            MenuEntry {
                name: String::from("Home"),
                url: String::from("/"),
            },
            MenuEntry {
                name: String::from("Contact"),
                url: String::from("/contact"),
            },
        ]
    };
    pub static ref LINK_ENTRIES: Vec<LinkEntry> = {
        vec![
            LinkEntry {
                name: String::from("GitHub"),
                url: String::from("https://github.com/friedemannsommer"),
                label: String::from("@friedemannsommer"),
            },
            LinkEntry {
                name: String::from("GitLab"),
                url: String::from("https://gitlab.com/friedemannsommer"),
                label: String::from("@friedemannsommer"),
            },
            LinkEntry {
                name: String::from("Twitter"),
                url: String::from("https://twitter.com/free_some_mem"),
                label: String::from("@free_some_mem"),
            },
            LinkEntry {
                name: String::from("LinkedIn"),
                url: String::from("https://www.linkedin.com/in/friedemann-sommer-0608b0130"),
                label: String::from("Friedemann Sommer"),
            },
            LinkEntry {
                name: String::from("Mail"),
                url: String::from("mailto:contact@friedemannsommer.com"),
                label: String::from("contact@friedemannsommer.com"),
            },
        ]
    };
    pub static ref STYLESHEET_HASH: String = get_sha256_hash(STYLESHEET);
}
