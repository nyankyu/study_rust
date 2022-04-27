fn main() {
    let integer_i8: i8 = -42;
    let integer_i64: i64 = 12345;
    let integer_usize: usize = 0b11001101;
    println!("integer i8: {}", integer_i8);
    println!("integer i64: {}", integer_i64);
    println!("integer usize: {}", integer_usize);

    println!("hoge {}", -1234i32);

    let float_32: f32 = -42.001;
    println!("float32: {}", float_32);

    let flag = true;
    let f: bool = flag;
    println!("flag: {}", f);

    let c: char = 'a';
    let c = '🍻';
    let c = 'β';
    println!("char: {}", c);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup_copy = tup;
    println!("tup.0: {}", tup_copy.0);
    println!("tup.1: {}", tup_copy.1);
    println!("tup.2: {}", tup_copy.2);

    let months = [
        "",
        "January",
        "Februaty",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("month: {}", months[5]);

    let a = [1, 2, 3, 4, 5];
    let index = 10;
    let element = a[index];
    println!("The value of element is: {}", element);
}
