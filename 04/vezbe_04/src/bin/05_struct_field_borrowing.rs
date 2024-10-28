struct Point { 
    x: i32, 
    y: i32 
}

fn main() {
    let mut p = Point { x: 0, y: 0 };  // `p` has full access here
    
    let x = &mut p.x;  // Mutably borrowing `p.x`
    
    // At this point:
    // - `p.x` is exclusively borrowed mutably, so only `x` can access it.
    // - `p` is partially restricted: cannot be accessed immutably because of the mutable borrow on `p.x`.
    // - `p.y` is unaffected; we can still access it independently.

    // This will not work:
    // let p_ref = &p;  // Immutable borrow on `p`
    // println!("p.x is not accessible: {}", p.x);  // `p.x` is not accessible 

    // We can access `p.y` without issues:
    println!("p.y is still accessible: {}", p.y);  // `p.y` can be accessed

    // Modify `p.x` through `x`:
    *x += 1;  // This works because `x` is a mutable reference to `p.x`

    println!("Updated p.x = {}", p.x); // `p.x` can be read after modifying
}
