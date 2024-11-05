fn main() {
    // 1. Immutable binding to an immutable reference
    let x = 10;           // `x` is an immutable binding to the value 10
    let y = &x;           // `y` is an immutable reference to `x`
    println!("y (immutable binding to immutable reference): {}", y);

    // Attempting to reassign `y` would cause a compile-time error:
    // y = &20; // Error: cannot assign to `y` because it is an immutable binding

    // Attempting to modify the value of `x` through `y` would also cause an error:
    // *y = 20; // Error: cannot assign to `*y` which is behind an `&` reference

    // 2. Mutable binding to an immutable reference
    let mut y_ref = &x;   // `y_ref` is a mutable binding to an immutable reference of `x`
    println!("y_ref (initial mutable binding to immutable reference): {}", y_ref);

    // Since `y_ref` is a mutable binding, we can reassign it to point to a different reference:
    y_ref = &20;          // Allowed: `y_ref` can be reassigned to a different reference
    println!("y_ref (reassigned to new immutable reference): {}", y_ref);

    // However, we still cannot modify the value of `x` through `y_ref`:
    // *y_ref = 20; // Error: cannot assign to `*y_ref` which is behind an immutable reference

    // 3. Immutable binding to a mutable reference
    let mut z = 30;       // `z` is a mutable binding to the value 30
    let z_ref = &mut z;   // `z_ref` is an immutable binding to a mutable reference of `z`
    *z_ref += 10;         // Allowed: modifies `z` through `z_ref`
    println!("z (after modification through immutable binding to mutable reference): {}", z);

    // Attempting to reassign `z_ref` to another mutable reference would cause an error:
    // z_ref = &mut z; // Error: cannot assign to immutable variable `z_ref`

    // 4. Mutable binding to a mutable reference
    let mut a = 40;       // `a` is a mutable binding to the value 40
    let mut a_ref = &mut a; // `a_ref` is a mutable binding to a mutable reference of `a`
    *a_ref += 10;         // Allowed: modifies `a` through `a_ref`
    println!("a (after modification through mutable binding to mutable reference): {}", a);

    // Since `a_ref` is a mutable binding, we can reassign it to point to another mutable reference:
    a_ref = &mut a;       // Allowed: `a_ref` can be reassigned to a new mutable reference
    *a_ref += 10;         // Allowed: modifies `a` again through `a_ref`
    println!("a (after reassignment and modification through mutable binding to mutable reference): {}", a);
}
