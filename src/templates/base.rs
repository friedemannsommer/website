use crate::constants::{Site, MENU_ENTRIES, SITE_DESCRIPTION, SITE_TITLE, STYLESHEET};
use horrorshow::{helper::doctype, Raw, Render};

#[allow(clippy::needless_lifetimes)]
pub fn render_base<'a>(site: &'a Site) -> impl Render + 'a {
    owned_html! {
        : doctype::HTML;
        html(lang="en") {
            head {
                title: SITE_TITLE;
                meta(name="description", content=SITE_DESCRIPTION);
                meta(name="viewport", content="width=device-width, initial-scale=1");
                style: Raw(STYLESHEET);
            }
            body(class="flexbox flexbox-horizontal") {
                |tplbuf| tplbuf << render_menu();
                section {
                    div(class="content") {
                        |tplbuf| match site {
                            Site::Contact => crate::templates::render_contact(tplbuf),
                            Site::Index => crate::templates::render_index(tplbuf),
                            _ => crate::templates::render_not_found(tplbuf)
                        }
                    }
                }
                |tplbuf| tplbuf << render_footer();
            }
        }
    }
}

fn render_menu() -> impl Render {
    html! {
        header {
            ul(class="flexbox flexbox-horizontal") {
                @ for entry in MENU_ENTRIES.iter() {
                    li {
                        a(
                            href=&entry.url,
                            title=&entry.name,
                            target="_self"
                        ): &entry.name;
                    }
                }
            }
        }
    }
}

fn render_footer() -> impl Render {
    html! {
        footer {
            ul(class="flexbox flexbox-horizontal") {
                li {
                    a(
                        target="_blank",
                        rel="noopener",
                        href="https://github.com/friedemannsommer/website"
                    ): "Source code for this site";
                }
            }
        }
    }
}
