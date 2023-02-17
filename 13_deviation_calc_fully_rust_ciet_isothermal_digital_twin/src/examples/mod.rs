#[warn(missing_docs)]
/// this is the most barebones basic
/// server, with two users and no anonymous connection,
/// you can't quite connect to it without some configuration
pub mod example1;
pub use example1::example_1_timer_server_no_connection;

/// example with anonymours server connection
pub mod example2;
pub use example2::*;


/// for example 3, we don't even need to configure user ids and passwords
/// we use the new_anonymous function to construct a server with no need for login
/// is quite similar to example 2, but uses
/// the new_anonymous associated function
pub mod example3;
pub use example3::*;
/// in example 4, we add a variable to 
/// the server, it is read only
pub mod example4;
pub use example4::*;
/// in example 5, we add two variables to the server, 
///
/// one is read only,
/// one is write only
pub mod example5;
pub use example5::*;
/// in example 6, we are making a kg to lbs converter
///
/// 1lb is 0.454 kg approx
/// this will be done using read and write variables
pub mod example6;
pub use example6::*;
/// example for testing polling action delays
pub mod example7;
pub use example7::*;

/// example for testing polling action delays
/// basically similar to example 7 but testing with different time
/// delays or thread::sleep()
pub mod example8;
pub use example8::*;

/// ciet server
pub mod ciet_server;
pub use ciet_server::*;

/// legacy ciet server with uncertainty
pub mod ciet_server_old_with_deviation;
pub use ciet_server_old_with_deviation::*;
