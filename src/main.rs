// #[macro_use]
// extern crate dotenv_codegen;
// extern crate dotenv;
// extern crate rocket;
// use dotenv::dotenv;

// //the base url of the website
// // #[get("/<name>/<age>")]
// // fn hello(name: String, age: u8) -> String {
// //     format!("Hello, {} year old named {}!", age, name)
// // }
// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![index])
// }
// fn main() {
//     dotenv().ok();
//     println!("Parsed environment variables");
//     let url = dotenv!("DATABASE_URL");
//     let builder = mysql::OptsBuilder::from_opts(mysql::Opts::from_url(&url).unwrap());
//     let pool = mysql::Pool::new(builder.ssl_opts(mysql::SslOpts::default())).unwrap();
//     let _conn = pool.get_conn().unwrap();
//     println!("Successfully connected to PlanetScale!");
//     rocket::ignite().mount("/hello", routes![hello]).launch();
// }


#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
