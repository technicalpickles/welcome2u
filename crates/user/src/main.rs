use users::{get_user_by_uid, get_current_uid};

use display::print_segment;

struct UserInfo {
    username: String,
    hostname: String,
}

impl UserInfo {
    fn new(username: String, hostname: String) -> Self {
        Self {
            username,
            hostname,
        }
    }

    fn username(&self) -> &str {
        &self.username
    }
    fn hostname(&self) -> &str {
        &self.hostname
    }

    fn collect() -> Self {
        let user = get_user_by_uid(get_current_uid()).unwrap();
        let username = user.name().to_str().unwrap();

        let hostname = hostname::get().unwrap();
        let hostname_str = hostname.to_str().unwrap();

        Self::new(username.to_string(), hostname_str.to_string())
    }
}

fn main() {
    let info = UserInfo::collect();
    let user = format!("{}@{}", info.username(), info.hostname());

    print_segment("Logged in as", &user);
}
