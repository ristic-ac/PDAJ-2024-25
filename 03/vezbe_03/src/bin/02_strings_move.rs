fn main() {
    // Integers implement the Copy trait, so the value is copied
    let x = 5;
    let y = x;
    println!("{}", x);
    println!("{}", y);

    // Strings do not implement the Copy trait, so the value is moved
    let s1 = String::from("hello");

    let s2 = s1;
    // let s2 = s1.clone();
    // println!("{s1}, world!");
    println!("{s2}, world!");
}
