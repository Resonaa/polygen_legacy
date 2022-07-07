mod announcement;
mod post;

use crate::concat_vec;

pub fn routes() -> Vec<rocket::Route> {
    concat_vec![announcement::routes(), post::routes()]
}
