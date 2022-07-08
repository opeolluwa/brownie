//for generating unique ids
use nanoid::nanoid;

#[derive(Debug)]
pub struct MinifiedLink {
    original_url: String,
    url_id: String,
    last_modified: String,
    total_views: u64,
}

#[derive(Debug)]
pub struct User {
    user_id: String,
    email: String,
    username: String,
    password: String,
    last_modified: String,
}

impl Users {
   pub fn find_by_id(id:String, pool:pool<>)
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
impl MinifiedLink {
    pub fn new(original_url: String, last_modified: String) -> MinifiedLink {
        let url_id = generate_url_id();
        let last_modified = last_modified;
        let total_views = 0;
        Uri {
            original_url,
            url_id,
            last_modified,
            total_views,
        }
    }
}

//generate a unique id for the url
fn generate_url_id() -> String {
    let url_id = nanoid!(6);
    let url_id = format!("{}", url_id);
    url_id
}
