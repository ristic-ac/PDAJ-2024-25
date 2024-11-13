fn main() {
    let x = 5;
    // x from the stack is referenced
    let y = &x;

    assert_eq!(5, x);
    // assert_eq!(5, y);
    assert_eq!(5, *y);
}