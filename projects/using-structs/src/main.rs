fn main() {
    let user1 = build_user(
        String::from("someusername123"),
        String::from("someone@example.com"),
    );
    println!(
        "activity: {}\nusername: {}\nemail: {}\ncounter: {}",
        user1.active, user1.username, user1.email, user1.sign_in_count
    );

    // Creating instances from other instances with struct update syntax
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("anotheruser@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!(
        "\nactivity: {}\nusername: {}\nemail: {}\ncounter: {}",
        user2.active, user2.username, user2.email, user2.sign_in_count
    );

    let user3 = User {
        email: String::from("justanotheruser@example.com"),
        ..user2 // Cannot use user1 here because data was moved to user2
    };
    println!(
        "\nactivity: {}\nusername: {}\nemail: {}\ncounter: {}",
        user3.active, user3.username, user3.email, user3.sign_in_count
    );
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
