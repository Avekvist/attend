use rocket_contrib::templates::Template;
use crate::models::teacher::TeacherCookie;
use rocket::response::Redirect;
use std::collections::HashMap;
use crate::AttendDatabase;
use super::Context;

#[get("/tags")]
pub fn authenticated(conn: AttendDatabase, _teacher: TeacherCookie) -> Template {
    use diesel::prelude::*;
    use crate::schema::tag::dsl as tag_dsl;
    use crate::schema::attendee::dsl as attendee_dsl;
    use crate::models::{ tag::*, attendee::* };

    let tag_results = tag_dsl::tag
        .load::<Tag>(&*conn)
        .expect("Couldn't access tags");

    let attendee_results = &attendee_dsl::attendee
        .load::<Attendee>(&*conn)
        .expect("Couldn't access attendees");
    
    let results: Vec<_> = tag_results.into_iter().filter(|tag| {
        let mut success = true;

        attendee_results.into_iter().for_each(|attendee| {
            if attendee.tag_id == tag.tag_id {
                success = false;
            }
        });

        success
    }).collect();

    let results: Vec<_> = results.into_iter().map(|tag| tag.tag_id).collect();

    let mut data = HashMap::new();
    data.insert("tags", results);

    let context = Context {
        logged_in: true,
        data,
    };

    Template::render("tags/tags", context)
}

#[get("/tags", rank = 2)]
pub fn unauthenticated() -> Redirect {
    Redirect::to("/login")
}

#[get("/tags/<id>")]
pub fn list_authenticated(_conn: AttendDatabase, _teacher: TeacherCookie, id: u16) -> Template {
    let mut data = HashMap::new();
    data.insert("tag", id);
    
    let context = Context {
        logged_in: true,
        data,
    };

    Template::render("tags/tag", context)
}

#[get("/tags/<_id>", rank = 2)]
pub fn list_unauthenticated(_id: u16) -> Redirect {
    Redirect::to("/login")
}
