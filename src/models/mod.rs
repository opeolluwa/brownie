// //for generating unique ids
// use nanoid::nanoid;

// #[derive(Debug)]
// pub struct MinifiedLink {
//     original_url: String,
//     url_id: String,
//     last_modified: String,
//     total_views: u64,
// }

// #[derive(Debug)]
// pub struct User {
//     user_id: String,
//     email: String,
//     username: String,
//     password: String,
//     last_modified: String,
// }

// //generate a unique id for the url
// fn generate_url_id() -> String {
//     let url_id = nanoid!(6);
//     let url_id = format!("{}", url_id);
//     url_id
// }
pub mod link;
pub mod users;
