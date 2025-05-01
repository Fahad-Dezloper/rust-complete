struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

pub fn structing(){
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("user@example.com"),
        sign_in_count: 1
    };
    print!("user 1 username {:?}", user1.username);
}