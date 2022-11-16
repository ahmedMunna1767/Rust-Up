// fn main() {
//     println!("Hello, world!");
//     let mut s = String::from("Ahmed Bin");
//     s.push_str(" Nasser");
//     {
//         let s2 = s.clone();
//         println!("{}", s2);
//     }
//     println!("{}", s);
// }

fn main() {
    let s = String::from("hello");

    change(&mut s);

    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
