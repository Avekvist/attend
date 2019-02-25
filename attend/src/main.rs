#![allow(proc_macro_derive_resolution_fallback)]
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[cfg(not(target_arch = "arm"))]
extern crate argonautica;
extern crate chrono;
extern crate dotenv;
extern crate serde;

use rocket_contrib::databases::diesel as ds;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use dotenv::dotenv;

pub mod catchers;
pub mod routes;
pub mod models;
pub mod schema;
pub mod helper;

use crate::models::class::ClassJSON;

#[database("attend")]
pub struct AttendDatabase(ds::MysqlConnection);

#[get("/test")]
fn test() -> rocket::response::Content<&'static str> {
    rocket::response::Content(rocket::http::ContentType::HTML,
        r"
            <form action='/handle_tag' method='post'>
                <input type='text' name='tag_id' />
                <input type='submit' value='Send' />
            </form>
        "
    )
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![
            test,

            routes::attendance::authenticated,
            routes::attendance::list_authenticated,
            routes::attendance::list_unauthenticated,
            routes::attendance::unauthenticated,

            routes::classes::authenticated,
            routes::classes::unauthenticated,

            routes::index::authenticated,
            routes::index::unauthenticated,

            routes::operations::create_attendee,
            routes::operations::create::create,
            routes::operations::create::authenticated,
            routes::operations::create::unauthenticated,
            routes::operations::handle_tag,
            routes::operations::handle_tag_failure,
            routes::operations::login::authenticated,
            routes::operations::login::login,
            routes::operations::login::unauthenticated,
            routes::operations::logout::authenticated,
            routes::operations::logout::unauthenticated,

            routes::support::authenticated,
            routes::support::unauthenticated,

            routes::students::authenticated,
            routes::students::list_authenticated,
            routes::students::list_unauthenticated,
            routes::students::unauthenticated,

            routes::tags::authenticated,
            routes::tags::list_authenticated,
            routes::tags::list_unauthenticated,
            routes::tags::unauthenticated,
        ])
        .mount("/", StaticFiles::from("static"))
        .register(catchers![
            catchers::internal_error,
            catchers::not_found,
            catchers::unprocessable_entity,
        ])
        .attach(Template::fairing())
        .attach(AttendDatabase::fairing())
}

pub fn get_classes() -> Result<Vec<ClassJSON>, serde_json::error::Error> {
    serde_json::from_str(
        &std::fs::read_to_string("class_data.json")
            .expect("Couldn't read the file. ")
    )
}

fn main() {
    let classes = get_classes().unwrap();

    for class in classes {
        println!("The class is called {} with id {}", class.name, class.id);
        println!("{}", class.time);
    }

    dotenv().ok();
    rocket().launch();
}
