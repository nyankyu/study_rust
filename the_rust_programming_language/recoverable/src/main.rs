use std::{fs::File, io::ErrorKind};

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
