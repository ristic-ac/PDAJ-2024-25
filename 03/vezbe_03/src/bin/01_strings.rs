fn main() {
    // Define the slice s, which refers the the string literal
    let s = "hello";
    // Valid until the end of the scope
    println!("{}", s);
    // Dropping the slice because it goes out of scope (isnt used anymore)

    // Define string s, which is mutable
    let mut s = String::from("hello"); // Includes pointer, len, cap

    // push_str() appends a literal to a String
    s.push_str(", world!");

    // This will print `hello, world!`
    println!("{s}");
}
