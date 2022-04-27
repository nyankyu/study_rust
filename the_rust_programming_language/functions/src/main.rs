fn main() {
    println!("Hello, world!");

    another_function(42, 0.0001);
}

fn another_function(x: i32, y: f32) {
    println!("The value of x is: {}, y is: {}", x, y);
}