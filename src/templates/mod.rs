use crate::constants::Site;
use horrorshow::{Error, Template};

mod base;
mod contact;
mod index;
mod not_found;

pub use contact::render_contact;
pub use index::render_index;
pub use not_found::render_not_found;

pub fn render_site(site: &Site) -> Result<String, Error> {
    base::render_base(site).into_string()
}
