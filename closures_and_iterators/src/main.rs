#![allow(unused)]
mod iter;
mod filter_iter;

#[derive(Debug,Clone,Copy)]
enum ShirtColor {
    Red,
    Blue
}

struct Inventory {
    shirts: Vec<ShirtColor>, 
}

impl Inventory {
    fn give_away(&self, user_preference: Option<ShirtColor>) -> ShirtColor{
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0; 
        let mut num_blue = 0; 
        for shirt in &self.shirts{
            match shirt {
                ShirtColor::Red => num_red = num_red + 1,
                ShirtColor::Blue => num_blue = num_blue + 1,
            }
        }

        if num_red > num_blue {
            return  ShirtColor::Red;
        }else {
            return  ShirtColor::Blue;
        }
    }
}

fn main(){
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.give_away(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.give_away(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
    sort_by_key();
}
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
// let add_one_v2 = |x: u32| -> u32 { x + 1 };
// let add_one_v3 = |x: u32|             { x + 1 };
// let add_one_v4 = |x: u32|               x + 1  ;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn sort_by_key() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r|-> u32 {r.width} );
    println!("{:#?}", list);
}

// FnOnce applies to closures that can be called once. All closures implement at least this trait, because all closures can be called. A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.
// FnMut applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
// Fn applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.