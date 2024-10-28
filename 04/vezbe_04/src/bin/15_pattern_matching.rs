// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//         None => None,
//     }
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//     }
// }

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None,
    }
}
fn main() {

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", five);
    println!("{:?}", six);
    println!("{:?}", none);
}