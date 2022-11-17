pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {
            println!("adding to wait list");
        }
    }
}

pub fn deliver_order() {
    println!("delivering an order");
}

// pub mod back_of_house {
//     pub fn fix_incorrect_order() {
//         cook_order();
//         super::deliver_order();
//     }

//     fn cook_order() {
//         println!("cooking order");
//     }
// }

// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_wait_list();

//     // Relative path
//     front_of_house::hosting::add_to_wait_list();
// }

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn new(toast: String, seasonal_fruit: String) -> Self {
            Self {
                toast,
                seasonal_fruit,
            }
        }

        pub fn summer(toast: &str) -> Breakfast {
            Self::new(String::from(toast), String::from("Peaches"))
        }

        pub fn list_ingredients(&self) {
            println!(
                "break fast with {} toast and {}.",
                self.toast, self.seasonal_fruit
            );
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

pub fn meal_detail() {
    let meal = back_of_house::Breakfast::summer("Rye");
    meal.list_ingredients();
}
