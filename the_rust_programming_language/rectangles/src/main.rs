#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&mut self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let mut rect1 = Rectangle {width: 30, height: 50};
    println!("{:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
