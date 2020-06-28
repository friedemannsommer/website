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
                        a(
                            href=&link.url,
                            target="_blank",
                            rel="noopener"
                        ): &link.label;
                    }
                }
            }
        };
}
