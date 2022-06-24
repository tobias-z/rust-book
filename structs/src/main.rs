#[allow(dead_code)]
fn main() {
    println!("Hello, world!");
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct AlwaysEqual;
