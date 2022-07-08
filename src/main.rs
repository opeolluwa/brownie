#[macro_use]
extern crate rocket; // rocket is a framework for building web applications with Rust.
use rocket_dyn_templates::Template; //for parsing of templates
mod routes; //import the route handlers from the routes module
use rocket::fs::{relative, FileServer}; //for serving static files
use rocket_db_pools::{sqlx, Database};
use routes::*; //import all route handlers from the routes module //for database connection

//init database config
#[derive(Database)]
#[database("rustly_datastore")]
struct RustlyDatastore(sqlx::MySqlPool);

#[launch]
fn rocket() -> _ {
    rocket::build()
        //views
        .mount("/", routes![views::index, views::login, views::sign_up])
        .mount("/auth", routes![auth::login, auth::sign_up])//auth routes through POST requests
        .mount("/dashboard", routes![user::dashboard])//dashboard routes through GET requests
        .mount("/static", FileServer::from(relative!("public"))) //static files
        .attach(Template::fairing()) //template engines
        .attach(RustlyDatastore::init()) //database connection
}
