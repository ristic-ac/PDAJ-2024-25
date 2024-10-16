fn main() {
    // Basic while loop
    let mut count = 0;
    while count < 5 {
        println!("Basic while loop, count: {}", count);
        count += 1;
    }

    // While loop with break
    let mut number = 10;
    while number > 0 {
        println!("While loop with break, number: {}", number);
        number -= 1;

        if number == 5 {
            println!("Breaking the loop as number reached 5");
            break; // Exit the loop when number is 5
        }
    }

    // While loop with continue
    let mut skip_number = 0;
    while skip_number < 5 {
        skip_number += 1;

        if skip_number == 3 {
            println!("Skipping number 3");
            continue;
        }

        println!("While loop with continue, skip_number: {}", skip_number);
    }

    // While loop that checks a condition from user input (simulated here)
    let mut user_input = 0;
    while user_input != 42 {
        user_input += 10; // Simulating user input increment
        println!("While loop, simulating user input: {}", user_input);
    }
    println!("Exited loop, user_input reached 42");

    // Nested while loops
    let mut outer = 1;
    while outer <= 3 {
        println!("Outer loop iteration: {}", outer);
        let mut inner = 1;
        while inner <= 2 {
            println!("  Inner loop iteration: {}", inner);
            inner += 1;
        }
        outer += 1;
    }
}
