use rocket_contrib::templates::Template;
use crate::models::teacher::TeacherCookie;
use rocket::response::Redirect;
use std::collections::HashMap;
use crate::AttendDatabase;
use super::Context;

#[get("/")]
pub fn authenticated(conn: AttendDatabase, teacher: TeacherCookie) -> Template {
    use diesel::prelude::*;
    use crate::schema::teacherclassbridge::dsl as tcb_dsl;
    use crate::schema::attendance::dsl as attendance_dsl;
    use crate::schema::class::dsl as class_dsl;
    use crate::models::teacherclassbridge::*;
    use crate::models::attendance::*;
    use crate::models::class::*;
    
    let datetime = chrono::prelude::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let date = &datetime.split(" ").collect::<Vec<_>>()[0];

    // Find the courses with attendances today
    // Check if the teacher got any of these courses
    // If true, print the courses in one section

    let attendance_results = attendance_dsl::attendance
        .filter(attendance_dsl::attendance_date.eq(date))
        .load::<Attendance>(&*conn)
        .expect("Error loading attendances");
    
    let tcb_results = &tcb_dsl::teacherclassbridge
        .filter(tcb_dsl::username.eq(teacher.name))
        .load::<TeacherClassBridge>(&*conn)
        .expect("Error loading teacher class bridges");

    let results: Vec<_> = attendance_results.into_iter().filter(|attendance_result| {
        let mut success = false;
        
        tcb_results.into_iter().for_each(|tcb_result| {
            if attendance_result.class_id == tcb_result.class_id {
                success = true;
            }
        });

        success
    }).collect();

    let result_vector: Vec<_> = results.into_iter().map(|attendance| attendance.attendance_date).collect();

    let mut data = HashMap::new();
    data.insert("attendances", result_vector);

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
