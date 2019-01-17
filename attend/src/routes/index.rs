use rocket_contrib::templates::Template;
use crate::models::teacher::TeacherCookie;
use rocket::response::Redirect;
use std::collections::HashMap;
use crate::AttendDatabase;
use super::Context;

#[derive(Serialize)]
struct ClassData {
    pub class_name: String,
    pub absent: Vec<crate::models::attendee::Attendee>,
    pub attending: Vec<crate::models::attendance::Attendance>
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
    let date = &datetime.split(" ").collect::<Vec<_>>()[0];

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
        .into_iter()
        .map(|tcb_result| tcb_result.class_id)
        .collect();

    let class_results = class_dsl::class
        .load::<Class>(&*conn)
        .expect("Error loading classes");

    let class_results: Vec<_> = class_results
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
/*
    let mut acb_results = Vec::new();

    for class in &class_results {
        acb_results.push(
            acb_dsl::attendeeclassbridge
                .filter(acb_dsl::class_id.eq(class.class_id))
                .load::<AttendeeClassBridge>(&*conn)
                .expect("Error loading attendee class bridges")
        );
    }
*/
    let absent = vec![
        crate::models::attendee::Attendee {
            attendee_id: 2,
            tag_id: String::from("231332"),
            attendee_name: String::from("Sidney"),
        }
    ];

    let attending = vec![
        crate::models::attendance::Attendance {
            attendance_id: 3,
            class_id: 2,
            attendee_id: 6,
            attendance_date: String::from("2019-01-14"),
            attendance_time: String::from("16:13:32"),
        }
    ];

    let class_data: Vec<ClassData> = vec![
        ClassData {
            class_name: String::from("Bah"),
            absent,
            attending
        }
    ];

    let mut data = HashMap::new()
        .insert("class", class_data);

    let context = Context {
        logged_in: true,
        data,
    };

    Template::render("index", context)
}

#[get("/", rank = 2)]
pub fn unauthenticated() -> Redirect {
    Redirect::to("/login")
}
