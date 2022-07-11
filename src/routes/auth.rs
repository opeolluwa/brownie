use crate::models::users::{ApiResponse, User};
use crate::RustlyDatastore;
use rocket::serde::{json::Json};
use rocket_db_pools::{Connection};
use rocket_dyn_templates::{context, Template};

//the login page accessible only to unauthenticated users via /auth/login
#[post("/login")]
pub fn login() -> Template {
    Template::render("auth/login", context! { /* name:"drizzle"  */})
}

//the sign up page accessible only to unauthenticated users via /auth/signup
#[post("/signup", data = "<user>", format = "application/json")]
pub fn sign_up(
    mut database: Connection<RustlyDatastore>,
    user: Json<User>,
) -> Option<ApiResponse<()>> {
    // let new_user = User::new(database, Json(user));
    // "sign up".to_string()
    todo!()
    /*  ApiResponse {
        success: todo!(),
        data: todo!(),
    } */
}
