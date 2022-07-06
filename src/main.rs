#[macro_use]
extern crate rocket;
use rocket_dyn_templates::{ Template};


//import the route handlers from the routes module
mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![routes::index,])
        .mount("/auth", routes![routes::login, routes::sign_up])
        .attach(Template::fairing())
}
