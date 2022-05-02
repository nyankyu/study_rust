fn main() {
    let mut s1 = String::from("hello");
    let mut s2 = String::from("hello");
    //let len = calculate_length(&mut s1, &mut s1);
    let len = calculate_length(&mut s1, &mut s2);
    println!("The length of '{}' is {}.", s1, len);

    let mut a = String::from("hoge");
    let r1 = &a;
    let r2 = &a;
    let r3 = &mut a;
    //println!("{}", r1);

    let reference_to_nothing = dangle();
}

fn calculate_length(s1: &mut String, s2: &mut String) -> usize {
    s1.push_str(", world!");
    s2.push_str(", world!");
    s1.len() + s2.len()
}

fn dangle() -> String {
    String::from("hoge")
}
