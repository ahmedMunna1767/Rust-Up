use unit_testing::adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}