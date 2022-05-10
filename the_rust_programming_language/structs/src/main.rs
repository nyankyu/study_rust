fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        email: String::from("aaa@bbb.ccc"),
        username: String::from("hogename"),
        sign_in_count: 123,
        active: false,
    };

    println!("{}", user1.username);
}
