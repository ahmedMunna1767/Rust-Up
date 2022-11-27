// advanced traits

pub mod function_pointers;
pub mod method_overloading;
pub mod operator_overloading;
pub mod super_traits;
pub mod wrapper_type;

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct UintCounter {
    count: u32,
}

impl UintCounter {
    fn new() -> UintCounter {
        UintCounter { count: 0 }
    }
}

struct IntCounter {
    c: i32,
}

impl IntCounter {
    fn new() -> IntCounter {
        IntCounter { c: 0 }
    }
}

impl Iterator for IntCounter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
        if self.c < 10 {
            self.c = self.c + 10;
            Some(self.c)
        } else {
            None
        }
    }
}

impl Iterator for UintCounter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn test_advanced_traits() {
    let mut c = UintCounter::new();
    println!("{}", c.next().unwrap());

    let mut c = IntCounter::new();
    println!("{}", c.next().unwrap());
}
