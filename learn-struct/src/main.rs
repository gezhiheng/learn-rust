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
        email: String::from("gezhiheng.me@gmail.com"),
        ..user1
    };

    println!("{}", &user2.username);

    println!("{}", user1.active);

    println!("{}", user1.email);

    println!("{}", user1.sign_in_count);
}
