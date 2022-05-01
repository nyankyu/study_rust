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

    let s3 = give_ownership();
    let s4 = String::from("hoge");
    let s5 = takes_and_gives_back(s4);
    println!("{}", s3);
    //println!("{}", s4);
    println!("{}", s5);

    let s10 = String::from("abura katabura");
    let (s20, len) = calculate_length(s10);
    println!("The length of '{}' is {}.", s20, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn give_ownership() -> String {
    String::from("World!")
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}