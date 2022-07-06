#[macro_use]
extern crate rocket;

// Try visiting:
//   http://127.0.0.1:8000/hello/world
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
