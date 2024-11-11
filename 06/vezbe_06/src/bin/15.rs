struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


fn main() {
    let x = 5;
    let y = MyBox::new(x);

    println!("Address of x: {:p}", &x);
    println!("Address of x within MyBox: {:p}", &*y);

    assert_eq!(5, x);
    // *(y) => *(y.deref())  This applies just once
    assert_eq!(5, *y);
}