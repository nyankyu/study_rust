fn main() {
    println!("Hello, world!");

    let x = {
        let y = 3;
        y * 4
    };

    another_function(x, 0.0001);
}

fn another_function(x: i32, y: f32) {
    println!("The value of x is: {}, y is: {}", x, y);
}