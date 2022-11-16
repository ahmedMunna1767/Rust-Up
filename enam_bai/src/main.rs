#![allow(unused)]

mod concise_control_flow;

enum IPAddrKind {
    V4(String),
    V6(String),
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

/// Quit has no data associated with it at all.
/// Move has named fields like a struct does.
/// Write includes a single String.
/// ChangeColor includes three i32 values.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("inside the call function");
        match self {
            Message::Quit => todo!(),
            Message::Move { x, y } => todo!(),
            Message::Write(value) => {
                println!("{value}");
                return;
            }
            Message::ChangeColor(_, _, _) => todo!(),
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    let four = IPAddrKind::V4(String::from("127.0.0.1"));
    let six = IPAddrKind::V6(String::from("::1"));

    let message = Message::Write(String::from("Munna"));
    message.call();

    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    match some_number {
        Some(val) => println!("some_number has value {val}"),
        None => println!("some_number doesn't have a value"),
    }

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y.unwrap();
    println!("{sum}");
    concise_control_flow::if_let();
}

fn dice_roll() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}

fn dice_roll_two() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // nothing happens if value is other than 3 / 7
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}

// Tony Hoare, the inventor of null, has this to say
// I call it my billion-dollar mistake. At that time,
// I was designing the first comprehensive type system
// for references in an object-oriented language.
// My goal was to ensure that all use of references
// should be absolutely safe, with checking performed
// automatically by the compiler. But I couldn’t resist
// the temptation to put in a null reference,
// simply because it was so easy to implement.
// This has led to innumerable errors, vulnerabilities,
// and system crashes, which have probably caused a billion
// dollars of pain and damage in the last forty years.
