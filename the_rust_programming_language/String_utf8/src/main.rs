fn main() {
    let mut s = "hoge".to_string();
    s.push_str(" piyo");
    s.pop();
    s.push('!');

    let ss = s + &String::from("!??");

    //println!("{}", s);
    println!("{}", ss);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    //let s4 = s1 + "-" + &s2 + "-" + &s3;
    let s4 = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s4);
}
