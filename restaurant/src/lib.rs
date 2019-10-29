/*#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}*/

mod front_of_house {
    pub mod hosting {  // needs to be public
        pub fn add_to_waitlist() {}  // needs to be public

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        pub fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();  // Use super to get to the root of module
    }

    fn cook_order() {}

    // Each field of a struct needs to be made public to be used elsewhere
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // Each variant of an enum becomes public by making the name public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// Use `use` so you don't have to specify the absolute or relative path all the time
// Also, adding the `pub` allows others to use this 
pub use crate::front_of_house::hosting:

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Using the `use` path
    hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind on the toast
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // We can only modify `toast`, not `seasonal_fruit`

    // Use a public enum
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

}
