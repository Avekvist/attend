use rocket::Request;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct ErrorContext<T> {
    logged_in: bool,
    error_code: u16,
    error: T,
}

#[catch(404)]
pub fn not_found(req: &Request) -> Template {
    let context = ErrorContext {
        logged_in: false,
        error_code: 404,
        error: format!("Not Found. Address '{}' couldn't be found. ", req.uri()),
    };
    
    Template::render("error", context)
}

#[catch(422)]
pub fn unprocessable_entity() -> Template {
    let context = ErrorContext {
        logged_in: false,
        error_code: 422,
        error: "Unprocessable Entity",
    };
    
    Template::render("error", context)
}

#[catch(500)]
pub fn internal_error() -> Template {
    let context = ErrorContext {
        logged_in: false,
        error_code: 500,
        error: "Internal Server Error",
    };
    
    Template::render("error", context)
}
