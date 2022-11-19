mod adder;
use adder::{ Rectangle, guess::{ Guess } };
fn main() {
    let rect = Rectangle { width: 30, height: 30 };
    let can_hold = rect.can_hold(
        &(Rectangle {
            width: 20,
            height: 20,
        })
    );
    println!("{}", can_hold);

    let _g: Guess = Guess::new(100);
}