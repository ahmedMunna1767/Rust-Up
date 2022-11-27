use std::slice;

use unsafe_rust::{
    function_pointers::one_two, method_overloading::test_method_overloading,
    operator_overloading::test_operator_overloading, super_traits::test_super_traits,
    test_advanced_traits, wrapper_type::test_wrapper_trait,
};

fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // let address = 0x012345usize;
    // let r = address as *const i32;

    println!("{}", num);
    unsafe {
        *r2 = *r2 + 1;
        println!("{}--{}", *r1, *r2);
        // println!("unknown address: {}", *r);
    }
    println!("{}", num);

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    let (a, b) = split_at_mut(&mut v, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    add_to_count(2);
    unsafe {
        println!("{}", COUNTER);
    }

    test_advanced_traits();
    test_operator_overloading();
    test_method_overloading();
    test_super_traits();
    test_wrapper_trait();
    one_two();
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
