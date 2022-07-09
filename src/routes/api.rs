/*    use rocket_db_pools::sqlx::Row;
   use rocket_db_pools::Connection;

   #[derive(Responder, Debug)]
   struct ApiResponse {
       status: String,
       message: String,
   }
   #[derive(FormData)]
   struct RauUrl(String);
*/

// #[post("/v1/links/minify", format = "application/json", data = "<raw_url>")]
#[post("/v1/links/minify")]
pub fn minify() -> &'static str {
    let raw_url = "https://www.rust-lang.org/en-US/";
    /*  ApiResponse {
        status: "success".to_string(),
        message: "minified url".to_string(),
    } */
    raw_url
}
