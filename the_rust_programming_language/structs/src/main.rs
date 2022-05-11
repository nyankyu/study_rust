fn main() {
    let user1 = build_user(String::from("mail address"), String::from("name name"));
    println!("{}", user1.email);

    let user2 = User {
        email: String::from("aaa@bbb.ccc"),
        username: String::from("hogename"),
        ..user1
    };
    println!("{}", user2.username);

    let bg_color = Color(0, 23, 255);
    let point_p = Point3D(0.0, 0.0, 23.5);
    println!("{}", bg_color.2);
    println!("{}", point_p.2);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point3D(f32, f32, f32);

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
