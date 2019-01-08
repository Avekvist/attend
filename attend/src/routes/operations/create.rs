use crate::models::teacher::{ TeacherCreate, TeacherCookie };
#[cfg(not(target_arch = "arm"))] use std::env;
use rocket::response::{ Flash, Redirect };
use rocket_contrib::templates::Template;
use rocket::request::FlashMessage;
use std::collections::HashMap;
use rocket::request::Form;
use crate::AttendDatabase;
use super::super::Context;

#[cfg(not(target_arch = "arm"))]
fn hash_password(password: &str) -> String {
    argonautica::Hasher::default()
        .with_password(password)
        .with_secret_key(env::var("SECRET_KEY").expect("SECRET_KEY must be set"))
        .hash()
        .unwrap()
}

#[cfg(target_arch = "arm")]
fn hash_password(password: &str) -> String {
    password
}

#[post("/process_create", data="<teacher_create>")]
pub fn create(conn: AttendDatabase, teacher_create: Form<TeacherCreate>) -> Result<Redirect, Flash<Redirect>> {
    use diesel::prelude::*;
    use crate::schema::teacher::dsl;
    use crate::models::teacher::*;
    
    let results = dsl::teacher
        .load::<Teacher>(&*conn)
        .expect("Error loading teachers");

    let mut in_system = false;

    for result in results {        
        if result.username == teacher_create.username {
            in_system = true;
            break;
        }
    }
    
    if !in_system {
        use crate::schema::teacher;

        let new_teacher = Teacher {
            teacher_name: teacher_create.teacher_name.clone(),
            username: teacher_create.username.clone(),
            password: hash_password(&teacher_create.password),
            is_admin: false,
        };

        let result = diesel::insert_into(teacher::table)
            .values(&new_teacher)
            .execute(&*conn);
        
        match result {
            Ok(_) => Ok(Redirect::to("/login")),
            _ => Err(Flash::error(Redirect::to("/create"), "Couldn't add teacher. "))
        }
    } else {
        Err(Flash::error(Redirect::to("/create"), "Teacher already exists. "))
    }
}

#[get("/create")]
pub fn authenticated(_teacher: TeacherCookie) -> Redirect {
    Redirect::to("/")
}

#[get("/create", rank = 2)]
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

    Template::render("operations/create", context)
}
