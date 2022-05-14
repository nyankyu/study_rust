#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // etc..
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let my_coin = Coin::Quarter(UsState::Alabama);
    println!("{}", value_in_cents(&my_coin));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", six);
    println!("{:?}", none);

    let number = 2;
    match number {
        0 => println!("{} is Favorite Number!", number),
        3 => println!("{} is Favorite Number!", number),
        5 => println!("{} is Favorite Number!", number),
        7 => println!("{} is Favorite Number!", number),
        _ => println!("{} is NOT Favorite Number!", number),
    }

    //let some_u8_value = None;
    let some_u8_value = Some(5u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(i) = some_u8_value {
        println!("value is {}", i);
    } else {
        println!("None");
    }
}
