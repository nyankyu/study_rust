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
}
