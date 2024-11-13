use std::cell::RefCell;

fn main() {
    // Immutable Box containing a RefCell, which provides interior mutability
    let immutable_box = Box::new(RefCell::new(5));

    // We can't mutate `immutable_box` itself, but we can modify its contents
    *immutable_box.borrow_mut() += 1;

    // Access the value through a borrow
    println!("The value inside the immutable box is: {}", immutable_box.borrow());
}
