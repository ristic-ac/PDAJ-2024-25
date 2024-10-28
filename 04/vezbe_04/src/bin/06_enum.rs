// This enumeration is used to define the type of IP address.
enum IpAddrKind {
    V4,
    V6,
}

// This struct is used to define the IP address.
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    match home.kind {
        IpAddrKind::V4 => println!("Home address is: V4"),
        IpAddrKind::V6 => println!("Home address is: V6"),
    }
    
}