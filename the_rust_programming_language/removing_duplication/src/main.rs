
fn main() {
    let point = Point {x: 5, y: 10};
    println!("point is [x: {}, y: {}]", point.x, point.y);
    let point = Point {x: 1.0, y: -4.5};
    println!("point is [x: {}, y: {}]", point.x, point.y);
    let point = Point {x: 10, y: -4.5};
    println!("point is [x: {}, y: {}]", point.x, point.y);

    let h = Hoge::AAA(1, 'a');
    println!("hoge {:?}", h);
}

struct Point<T, U> {
    x: T,
    y: U,
}

#[derive(Debug)]
enum Hoge<T, U> {
    AAA(T, U),
    BBB,
}
