fn not_main() {
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y} {x} {z}");

    let _five_hundred = tup.0;

    let _six_point_four = tup.1;

    let _one = tup.2;

    // let a: [i32; 5] = [1, 2, 3, 4, 5]; // array

    // let a = [3; 5]; // [3, 3, 3, 3, 3]

    // let y = {
    //     let x = 3;
    //     x + 1
    // }; // 4
}

fn plus_one(x: i32) -> (i32, i64) {
    return (x, (x * x).into());
}

fn control_flow() {
    let number = 6;
    let number = if number != 6 { 5 } else { 6 };
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn loops_in_rust() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
    let mut a = [10, 20, 30, 40, 50];

    a.reverse();
    for element in a {
        println!("the value is: {element}");
    }
}

fn range_for() {
    for number in (1..=4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    for_loop();
    while_loop();
    labels();
    loops_in_rust();
}

fn main() {
    range_for();
    let num = fib(10);
    println!("{num}");
    control_flow();
    plus_one(6);
    not_main();
}

fn fib(n: u8) -> u32 {
    if n <= 0 || n == 1 {
        return 0;
    } else if n == 2 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

// A scalar type represents a single value. Rust has four primary scalar types:
// integers, floating-point numbers, Booleans, and characters.
// You may recognize these from other programming languages. Let’s jump into how they work in Rust.
