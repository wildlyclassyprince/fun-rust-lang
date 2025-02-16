mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Testing use of super - absolute path
    crate::back_of_house::fix_incorrect_order();

    // Testing use of super - relative path
    back_of_house::fix_incorrect_order();

    // Testing constructing public struct - public fields need to be annotated
    // Order a breakfast with rye toast
    let mut meal = crate::back_of_house::Breakfast::summer("Rye");
    // Change your mind and order wheat toast instead
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal,
    // unless we make the seasonal_fruit field public as well
    meal.seasonal_fruit = String::from("Blueberries");

    // If you make an enum public, all of its variants are then public
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    println!("Available lunch appetizers are {order1:?} and {order2:?}");

}

// Demonstrating how to use super
fn deliver_order() {}

mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        pub seasonal_fruit: String, // remove public annotation and see what happens!
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
