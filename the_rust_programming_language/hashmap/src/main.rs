use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 25);

    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yelow")];
    let initial_scores = vec![10, 25];

    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores2);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    //println!("{}, {}", field_name, field_value);
    println!("{:?}", map);

    let mut int_str_map: HashMap<i32, &String> = HashMap::new();
    let s = String::from("hoge");
    int_str_map.insert(10, &s);
    println!("{}", s);
    println!("{:?}", int_str_map);

    let score = scores.get(&String::from("Blue"));
    println!("{}", score.unwrap());

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("Yellow"), 20);
    println!("{:?}", scores);

    scores.entry(String::from("White")).or_insert(60);
    scores.entry(String::from("Blue")).or_insert(300);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut char_map = HashMap::new();
    for c in text.chars() {
        let count = char_map.entry(c).or_insert(0);
        *count += 1;
    }
    println!("{:?}", char_map);
}
