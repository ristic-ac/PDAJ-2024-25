fn main() {
    let b = Box::new(5);
    println!("b = {b}");

    // Dereferencing the Box to access the value
    let deref_b = *b;
    println!("Dereferenced b = {deref_b}");

    // Modifying the value inside the Box
    let mut b = Box::new(10);
    *b = 20;
    println!("Modified b = {b}");
    
    // Using Box with a more complex data type (a tuple)
    let complex_box = Box::new((1, "hello"));
    println!("complex_box = {:?}", complex_box);
}
