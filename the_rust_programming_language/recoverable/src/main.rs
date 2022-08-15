use std::f32::consts::E;
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("userlist.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    let username = read_username_from_file().expect("fialed read userlist file.");
    print!("{}", username);
}
