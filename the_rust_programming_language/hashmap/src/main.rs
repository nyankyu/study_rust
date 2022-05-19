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
}
