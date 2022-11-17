use lib_crate::{eat_at_restaurant, front_of_house, meal_detail};

fn main() {
    eat_at_restaurant();
    println!("<- ahmed bin nasser ->");
    front_of_house::hosting::add_to_wait_list();
    // back_of_house::fix_incorrect_order();
    eat_at_restaurant();
    meal_detail();
}
