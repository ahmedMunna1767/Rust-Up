#![allow(unused)]
fn main_one() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    println!("{:?}", v);

    print_vector(&v);
    v.reverse();
    print_vector(&v);

    let third = &v[2];
    println!("{third}");

    get_index(&v, 2);
    get_index(&v, 4);

    for i in &mut v {
        *i += 50;
    }

    print_vector(&v);
}

fn get_index(v: &Vec<i32>, index: usize) {
    let third = v.get(index);
    match third {
        Some(v) => println!("{v}"),
        None => println!("highest index available is {}", v.len() - 1),
    }
}

fn print_vector(v: &Vec<i32>) {
    for a in v {
        print!("{} ", a);
    }
    println!()
}
use std::collections::HashMap;
fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}-> s1 is {s1}", s2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("{s}, {s1}, {s2}");

    for c in "Зд".chars() {
        println!("{}", c);
    }

    for b in "Зд".bytes() {
        println!("{}", b);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
