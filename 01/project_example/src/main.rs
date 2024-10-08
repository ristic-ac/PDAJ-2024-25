// Use the library crate defined in lib.rs
use project_example::network;
use project_example::utils;

fn main() {
    println!("This is the main binary crate.");
    // Call functions from the "utils" module
    utils::print_hello();
    utils::helper::print_help();

    // Call functions from the "network" module
    network::connect();
    network::server::start_server();
}
