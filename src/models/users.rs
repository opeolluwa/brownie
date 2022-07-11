use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::{sqlx, sqlx::Executor, Connection};
extern crate bcrypt;
use crate::RustlyDatastore;
use bcrypt::{hash, DEFAULT_COST};

///define a public struct to handle  user data
/// the obeject will be major interface in creation and query of users

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    firstname: String,
    lastname: String,
    username: String,
    password: String,
    email: String,
}


///an object to return response  for each api request
/// the object will contain a success field which return true or false
/// the object will contain a message field which will contain a message if the request is not successful and more detailed information
/// finally the field will contain a data which is implemented as a geeric field.
/// the data field will contain any data that need to be sent back to the client
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    success: bool,
    message: String,
    data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError<T> {
    success: bool,
    message: String,
    data: T,
    status_code: u16,
}

/// an object to return the status of a database operation
/// the object will contain a success field which return true or false
/// the object will also contain a data field which is implemented as a generic.
/// the data field will hold and data that need to be sent back to the client application
#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseResponse<T> {
    success: bool,
    data: T,
}

/// methods to create and query the user object
impl User {
    pub async fn new(mut database: Connection<RustlyDatastore>, user: User) -> ApiResponse<()> {
        let hashed_password = hash(user.password, DEFAULT_COST).unwrap();
        let query =
            sqlx::query("INSERT INTO user_information (firstname, lastname, username, password, email) VALUES (?, ?, ?, ?,?)")
                .bind(user.firstname)
                .bind(user.lastname)
                .bind(user.username)
                .bind(hashed_password)
                .bind(user.email);
        //execute the query
        let query_response = database.execute(query).await;
        println!("{:?}", query_response);
        //return the response
        ApiResponse {
            success: true,
            data: (),
            message: "User successfully created".to_string(),
        }
    }
    /* ///add new user to database
    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.user_id, user);
    }

    pub fn get_user(&self, user_id: &str) -> Option<&User> {
        self.users.get(user_id)
    }

    pub fn get_user_mut(&mut self, user_id: &str) -> Option<&mut User> {
        self.users.get_mut(user_id)
    }

    pub fn upadate_user(&mut self, user: User) {
        self.users.insert(user.user_id, user);
    } */
}
