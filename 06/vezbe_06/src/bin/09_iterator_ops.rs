fn filter_evens(numbers: Vec<i32>) -> Vec<i32> {
    numbers.into_iter().filter(|&x| x % 2 == 0).collect()
}

fn square_numbers(numbers: Vec<i32>) -> Vec<i32> {
    numbers.into_iter().map(|x| x * x).collect()
}

fn sum_numbers(numbers: Vec<i32>) -> i32 {
    numbers.into_iter().fold(0, |acc, x| acc + x)
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Step 1: Filter even numbers
    let evens = filter_evens(numbers.clone());
    println!("Even numbers: {:?}", evens);
    
    // Step 2: Square the filtered even numbers
    let squares = square_numbers(evens.clone());
    println!("Squared numbers: {:?}", squares);
    
    // Step 3: Sum the squared numbers
    let sum = sum_numbers(squares);
    println!("Sum of squared numbers: {}", sum);
}
