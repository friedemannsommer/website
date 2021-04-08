use crate::constants::LINK_ENTRIES;
use horrorshow::TemplateBuffer;

pub fn render_contact(tplbuf: &mut TemplateBuffer<'_>) {
    tplbuf
        << html! {
            h1: "Contact";
            ul {
                @ for link in LINK_ENTRIES.iter() {
                    li {
                        : format!("{}: ", &link.name);
                        @ if let Some(url) = &link.url {
                            a(
                                href=url,
                                target="_blank",
                                rel="noopener"
                            ): &link.label;
                        } else {
                            : &link.label
                        }
                    }
                }
            }
        };
}
