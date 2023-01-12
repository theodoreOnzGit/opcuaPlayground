#[warn(missing_docs)]
use local_ip_address::local_ip;
pub mod examples;
pub use examples::example1::*;
pub use examples::example2::*;

fn main() {
    println!("Hello, world!");


    let run_server = true;

    example_2_timer_server_auto_ip_addr_no_connection(run_server);


    

}


