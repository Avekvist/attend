use rocket::request::{ Request, FromRequest, Outcome };
use rocket::outcome::IntoOutcome;
use crate::schema::teacher;
use std::str;

#[table_name = "teacher"]
#[derive(Serialize, FromForm, Queryable, Insertable)]
pub struct Teacher {
    pub username: String,
    pub password: String,
    pub teacher_name: String,
    pub is_admin: bool,
}

#[derive(Serialize, FromForm)]
pub struct TeacherCreate {
    pub teacher_name: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, FromForm)]
pub struct TeacherLogin {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, FromForm)]
pub struct TeacherCookie {
    pub name: String,
    pub username: String,
}

impl str::FromStr for TeacherCookie {
    type Err = ();

    fn from_str(input_string: &str) -> Result<Self, Self::Err> {
        let result: Vec<&str> = input_string.split("&").collect();

        if result.len() > 1 {
            let name = result[0].split("=").collect::<Vec<&str>>()[1];
            let username = result[1].split("=").collect::<Vec<&str>>()[1];

            Ok(TeacherCookie { name: name.to_string(), username: username.to_string(), })
        } else {
            Err(())
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for TeacherCookie {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, ()> {
        request.cookies()
            .get_private("teacher")
            .and_then(|cookie| cookie.value().parse().ok())
            .or_forward(())
    }
}
