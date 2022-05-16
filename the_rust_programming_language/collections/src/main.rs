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

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(42),
        SpreadsheetCell::Float(12.345),
        SpreadsheetCell::Text(String::from("hoge")),
    ];
    println!("{:?}", row);

    let mut vv: Vec<i32> = Vec::with_capacity(5);
    vv.resize(5, 1);
    println!("{:?}", vv);
}
