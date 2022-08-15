use std::f32::consts::E;
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("userlist.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let username = read_username_from_file().expect("fialed read userlist file.");
    print!("{}", username);
}
