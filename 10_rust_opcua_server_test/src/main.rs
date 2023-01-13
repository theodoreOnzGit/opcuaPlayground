#[warn(missing_docs)]
use local_ip_address::local_ip;
pub mod examples;
pub use examples::example1::*;
pub use examples::example2::*;
pub use examples::example3::*;
pub use examples::example4::*;
pub use examples::example5::*;


fn main() {
    println!("Hello, world!");


    let run_server = true;
    let dont_run_server = false;

    example_3_timer_server_auto_ip_addr(dont_run_server);
    example_4_timer_server_auto_ip_addr(dont_run_server);
    example_5_read_and_write_variables(run_server);

    

}


