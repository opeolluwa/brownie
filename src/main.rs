#[macro_use]
extern crate rocket; // rocket is a framework for building web applications with Rust.
use rocket_dyn_templates::Template; //for parsing of templates
mod routes; //import the route handlers from the routes module

//static files 
use rocket_contrib::serve::StaticFiles;

//import database config
// use rocket_contrib::databases::diesel;
// #[database("datastore")] //connect to the database
// struct RustlyDatastore(diesel::MysqlConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            //the views are in the templates base dir
            routes![
                routes::views::index,
                routes::views::login,
                routes::views::sign_up
            ],
        )
        //the authentication endpoints
        .mount("/auth", routes![routes::auth::login, routes::auth::sign_up])
        .mount("/public", StaticFiles::from("/static")) //static files
        .attach(Template::fairing()) //template engines
        // .attach(RustlyDatastore::fairing()) //database connection
}
