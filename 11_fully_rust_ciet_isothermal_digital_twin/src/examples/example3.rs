#[warn(missing_docs)]
use opcua::server::prelude::*;
use local_ip_address::local_ip;
use opcua::server::{state::ServerState, config};

pub fn example_3_timer_server_auto_ip_addr(run_server: bool){

    // for example 3, we don't even need to configure user ids and passwords
    // we use the new_anonymous function to construct a server with no need for login
    let server_builder = ServerBuilder::new_anonymous("test_server_3");
    let server_builder =
        server_builder.application_uri("urn:OPC UA Sample Server");

    // here's where i have a function to get my ip address
    let ip_address = get_ip_as_str();

    let server_builder = 
        server_builder.host_and_port(&ip_address, 4840);

    let is_server_valid = 
        server_builder.is_valid();

    println!("checking server validity..., found to be:");
    println!("{}",is_server_valid);




    // then we build the server

    let mut server = server_builder.server().unwrap();
    
    // step 2, u can add variables or nodes
    // which now i am not adding
    //
    // step 3 is to add closures (functions) which you want to do
    // every second
    // use the server.add_polling_action(interval in ms, closure);
    // in order to define an action that is done every second
    //

    // lets have a closure that activates every 2s

    let timer = || {

        println!("\n hello there!, the time in utc now is : \n");

        use chrono::Utc;
        println!("{}",Utc::now());
        println!("{}",DateTime::now());

    };

    server.add_polling_action(2000, timer);



    let print_endpoint_simple = || {
        let ip_add = get_ip_as_str();

        println!("\n opc.tcp://{}:{}{} \n",ip_add,4840,"/");
    };

    //server.add_polling_action(5000, print_endpoint);
    server.add_polling_action(5000, print_endpoint_simple);

    // step 3: when you finish configuring the server, tasks and etc
    // run the server
    //
    // the server should run, but it does not really allow connections
    //

    // runs server if the user wants to
    if run_server == true {
        server.run();
    }

    // let's also have something to print the endpoint


}

fn get_ip_as_str() -> String {

    let my_local_ip = local_ip().unwrap();

    println!("This is my local IP address: {:?}", my_local_ip);

    // i can convert it to a string

    let ip_add_string : String = my_local_ip.to_string();

    println!("{}",ip_add_string);

    return ip_add_string;

}
