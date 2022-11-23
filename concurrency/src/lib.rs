#![allow(unused)]
use core::time;
use std::{
    sync::{mpsc, Mutex},
    thread,
};

fn main_threads() {
    println!("Hello, world!");
    let my_thread = thread::spawn(|| {
        for i in 1..10 {
            println!("in thread one: {}", i);
            thread::sleep(time::Duration::from_secs(2));
        }
        println!("My thread completed its execution");
    });

    let my_thread_two = thread::spawn(|| {
        for i in 10..=15 {
            println!("in thread two: {}", i);
            thread::sleep(time::Duration::from_secs(1))
        }
        println!("my thread two completed its execution");
    });

    my_thread_two.join().unwrap();
    my_thread.join().unwrap();
}

fn main_move_ownership() {
    println!("Hello there");
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        thread::sleep(time::Duration::from_millis(10));
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn main_channel() {
    let (tx, rx) = mpsc::channel::<String>();

    let thread_one = thread::spawn(move || {
        let val = String::from("Ahmed Bin Nasser");
        tx.send(val).unwrap();
        tx.send(String::from("munna")).unwrap();
        println!("thread_one ending")
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    thread_one.join().unwrap();
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn main_multi_thread_communication_via_channel() {
    let (tx, rx) = mpsc::channel::<String>();
    let tx1 = tx.clone();
    let sender_thread = thread::spawn(move || {
        let vector = vec![
            String::from("Ahmed"),
            String::from("Bin"),
            String::from("Nasser"),
        ];

        thread::sleep(time::Duration::from_millis(500 * 2));
        for v in vector {
            tx.send(v).unwrap();
            thread::sleep(time::Duration::from_millis(500 * 2));
        }
    });

    let sender_thread_two = thread::spawn(move || {
        let values = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in values {
            tx1.send(val).unwrap();
            thread::sleep(time::Duration::from_secs(1));
        }
    });

    // sender_thread.join().unwrap();
    // drop(rx);

    // for msg in rx {
    //     print!("{} ", msg);
    // }

    let receiver_thread = thread::spawn(move || {
        // thread::sleep(time::Duration::from_millis(5000));

        for msg in rx {
            println!("{} ", msg)
        }
    });

    sender_thread.join().unwrap();
    receiver_thread.join().unwrap();
}

fn main_mutex() {
    let m = Mutex::new(5);
    {
        let mut n = m.lock().unwrap();
        *n = 6;
    }
    print!("{:#?}", m);
}
