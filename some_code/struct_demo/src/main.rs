fn main() {
    println!("Hello, world!");

    let user1: User = build_user(String::from("kun"), String::from("kun@kungal.com"));

    println!("{}", user1.email);
}

struct User {
    username: String,
    email: String,
    count: u64,
    active: bool,
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        count: 1,
    }
}
