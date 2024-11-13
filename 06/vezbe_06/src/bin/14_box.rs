fn main() {
    let x = 5;
    // x is copied onto the heap
    let y = Box::new(x);

    println!("Address of x: {:p}", &x);
    println!("Address of x within Box: {:p}", &*y);

    assert_eq!(5, x);
    // assert_eq!(5, y);
    assert_eq!(5, *y);
}
