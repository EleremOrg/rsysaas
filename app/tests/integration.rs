mod common;

fn add_two(u: usize) -> usize {
    u
}

#[test]
fn it_adds_two() {
    common::setup();

    let result = add_two(2);
    assert_eq!(result, 4);
}
