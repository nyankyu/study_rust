fn main() {
    let s = "Hello, world!";
    let mut ss = String::from("hello world");
    ss.push_str(", str");

    println!("{}", s);
    println!("{}", ss);

    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);

    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1: [{}]", s1);
    println!("s2: [{}]", s2);

    takes_ownership(s1);
    //println!("{}", s1);

    make_copy(x);
    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
}