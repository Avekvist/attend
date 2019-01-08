use crate::models::teacher::{ TeacherCookie, TeacherLogin };
#[cfg(not(target_arch = "arm"))] use std::env;
use rocket::response::{ Flash, Redirect };
use rocket_contrib::templates::Template;
use rocket::http::{ Cookie, Cookies };
use rocket::request::FlashMessage;
use std::collections::HashMap;
use rocket::request::Form;
use crate::AttendDatabase;
use super::super::Context;

#[cfg(not(target_arch = "arm"))]
fn verify_password(password: &str, hashed_password: &str) -> bool {
    argonautica::Verifier::default()
        .with_hash(hashed_password)
        .with_password(password)
        .with_secret_key(env::var("SECRET_KEY").expect("SECRET_KEY must be set"))
        .verify()
        .unwrap()
}

#[cfg(target_arch = "arm")]
fn verify_password(password: &str, hashed_password: &str) -> bool {
    password == hashed_password
}

#[post("/process_login", data="<teacher_login>")]
pub fn login(conn: AttendDatabase, mut cookies: Cookies, teacher_login: Form<TeacherLogin>) -> Result<Redirect, Flash<Redirect>> {
    use diesel::prelude::*;
    use crate::schema::teacher::dsl::*;
    use crate::models::teacher::*;

    let results = teacher
        .load::<Teacher>(&*conn);

    match results {
        Ok(results) => {
            let mut success = false;

            for result in results {
                if result.username == teacher_login.username && verify_password(&teacher_login.password, &result.password) && result.is_admin {
                    cookies.add_private(Cookie::new("teacher", format!("name={}&username={}", result.teacher_name, result.username)));
                    success = true;
                }
            }

            if success {
                Ok(Redirect::to("/"))
            } else {
                Err(Flash::error(Redirect::to("/login"), "Invalid username or password. "))
            }
        },
        _ => Err(Flash::error(Redirect::to("/login"), "Couldn't access database. "))
    }
}

#[get("/login")]
pub fn authenticated(_teacher: TeacherCookie) -> Redirect {
    Redirect::to("/")
}

#[get("/login", rank = 2)]
pub fn unauthenticated(flash: Option<FlashMessage>) -> Template {
    let mut show_flash = false;

    let flash_result = flash.map(|msg| {
        show_flash = true;
        format!("{}: {}", msg.name(), msg.msg())
    }).unwrap_or_else(|| {
        show_flash = false;
        "".to_string()
    });
    
    let mut data = HashMap::new();
    data.insert("show_flash", String::from(if show_flash { "true" } else { "false" }));
    data.insert("flash_result", flash_result);

    let context = Context {
        logged_in: false,
        data,
    };

    Template::render("operations/login", context)
}
