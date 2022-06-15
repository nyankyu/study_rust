use std::collections::HashMap;
use rand::{thread_rng, Rng};

fn main() {
    /*
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

    println!("=======================");
    */
    let mut list = [0i8; 64];
    thread_rng().fill(&mut list);
    //println!("{:?}", list);

    let (mean, median, mode) = make_mean_median_mode(&list);
    println!("mean:\t{}", mean);
    println!("median:\t{}", median);
    println!("mode:\t{}", mode);
}

fn make_mean_median_mode(list: &[i8]) -> (f32, i8, i8) {
    let mut v = Vec::from(list);
    let half_len = v.len() / 2;
    v.sort();
    println!("{:?}", v);

    let mut sum: i32 = 0;
    let mut h = HashMap::new();
    for i in &v {
        let count = h.entry(i).or_insert(0);
        *count += 1;
        sum += *i as i32;
    }

    (
        sum as f32 / list.len() as f32,
        *(v.get(half_len).unwrap()),
        **(h.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0)
    )
}
