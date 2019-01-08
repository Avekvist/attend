use rocket_contrib::templates::Template;
use crate::models::teacher::TeacherCookie;
use rocket::response::Redirect;
use std::collections::HashMap;
use crate::AttendDatabase;
use super::Context;

#[get("/attendance")]
pub fn authenticated(conn: AttendDatabase, teacher: TeacherCookie) -> Template {
    use diesel::prelude::*;
    use crate::schema::teacherclassbridge::dsl as tcb_dsl;
    use crate::schema::attendance::dsl as attendance_dsl;
    use crate::schema::class::dsl as class_dsl;
    use crate::models::teacherclassbridge::*;
    use crate::models::attendance::*;
    use crate::models::class::*;

    let attendance_results = attendance_dsl::attendance
        .load::<Attendance>(&*conn)
        .expect("Couldn't loading attendances");

    let tcb_results = &tcb_dsl::teacherclassbridge
        .filter(tcb_dsl::username.eq(teacher.name))
        .load::<TeacherClassBridge>(&*conn)
        .expect("Error loading teacher class bridges");

    let class_results = &class_dsl::class
        .load::<Class>(&*conn)
        .expect("Error loading classes");

    let class_results: Vec<_> = class_results.into_iter().filter(|class_result| {
        let mut success = false;
        
        tcb_results.into_iter().for_each(|tcb_result| {
            if tcb_result.class_id == class_result.class_id {
                success = true;
            }
        });

        success
    }).collect();

    let attendance_results: &Vec<_> = &attendance_results.into_iter().filter(|attendance_result| {
        let mut success = false;
        
        tcb_results.into_iter().for_each(|tcb_result| {
            if attendance_result.class_id == tcb_result.class_id {
                success = true;
            }
        });

        success
    }).collect();

    let mut results: Vec<(String, String)> = Vec::new();

    for class_result in class_results {
        for attendance_result in attendance_results {
            if class_result.class_id == attendance_result.class_id {
                results.push((class_result.class_name.clone(), attendance_result.attendance_date.clone()));
            }
        }
    }

    //let results: Vec<_> = attendance_results.into_iter().map(|attendance| attendance.attendance_date).collect();

    let mut data = HashMap::new();
    data.insert("attendances", results);

    let context = Context {
        logged_in: true,
        data,
    };

    Template::render("attendance/attendances", context)
}

#[get("/attendance", rank = 2)]
pub fn unauthenticated() -> Redirect {
    Redirect::to("/login")
}

#[get("/attendance/<_id>")]
pub fn list_authenticated(_conn: AttendDatabase, _teacher: TeacherCookie, _id: String) -> Template {
    let context = Context::<Option<u8>> {
        logged_in: true,
        data: None,
    };

    Template::render("attendance/attendance", context)
}

#[get("/attendance/<_id>", rank = 2)]
pub fn list_unauthenticated(_id: u16) -> Redirect {
    Redirect::to("/login")
}
