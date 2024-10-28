fn demonstrate_placeholder() {
    // opt has          Read and Own permissions
    // opt@Some.0 has   Read and Own permissions
    let opt: Option<String> = 
        Some(String::from("Hello world"));
    
    match opt {
        Some(_) => println!("Some!"),   // Because placeholder is used, opt@Some.0 is not consumed
        None => println!("None!")
    };

    match opt {
        Some(_) => println!("Some!"),   // Because placeholder is used, opt@Some.0 is not consumed
        None => println!("None!")
    };
    
    println!("{:?}", opt);
}

// fn demonstrate_moving() {
//     // opt has          Read and Own permissions
//     // opt@Some.0 has   Read and Own permissions
//     let opt: Option<String> = 
//         Some(String::from("Hello world"));
    
//     match opt {
//         Some(value) => println!("{}", value),   // Because value is caught as value, opt@Some.0 is consumed
//         None => println!("None!")
//     };

//     match opt {
//         // Some(_) => println!("Proba"),   
//         Some(value) => println!("{}", value),   
//         None => println!("None!")
//     };

//     println!("{:?}", opt);
// }

fn demonstrate_borrowing_1() {
    // opt has          Read and Own permissions
    // opt@Some.0 has   Read and Own permissions
    let opt: Option<String> = 
        Some(String::from("Hello world"));
    
    // This is called "reference push down"
    match &opt {
        Some(value) => println!("{}", value),   // Because reference is used, opt@Some.0 is not consumed
        None => println!("None!")
    };

    match &opt {
        Some(value) => println!("{}", value),   // Because reference is used, opt@Some.0 is not consumed
        None => println!("None!")
    };

    println!("{:?}", opt);
}

fn demonstrate_borrowing_2() {
    // opt has          Read and Own permissions
    // opt@Some.0 has   Read and Own permissions
    let opt: Option<String> = 
        Some(String::from("Hello world"));
    
    match opt {
        Some(ref value) => println!("{}", value),   // Because ref is used, opt@Some.0 is not consumed
        None => println!("None!")
    };

    match opt {
        Some(ref value) => println!("{}", value),   // Because ref is used, opt@Some.0 is not consumed
        None => println!("None!")
    };

    println!("{:?}", opt);
}

fn main() {
    demonstrate_placeholder();
    // demonstrate_moving();
    demonstrate_borrowing_1();
    demonstrate_borrowing_2();
}