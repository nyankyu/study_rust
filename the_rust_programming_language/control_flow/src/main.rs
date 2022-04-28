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

}
