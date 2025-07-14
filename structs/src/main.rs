fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // Struct update syntax for instances that include most values from another instance
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // Struct update syntax allows '..' to update remaining, non-explicit fields
    // Fields can be specified in any order
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Field init shorthand
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // Rather than repeating username & email (e.g., username: username), just write the fields
        // directly
        username,
        email,
        sign_in_count: 1,
    }
}
