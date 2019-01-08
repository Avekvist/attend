use rocket::response::{ Flash, Redirect };
use crate::models::attendee::AttendeeCreate;
use rocket::request::Form;
use crate::AttendDatabase;
pub mod logout;
pub mod create;
pub mod login;

#[post("/process_create_attendee", data="<attendee_create>")]
pub fn create_attendee(conn: AttendDatabase, attendee_create: Form<AttendeeCreate>) -> Result<Redirect, Flash<Redirect>> {
    use diesel::prelude::*;
    use crate::schema::attendee::dsl;
    use crate::models::attendee::*;

    let attendee_results = dsl::attendee
        .load::<Attendee>(&*conn)
        .expect("Couldn't access attendees");
    
    let mut in_system = false;

    for attendee_result in attendee_results {
        if attendee_result.tag_id == attendee_create.tag_id {
            in_system = true;
            break;
        }
    }

    if !in_system {
        use crate::schema::attendee;

        let new_attendee = AttendeeCreate {
            tag_id: attendee_create.tag_id.clone(),
            attendee_name: attendee_create.attendee_name.clone(),
        };

        let result = diesel::insert_into(attendee::table)
            .values(&new_attendee)
            .execute(&*conn);
        
        match result {
            Ok(_) => Ok(Redirect::to("/tags")),
            _ => Err(Flash::error(Redirect::to(format!("/tags/{}", attendee_create.tag_id)), "Couldn't add attendee. "))
        }
    } else {
        Err(Flash::error(Redirect::to(format!("/tags/{}", attendee_create.tag_id)), "Tag is already assigned. "))
    }
}

#[post("/handle_tag", data="<tag_id>")]
pub fn handle_tag(conn: AttendDatabase, tag_id: String) -> &'static str {
    use diesel::prelude::*;
    use crate::schema::attendee::dsl;
    use crate::models::{ attendee::*, attendance::*, tag::*, };

    let tag_id = tag_id.split("=").collect::<Vec<&str>>()[1];

    let results = dsl::attendee
        .load::<Attendee>(&*conn)
        .expect("Error loading attendees");

    let attendee_results: Vec<_> = results.into_iter().filter(|attendee| attendee.tag_id == tag_id).collect();

    if attendee_results.len() > 0 {
        use crate::schema::attendance;

        let datetime = chrono::prelude::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let date = &datetime.split(" ").collect::<Vec<_>>()[0];
        let time = &datetime.split(" ").collect::<Vec<_>>()[1];

        let new_attendance = AttendanceCreate {
            class_id: 0,
            attendee_id: attendee_results[0].attendee_id,
            attendance_date: date.to_string(),
            attendance_time: time.to_string(),
        };

        let result = diesel::insert_into(attendance::table)
            .values(&new_attendance)
            .execute(&*conn);

        match result {
            Ok(_) => "OK",
            _ => "Couldn't add attendance. ",
        }
    } else {
        use crate::schema::tag::dsl;

        let results = dsl::tag
            .load::<Tag>(&*conn)
            .expect("Error loading tags");

        let tag_results: Vec<_> = results.into_iter().filter(|tag| tag.tag_id == tag_id).collect();

        if tag_results.len() > 0 {
            return "In System Not An Attendee";
        } else {
            use crate::schema::tag;

            let new_tag = Tag {
                tag_id: tag_id.to_string(),
                tag_date: chrono::NaiveDateTime::parse_from_str(&chrono::prelude::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), "%Y-%m-%d %H:%M:%S").unwrap(),
            };

            let result = diesel::insert_into(tag::table)
                .values(&new_tag)
                .execute(&*conn);
            
            match result {
                Ok(_) => "In System Not An Attendee",
                _ => "Couldn't handle tag. "
            }
        }
    }
}

#[post("/handle_tag", data="<_tag_id>", rank = 2)]
pub fn handle_tag_failure(_tag_id: String) -> &'static str {
    "Couldn't access database. "
}
