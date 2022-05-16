mod front_of_house;
mod back_of_house;

pub use crate::front_of_house::hosting;

use crate::hosting::seat_at_table;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    seat_at_table();

    use back_of_house::Breakfast;

    let mut meal = Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please.", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
