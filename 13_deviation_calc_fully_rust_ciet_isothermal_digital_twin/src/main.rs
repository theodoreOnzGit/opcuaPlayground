#[warn(missing_docs)]

/// this module holds some examples for basic opcua servers
/// no ciet or anything there
pub mod examples;
pub use examples::ciet_server_old_with_deviation::*;

pub mod ciet_libraries;
pub use ciet_libraries::*;


use crate::examples::ciet_server;
use crate::examples::ciet_server_old_with_deviation;


fn main() {
    println!("Hello, world!");


    let run_server = true;
    let dont_run_server = false;




    ciet_server::construct_and_run_ciet_server(dont_run_server);
    ciet_server_old_with_deviation::construct_and_run_ciet_server(run_server);

}


#[cfg(test)]
pub mod tests_and_examples; 
