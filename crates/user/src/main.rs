use users::{get_user_by_uid, get_current_uid};

fn main() {
    let user = get_user_by_uid(get_current_uid()).unwrap();
    let username = user.name().to_str().unwrap();
    let hostname = hostname::get().unwrap();
    let hostname_str = hostname.to_str().unwrap();
    
    println!("{}@{}", username, hostname_str);
}
