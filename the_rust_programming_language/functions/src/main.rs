fn main() {
    println!("Hello, world!");

    let x = {
        let y = 3;
        plus_one(y) * five()
    };

    another_function(x, 0.0001);
}

fn another_function(x: i32, y: f32) {
    println!("The value of x is: {}, y is: {}", x, y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}