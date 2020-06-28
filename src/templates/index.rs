use crate::constants::SITE_TITLE;
use horrorshow::TemplateBuffer;

pub fn render_index(tplbuf: &mut TemplateBuffer<'_>) {
    tplbuf
        << html! {
            h1: SITE_TITLE;
            h2 {
                : "Software Engineer ";
                a(
                    href="https://www.advanced-store.com",
                    title="advancedSTORE",
                    target="_blank",
                    rel="noopener"
                ): "@advancedSTORE";
            }
        };
}
