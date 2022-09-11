
fn main() {
    let point = Point {x: 5, y: 10};
    println!("point is [x: {}, y: {}]", point.x(), point.y());
    let point = Point {x: 1.0, y: -4.5};
    println!("point is [x: {}, y: {}]", point.x(), point.y());
    println!("distance is {}", point.distance_from_origin());
    let point = Point {x: 10, y: -4.5};
    println!("point is [x: {}, y: {}]", point.x(), point.y());

    let p = Point { x: "hello", y: "world"};
    let pp = point.mixup(p);
    println!("point is [x: {}, y: {}]", pp.x(), pp.y());

    let h = Hoge::AAA(1, 'a');
    println!("hoge {:?}", h);
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point { x: self.x, y: other.y }
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
enum Hoge<T, U> {
    AAA(T, U),
    BBB,
}
