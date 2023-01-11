#[warn(missing_docs)]
use opcua::server::prelude::*;
use local_ip_address::local_ip;
pub mod examples;
pub use examples::example1::*;

fn main() {
    println!("Hello, world!");


    let run_server = false;

    example_1_timer_server_no_connection(run_server);

    // now i want to auto get ip addresses
    // i use the local ip address crate

    let my_local_ip = local_ip().unwrap();

    println!("This is my local IP address: {:?}", my_local_ip);

    // i can convert it to a string

    let ip_add_str : String = my_local_ip.to_string();

    println!("{}",ip_add_str);

    

}


