#[macro_use]
extern crate rocket; // rocket is a framework for building web applications with Rust.
use rocket_dyn_templates::Template; //for parsing of templates
mod routes; //import the route handlers from the routes module

//mount the route handlers from the routes module
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![routes::index,])
        .mount("/auth", routes![routes::login, routes::sign_up])
        .attach(Template::fairing())
}
