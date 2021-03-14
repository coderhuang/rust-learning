mod front_of_house;
mod back_of_house;

pub use crate::front_of_house::hosting;
pub use back_of_house::{Appetizer, Breakfast};

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    self::front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();

    let mut meal = Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;
}
