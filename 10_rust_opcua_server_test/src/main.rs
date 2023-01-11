use opcua::server::prelude::*;

fn main() {
    println!("Hello, world!");

    // let's build a new server
    // step 1 is to congifure the server builder

    let server_builder = ServerBuilder::new();

    // server_builder construction
    // note that each time we use one of these methods,
    // it consumes self and returns self
    // so we need to keep binding the output back to server builder
    let server_builder = 
        server_builder.application_name("test server_builder");

    // set the application uri, note that
    // it is NOT the discovery endpoint
    // also it consumes
    let server_builder =
        server_builder.application_uri("urn:OPC UA Sample Server");

    let server_builder =
        server_builder.discovery_urls(
            vec![server_url_from_endpoint_url("opc.tcp://127.0.0.1:4855/").unwrap()]);


    // then we build the server
    
    // step 2, u can add variables or nodes
    //
    // step 3 is to add closures (functions) which you want to do
    // every second
    // use the server.add_polling_action(interval in ms, closure);
    // in order to define an action that is done every second
    //



    // step 3: when you finish configuring the server, tasks and etc
    // run the server

}

