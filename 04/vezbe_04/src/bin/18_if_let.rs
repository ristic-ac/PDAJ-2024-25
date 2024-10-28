#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Penny;
    let mut count = 0;

    match &coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }

    if let Coin::Quarter(state) = &coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } 
    println!("Count: {}", count);
}