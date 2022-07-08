//for generating unique ids
use nanoid::nanoid;

struct MinifiedLink {
    original_url: String,
    url_id: String,
    last_modified: String,
    total_views: u64,
}

impl MinifiedLink {
    fn new(original_url: String, last_modified: String) -> MinifiedLink {
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
