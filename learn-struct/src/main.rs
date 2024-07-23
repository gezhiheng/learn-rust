struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: "someone@example.com".to_string(),
        username: "henry".to_string(),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.username);

    let user2 = User {
        ..user1
    };

    println!("{}", user1.sign_in_count);
}
