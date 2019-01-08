use rocket::http::{ Status, ContentType };
use rocket::request::Request;
use rocket::response::Responder;
use rocket::Response;

use std::io::Cursor;

use crate::schema::attendee;

#[derive(Queryable)]
pub struct Attendee {
    pub attendee_id: i32,
    pub tag_id: String,
    pub attendee_name: String,
}

#[table_name = "attendee"]
#[derive(Insertable, FromForm)]
pub struct AttendeeCreate {
    pub tag_id: String,
    pub attendee_name: String,
}

impl<'r> Responder<'r> for Attendee {
    fn respond_to(self, request: &Request<'_>) -> Result<Response<'r>, Status> {
        if let Some(_) = request.cookies().get_private("teacher") {
            Ok(Response::build()
                .status(Status::Ok)
                .header(ContentType::HTML)
                .sized_body(Cursor::new(format!("ID: {}, Name: {}", self.attendee_id, self.attendee_name)))
                .finalize())
        } else {
            Err(Status::NotFound)
        }
    }
}
