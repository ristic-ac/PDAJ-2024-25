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
    let penny = Coin::Penny;
    let quarter = Coin::Quarter(UsState::Alaska);
    let mut count = 0;

    match &penny {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }

    if let Coin::Quarter(state) = &penny {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

    if let Coin::Quarter(state) = penny {
        println!("State quarter from {state:?}!");
    } 

    if let Coin::Quarter(state) = quarter {
        if let UsState::Alaska = state {
            println!("State quarter from Alaska!");
        } else {
            count += 1;
        }
    } else {
        count += 1;
    }
    // println!("Quarter: {:?}", quarter);
    println!("Count: {}", count);
}