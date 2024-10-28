#[derive(Debug,Clone)]
enum IpAddr {
    V4(u8, u8, u8, u8), // Tuple struct
    V6(String),         // Tuple struct
}
fn main() {
    
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    let home2 = home.clone(); 
    println!("{:?}", home);
}