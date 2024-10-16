fn main() {
    // Simple infinite loop
    let mut counter = 0;
    loop {
        counter += 1;
        println!("Infinite loop, counter: {}", counter);

        if counter >= 5 {
            break; // Exit the loop when counter reaches 5
        }
    }

    // Loop with continue, skipping an iteration
    let mut number = 0;
    loop {
        number += 1;

        if number == 3 {
            println!("Skipping number 3");
            continue; // Skip the rest of this iteration
        }

        println!("Loop, number: {}", number);

        if number >= 5 {
            break; // Exit the loop when number reaches 5
        }
    }

    // Loop with a return value (using break to return a value)
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // Break and return counter * 2
        }
    };
    println!("Result from loop is: {}", result);

    // Nested loop with break
    let mut outer_count = 0;
    let mut inner_count = 0;
    loop {
        outer_count += 1;
        println!("Outer loop count: {}", outer_count);

        loop {
            inner_count += 1;
            println!("  Inner loop count: {}", inner_count);

            if inner_count >= 3 {
                break; // Exit inner loop when inner_count reaches 3
            }
        }

        if outer_count >= 2 {
            break; // Exit outer loop when outer_count reaches 2
        }
    }

    // Infinite loop without breaking (commented out to avoid actual infinite loop)
    /*
    loop {
        println!("This is an infinite loop");
    }
    */
}
