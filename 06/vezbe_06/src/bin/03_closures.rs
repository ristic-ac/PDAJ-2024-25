#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // 1. Demonstrating FnOnce with a closure that moves a captured value
    let value = String::from("I will be moved");

    let closure_once = move || {
        println!("FnOnce closure called: {}", value);
    };

    // The closure will be called once and move the value out of the environment
    closure_once(); // This will work
    closure_once(); // This works as well

    // let closure_twice = move || { // Error: value has been moved
    //     println!("FnOnce closure called: {}", value);
    // };
    // println!("value after move: {}", value);  // Error: value has been moved
    // 2. Demonstrating FnMut with a closure that mutates a captured value
    let mut counter = 0;

    let closure_mut = |r: &Rectangle| {
        counter += 1;
        println!("FnMut closure called {} times, current rectangle width: {}", counter, r.width);
        r.width
    };

    let mut rectangles = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    // `sort_by_key` requires an `FnMut` closure because it calls the closure multiple times
    rectangles.sort_by_key(closure_mut);
    println!("Sorted rectangles: {:#?}", rectangles);
    println!("Total operations: {}", counter);

    // 3. Demonstrating Fn with a closure that does not mutate or move any captured values
    let closure_fn = |r: &Rectangle| r.width;
    
    // This closure does not mutate or move anything, so it can be used in contexts requiring `Fn`
    let width = closure_fn(&rectangles[0]);
    println!("Width of the first rectangle: {}", width);

    // 4. Example showing the behavior of `unwrap_or_else` with a `FnOnce` closure
    let some_option = Some(42);
    let none_option: Option<i32> = None;

    let result_some = some_option.unwrap_or_else(|| {
        println!("This will not be called because the Option is Some");
        100
    });

    let result_none = none_option.unwrap_or_else(|| {
        println!("This will be called because the Option is None");
        100
    });

    println!("Result of unwrap_or_else with Some: {}", result_some);
    println!("Result of unwrap_or_else with None: {}", result_none);
}
