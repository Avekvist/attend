use rocket_contrib::templates::Template;
use crate::models::teacher::TeacherCookie;
use super::Context;

#[get("/support")]
pub fn authenticated(_teacher: TeacherCookie) -> Template {
    let context = Context::<Option<u8>> {
        logged_in: true,
        data: None,
    };

    Template::render("support", context)
}

#[get("/support", rank = 2)]
pub fn unauthenticated() -> Template {
    let context = Context::<Option<u8>> {
        logged_in: false,
        data: None,
    };

    Template::render("support", context)
}
