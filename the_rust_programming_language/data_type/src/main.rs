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
}
