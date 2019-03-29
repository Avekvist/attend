use rocket_contrib::templates::Template;
use crate::models::teacher::TeacherCookie;
use rocket::response::Redirect;
use crate::AttendDatabase;

#[derive(Serialize)]
pub struct ClassData {
    pub class_name: String,
    pub absentees: Vec<crate::models::attendee::Attendee>,
    pub attendees: Vec<crate::models::attendee::Attendee>
}

#[derive(Serialize)]
pub struct Context<'a> {
    pub logged_in: bool,
    pub date: &'a str,
    pub data: Vec<ClassData>,
}

#[get("/")]
pub fn authenticated(conn: AttendDatabase, teacher: TeacherCookie) -> Template {
    use crate::schema::attendeeclassbridge::dsl as acb_dsl;
    use crate::schema::teacherclassbridge::dsl as tcb_dsl;
    use crate::schema::attendance::dsl as attendance_dsl;
    use crate::schema::attendee::dsl as attendee_dsl;
    use crate::schema::class::dsl as class_dsl;
    use crate::models::attendeeclassbridge::*;
    use crate::models::teacherclassbridge::*;
    use crate::models::attendance::*;
    use crate::models::attendee::*;
    use crate::models::class::*;
    use diesel::prelude::*;

    let datetime =
        chrono::prelude::Local::now()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

    let date =
        &datetime
            .split(' ')
            .collect::<Vec<_>>()[0];

    // Search TeacherClassBridge with the username from the TeacherCookie
    // Find all of the courses, search for Attendees with AttendeeClassBridge
    // Check all Attendances that got the same ID and an attendance today.

    let tcb_results =
        &tcb_dsl::teacherclassbridge
            .filter(tcb_dsl::username.eq(teacher.name))
            .order(tcb_dsl::class_id.desc())
            .load::<TeacherClassBridge>(&*conn)
            .expect("Error loading teacher class bridges");

    let class_id_results: Vec<i32> =
        tcb_results
            .iter()
            .map(|tcb_result| tcb_result.class_id)
            .collect();

    let class_results =
        class_dsl::class
            .load::<Class>(&*conn)
            .expect("Error loading classes");

    let class_results: Vec<_> =
        class_results
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

    // Fetch Attendees
    let attendees =
        attendee_dsl::attendee
            .load::<Attendee>(&*conn)
            .expect("Error loading attendees");

    // Sort them in their appropriate classes
    let mut classes = vec![];
    let mut class_name = String::new();

    for class_id_result in &class_id_results {
        for class_result in &class_results {
            if class_result.class_id == *class_id_result {
                class_name = class_result.class_name.clone();
            }
        }

        let acb_results =
            &acb_dsl::attendeeclassbridge
                .filter(acb_dsl::class_id.eq(class_id_result))
                .order(acb_dsl::attendee_id.asc())
                .load::<AttendeeClassBridge>(&*conn)
                .expect("Error loading attendee class bridges");

        // Match attendees in class
        let attendees_results: Vec<_> =
            attendees
                .clone()
                .into_iter()
                .filter(|attendee| {
                    let mut result = false;

                    for acb_result in acb_results {
                        if acb_result.attendee_id == attendee.attendee_id {
                            result = true;
                        }
                    }

                    result
                })
                .collect();

        // Match attendees with attendance for today
        let attendance =
            attendance_dsl::attendance
                .filter(attendance_dsl::attendance_date.eq(date))
                .order(attendance_dsl::attendee_id.asc())
                .load::<Attendance>(&*conn)
                .expect("Error loading attendance");

        let attendees =
            attendees_results
                .clone()
                .into_iter()
                .filter(|attendee| {
                    let mut result = false;

                    for attendance_result in &attendance {
                        for acb_result in acb_results {
                            if acb_result.class_id == *class_id_result && acb_result.class_id.eq(&attendance_result.class_id) && acb_result.attendee_id.eq(&attendee.attendee_id) && attendance_result.attendee_id.eq(&attendee.attendee_id) {
                                    result = true;
                            }
                        }
                    }

                    result
                })
                .collect();

        let absentees =
            attendees_results
                .clone()
                .into_iter()
                .filter(|attendee| {
                    let mut result = true;

                    for attendance_result in &attendance {
                        for acb_result in acb_results {
                            if acb_result.class_id == *class_id_result && acb_result.class_id.eq(&attendance_result.class_id) && acb_result.attendee_id.eq(&attendee.attendee_id) && attendance_result.attendee_id.eq(&attendee.attendee_id) {
                                result = false;
                            }
                        }
                    }

                    result
                })
                .collect();

        let class_data = ClassData {
            class_name: class_name.to_owned(),
            attendees,
            absentees,
        };

        classes.push(class_data);
    }

    let context = Context {
        logged_in: true,
        date,
        data: classes,
    };

    Template::render("index", context)
}

#[get("/", rank = 2)]
pub fn unauthenticated() -> Redirect {
    Redirect::to("/login")
}
