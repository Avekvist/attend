use rocket::http::{ Cookie, Cookies };
use rocket::response::Redirect;

#[get("/logout")]
pub fn authenticated(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("teacher"));

    Redirect::to("/login")
}

#[get("/logout", rank = 2)]
pub fn unauthenticated() -> Redirect {
    Redirect::to("/login")
}
