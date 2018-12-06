use actix_web::{Error, HttpResponse, Path};
use constants;
use std::collections::HashMap;

lazy_static! {
    static ref HASHMAP: HashMap<&'static str, (&'static str, &'static str)> = {
        let mut m = HashMap::new();
        m.insert(
            "styles.css",
            (
                constants::CONTENT_TYPE_CSS,
                include_str!("./assets/styles.css"),
            ),
        );
        m
    };
}

pub fn handle_request(param: Path<(String)>) -> Result<HttpResponse, Error> {
    let filename = param.to_string();
    let entry = HASHMAP.get(&*filename);

    if entry.is_none() {
        return Ok(HttpResponse::NotFound()
            .content_type(constants::CONTENT_TYPE_TEXT)
            .body(format!("File \"{}\" not found", filename)));
    }

    let file = entry.unwrap();

    Ok(HttpResponse::Ok().content_type(file.0).body(file.1))
}
