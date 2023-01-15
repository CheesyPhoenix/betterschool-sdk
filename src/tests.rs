use super::*;

#[test]
fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}

#[test]
fn multiplication() {
    assert_eq!(6, multiply(3, 2))
}
