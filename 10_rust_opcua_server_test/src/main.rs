#[warn(missing_docs)]
use local_ip_address::local_ip;
pub mod examples;
pub use examples::example1::*;
pub use examples::example2::*;
pub use examples::example3::*;

fn main() {
    println!("Hello, world!");


    let run_server = true;

    example_3_timer_server_auto_ip_addr(run_server);


    

}


