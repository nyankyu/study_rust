#[derive(Debug)]
enum IpAddr {
    V4(u8, u8,u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("[{:?}]: called", self);
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);

    let msg1 = Message::Write(String::from("most inportant message"));
    let msg2 = Message::Quit;
    let msg3 = Message::Move { x: 42, y: 90 };
    msg1.call();
    msg2.call();
    msg3.call();
}
