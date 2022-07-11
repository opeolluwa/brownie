use crate::RustlyDatastore;
use nanoid::nanoid;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_db_pools::{sqlx, sqlx::Executor, Connection};

#[derive(Responder, Debug, Serialize)]
pub struct ApiResponse {
    status: String,
    // message: String,
}
#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct RequestPayload {
    raw_url: String,
}

#[post("/v1/links/minify", format = "application/json", data = "<payload>")]
pub async fn minify(
    mut database: Connection<RustlyDatastore>,
    payload: Json<RequestPayload>,
) -> Json<String> {
    println!("{:?}, raw_url{}", payload, payload.raw_url);
    let raw_url = &payload.clone().raw_url;
    let original_url = raw_url.parse::<String>().unwrap();
    let url_id = nanoid!(6);
    let total_views = 0;

    //save the data
    let query = sqlx::query(
        "INSERT INTO links (url_id, original_url, total_views) VALUES (?, ?, ?)",
    )
    .bind(url_id.clone())
    .bind(original_url)
    .bind(total_views);
    // .execute(&mut database);
    let res = database.execute(query).await;
    println!("{:?}", res);
    // Json(response) */
    let minified_url = format!("{}/{}", dotenv!("APP_URL"), url_id);
    Json(minified_url)
}

// to retrieve the minified url from the database then redirect them to the original url
// #[get("/<url_id>")]
// pub fn get_minified_url(url_id: String) -> &'static str {
//     "https://www.rust-lang.org/en-US/"
// }
