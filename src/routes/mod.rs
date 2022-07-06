//definition of all route handlers
use rocket_dyn_templates::{context, Template};
//the home page
#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! { /* name:"drizzle" */ })
}

//the login page accessible only to unauthenticated users via /auth/login
#[get("/login")]
pub fn login() -> Template {
    Template::render("auth/login", context! { /* name:"drizzle"  */})
}

//the sign up page accessible only to unauthenticated users via /auth/signup
#[get("/signup")]
pub fn sign_up() -> String {
    "sign up".to_string()
}
