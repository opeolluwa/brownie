use crate::RustlyDatastore;
// use mysql::serde::Serialize;
use nanoid::nanoid;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket_db_pools::sqlx;
use rocket_db_pools::Connection;

#[derive(Responder, Debug, Serialize)]
pub struct ApiResponse {
    status: String,
    // message: String,
}

#[post("/v1/links/minify", format = "application/json", data = "<raw_url>")]
pub fn minify(mut database: Connection<RustlyDatastore>, raw_url: String) -> Json<ApiResponse> {
    println!("{}", raw_url);
    let original_url = raw_url.parse::<String>().unwrap();
    let url_id = nanoid!(6);
    let last_modified = String::from("2020-01-01");
    let total_views = 0;

    //save the data
    let query = sqlx::query(
        "INSERT INTO links (url_id, original_url, last_modified, total_views) VALUES (?, ?, ?, ?)",
    )
    .bind(url_id)
    .bind(original_url)
    .bind(last_modified)
    .bind(total_views);
    let response = ApiResponse {
        status: "success".to_string(),
        // message: "minified url".to_string(),
        // payload: { minified_url:  "http://localhost:8000".to_string() },
    };
    Json(response)
    // Json(raw_url)
}

//to retrieve the minified url from the database then redirect them to the original url
// #[get("/<url_id>")]
// pub fn get_minified_url(url_id: String) -> &'static str {
//     "https://www.rust-lang.org/en-US/"
// }
