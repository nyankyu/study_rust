fn main() {
    let mut v = vec![0, 1, 2, 3];

    let third: &i32 = &v[2];
    //v.push(10);
    //v[0] = 10;
    println!("The third element is {}", third);

    match v.get(2) {
        Some(value) => println!("The third element is {}", value),
        None => println!("There is no third element."),
    }

    for i in &mut v {
        *i *= 10;
    }
    println!("{:?}", v);
}
