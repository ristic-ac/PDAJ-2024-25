fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    let i1 = 5;
    let i2 = &i1;
    let i3 = *i2; // Value is copied
    println!("The value of i1 is {i1}.");
    println!("The value of i2 is {i2}.");
    println!("The value of i3 is {i3}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
