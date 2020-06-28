use horrorshow::TemplateBuffer;

pub fn render_not_found(tplbuf: &mut TemplateBuffer<'_>) {
    tplbuf
        << html! {
            h1: "404";
            h2: "There isn't something here i promise.";
        };
}
