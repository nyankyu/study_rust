fn main() {
    let s = "Hello, world!";
    let mut ss = String::from("hello world");
    ss.push_str(", str");

    println!("{}", s);
    println!("{}", ss);
}
