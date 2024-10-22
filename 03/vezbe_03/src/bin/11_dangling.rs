fn main() {
    // let reference_to_nothing = dangle();
    let reference_to_something = no_dangle();
}

// fn dangle() -> &str {
//     let s = String::from("hello");

//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
