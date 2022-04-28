fn main() {
    let condition = true;
    let number = if condition {
        3
    } else {
        10
    };

    if number < 3 {
        println!("number is less than 3");
    } else if number == 3 {
        println!("number is 3");
    } else if number == 4 {
        println!("number is 4");
    } else {
        println!("number is greater then 4");
    }

    /*
    loop {
        println!("again!");
    }
    */
    let mut count = 10;
    while count != 0 {
        println!("{}", count);
        count = count - 1;
    }

    let array = [10, 20, 30, 40, 50];
    for elem in array.iter() {
        println!("the value is: {}", elem);
    }

    for n in (1..5).rev() {
        println!("{}", n);
    }
}
