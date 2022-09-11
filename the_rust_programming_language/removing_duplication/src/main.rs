fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!("list is {:?}", number_list);
    println!("The largest number is {}", largest(&number_list));

    let number_list = vec![34, 50, 25, -100, 65];
    println!("list is {:?}", number_list);
    println!("The largest number is {}", largest(&number_list));
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
