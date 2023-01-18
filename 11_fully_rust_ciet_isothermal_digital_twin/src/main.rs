#[warn(missing_docs)]

/// this module holds some examples for basic opcua servers
/// no ciet or anything there
pub mod examples;
pub use examples::example1::*;
pub use examples::example2::*;
pub use examples::example3::*;
pub use examples::example4::*;
pub use examples::example5::*;
pub use examples::example6::*;
pub use examples::example7::*;
pub use examples::example8::*;

pub mod ciet_libraries;
pub use ciet_libraries::*;

use crate::examples::ciet_server::construct_and_run_ciet_server;


fn main() {
    println!("Hello, world!");


    let run_server = true;
    let dont_run_server = false;

    //example_3_timer_server_auto_ip_addr(dont_run_server);
    //example_4_timer_server_auto_ip_addr(dont_run_server);
    //example_5_read_and_write_variables(run_server);
    //example_6_read_and_write_variables(run_server);
    //example_7_check_polling_action_delays(run_server);
    //example_8_check_polling_action_delays_two(run_server);



    construct_and_run_ciet_server(run_server);

}


