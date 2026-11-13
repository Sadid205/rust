struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        sign_in_count: 1,
        active: true,
    };

    user1.email = String::from("another@example.com");

    let user2 = User {
        email: String::from("bob@example.com"),
        ..user1
    };
    println!("User: {}, Email: {}, Sign-in Count: {}, Active: {}", user2.username, user2.email, user2.sign_in_count, user2.active);
    // println!("User: {}, Email: {}, Sign-in Count: {}, Active: {}", user1.username, user1.email, user1.sign_in_count, user1.active);
}
