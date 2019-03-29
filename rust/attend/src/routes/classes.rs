use rocket_contrib::templates::Template;
use crate::models::teacher::TeacherCookie;
use rocket::response::Redirect;
use crate::AttendDatabase;
use super::Context;

#[get("/classes")]
pub fn authenticated(_conn: AttendDatabase, _teacher: TeacherCookie) -> Template {
    let context = Context::<Option<u8>> {
        logged_in: true,
        data: None,
    };

    Template::render("classes/classes", context)
}

#[get("/classes", rank = 2)]
pub fn unauthenticated() -> Redirect {
    Redirect::to("/login")
}

#[get("/classes/<_id>")]
pub fn list_authenticated(_conn: AttendDatabase, _teacher: TeacherCookie, _id: u16) -> Template {
    let context = Context::<Option<u8>> {
        logged_in: true,
        data: None,
    };

    Template::render("classes/class", context)
}

#[get("/classes/<_id>", rank = 2)]
pub fn list_unauthenticated(_id: u16) -> Redirect {
    Redirect::to("/login")
}
