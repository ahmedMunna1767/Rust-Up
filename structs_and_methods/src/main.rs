struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("abc"),
        email: String::from("abc@example.com"),
        sign_in_count: 1,
    };
    let user2 = build_user(user1.email, user1.username);

    println!("Hello, world!");
    println!(
        "New user created with name: {}, email: {}",
        user2.username, user2.email
    );

    let user3 = User {
        email: String::from("another@example.com"),
        ..user2 // take rest of the fields from user2
    };

    println!(
        "{}-{}-{}-{}",
        user3.active, user3.email, user3.username, user3.sign_in_count
    );

    let red = Color { 0: 255, 1: 0, 2: 0 };
    println!("R: {}, G: {}, B: {}", red.0, red.1, red.2);

    let xyz = Point { 0: 1, 1: 2, 2: 3 };
    println!("X: {}, Y: {}, Z: {}", xyz.0, xyz.1, xyz.2);

    let black = Color(0, 0, 0);
    println!("R: {}, G: {}, B: {}", black.0, black.1, black.2);

    let origin = Point(0, 0, 0);
    println!("X: {}, Y: {}, Z: {}", origin.0, origin.1, origin.2);
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("{:?}", rect1);

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let sq = Rectangle::square(10);
    println!("{:?}, area: {}", sq, sq.area());
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
