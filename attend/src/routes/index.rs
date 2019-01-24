use rocket_contrib::templates::Template;
use crate::models::teacher::TeacherCookie;
use rocket::response::Redirect;
use std::collections::HashMap;
use crate::AttendDatabase;
use super::Context;

#[derive(Serialize)]
pub struct ClassData {
    pub class_name: String,
    pub absent: Vec<crate::models::attendee::Attendee>,
    pub attending: Vec<crate::models::attendee::Attendee>
}

#[get("/")]
pub fn authenticated(conn: AttendDatabase, teacher: TeacherCookie) -> Template {
    use crate::schema::attendeeclassbridge::dsl as acb_dsl;
    use crate::schema::teacherclassbridge::dsl as tcb_dsl;
    use crate::schema::class::dsl as class_dsl;
    use crate::models::attendeeclassbridge::*;
    use crate::models::teacherclassbridge::*;
    use crate::models::class::*;
    use diesel::prelude::*;

    let datetime = chrono::prelude::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let _date = &datetime.split(' ').collect::<Vec<_>>()[0];

    // Search TeacherClassBridge with the username from the TeacherCookie
    // Find all of the courses, search for Attendees with AttendeeClassBridge
    // Check all Attendances that got the same ID and an attendance today.

    let tcb_results = &tcb_dsl::teacherclassbridge
        .filter(tcb_dsl::username.eq(teacher.name))
        .order(tcb_dsl::class_id.desc())
        .load::<TeacherClassBridge>(&*conn)
        .expect("Error loading teacher class bridges");

    let class_id_results: Vec<i32> =
        tcb_results
        .iter()
        .map(|tcb_result| tcb_result.class_id)
        .collect();

    let class_results = class_dsl::class
        .load::<Class>(&*conn)
        .expect("Error loading classes");

    let _class_results: Vec<_> = class_results
        .into_iter()
        .filter(|class_result| {
            let mut result = false;
            for class_id_result in &class_id_results {
                if class_id_result.eq(&class_result.class_id) {
                    result = true;
                }
            }

            result
        })
        .collect();

    let absent = vec![
        crate::models::attendee::Attendee {
            attendee_id: 2,
            tag_id: String::from("231332"),
            attendee_name: String::from("Sidney"),
        }
    ];

    let attending = vec![
        crate::models::attendee::Attendee {
            attendee_id: 3,
            tag_id: String::from("12321232"),
            attendee_name: String::from("Louise"),
        }
    ];

    let class: Vec<ClassData> = vec![
        ClassData {
            class_name: String::from("Bah"),
            absent,
            attending
        }
    ];

    let context = Context {
        logged_in: true,
        data: class,
    };

    Template::render("index", context)
}

#[get("/", rank = 2)]
pub fn unauthenticated() -> Redirect {
    Redirect::to("/login")
}
