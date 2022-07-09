use rocket_dyn_templates::{context, Template};
//the login page accessible only to unauthenticated users via /auth/login
#[post("/login")]
pub fn login() -> Template {
    Template::render("auth/login", context! { /* name:"drizzle"  */})
}

//the sign up page accessible only to unauthenticated users via /auth/signup
#[post("/signup")]
pub fn sign_up() -> String {
    "sign up".to_string()
}
