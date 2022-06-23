use std::{collections::HashMap, str::SplitWhitespace};

fn add_menber(menbers: &mut HashMap<String, Vec<String>>, iter: &mut SplitWhitespace) {
    let name;
    match iter.next() {
        None => return,
        Some(str) => name = str,
    }

    match iter.next() {
        None => return,
        Some(to) => if !to.eq("to") {
            return;
        },
    }

    let department;
    match iter.next() {
        None => return,
        Some(str) => department = str,
    }

    menbers
        .entry(String::from(department))
        .or_insert(vec![])
        .push(String::from(name));
}

fn display(menbers: &mut HashMap<String, Vec<String>>) {
    for department in menbers {
        println!("{}:", department.0);

        for name in department.1 {
            println!("\t- {}", name);
        }

        println!();
    }
}

fn command(menbers: &mut HashMap<String, Vec<String>>, com: &str) {
    let mut iter = com.split_whitespace();

    match iter.next() {
        Some("add") => add_menber(menbers, &mut iter),
        _ => println!("ERROR: "),
    }
}

fn main() {
    let mut menbers = HashMap::new();
    command(&mut menbers, "add Tom to engineering");
    command(&mut menbers, "add Alic to engineering");
    command(&mut menbers, "add Alic to Sales");

    display(&mut menbers);
}
