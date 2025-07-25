use adder::add_two;
mod common;
#[test]
fn it_adds_two() {
    let result = add_two(2);
    assert_ne!(result, 4);
}
