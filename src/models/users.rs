///define a public struct to handle  user data
/// the obeject will be major interface in creation and query of users
pub struct User {
    firstname: String,
    username: String,
    password: String,
    email: String,
}

///an object to return for each operation
pub struct OpsStatus<T> {
    success: bool,
    message: String,
    data: T,
}
//add methods to create and query the user object
impl User {
    pub fn new(firstname: &str, username: &str, password: &str) -> Self {
        User {
            firstname: firstname.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            email: "".to_string(),
        }
    }

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
    }
}

//  impl MinifiedLink {
//      pub fn new(original_url: String, last_modified: String) -> MinifiedLink {
//          let url_id = generate_url_id();
//          let last_modified = last_modified;
//          let total_views = 0;
//          Uri {
//              original_url,
//              url_id,
//              last_modified,
//              total_views,
//          }
//      }
//  }
