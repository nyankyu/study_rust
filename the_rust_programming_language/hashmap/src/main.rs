use rand::{distributions::Uniform, thread_rng, Rng};
use std::collections::HashMap;

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

    let distr = Uniform::from(-50..50);
    let list: Vec<i32> = thread_rng().sample_iter(distr).take(100).collect();
    println!("{:?}", list);

    println!("mean:\t{}", mean(&list));
    println!("median:\t{}", median(&list));
    println!("mode:\t{}", mode(&list));
}

fn mean(list: &Vec<i32>) -> f32 {
    let sum: i32 = list.iter().sum();
    sum as f32 / list.len() as f32
}

fn median(list: &Vec<i32>) -> i32 {
    let mut list_ = list.to_vec();
    list_.sort_unstable();
    //println!("{:?}", list_);
    let half_len = list_.len() / 2;
    list_[half_len]
}

fn mode(list: &Vec<i32>) -> i32 {
    let mut frequency = HashMap::new();
    for i in list {
        let count = frequency.entry(i).or_insert(0);
        *count += 1;
    }
    **(frequency.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0)
}
