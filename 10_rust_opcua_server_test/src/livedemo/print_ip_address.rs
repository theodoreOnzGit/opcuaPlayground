use opcua::server::prelude::*;
use local_ip_address::local_ip;


pub fn demo_2_print_ip_address(run_server: bool) {


    let server_builder = ServerBuilder::new();

    let server_builder = 
        server_builder.application_name("my opcua demo server");

    // we will write it the truncated way from now on...

    // you can use a localhost ip address if you don't want to
    // conenct to another pc:
    // ipv4:
    // 127.0.0.1

    let ip_address = get_ip_as_str();
    let port_number: u16 = 4840;


    let server_builder = 
        server_builder.application_uri("urn:OPC UA Sample Server")
        .host_and_port(&ip_address,port_number);


    let EXAMPLE_ENDPOINT_PATH: &str = 
        "/demo_server";


    // adding an anonymous user...
    let user_token_ids = vec![ANONYMOUS_USER_TOKEN_ID.to_string()];

    // add endpoint

    let server_builder = 
        server_builder.endpoint(
            "none",
            ServerEndpoint::new_none(EXAMPLE_ENDPOINT_PATH, &user_token_ids),
            );

    let server_builder = 
        server_builder.discovery_urls(
            vec![
            EXAMPLE_ENDPOINT_PATH.into()
            ]);

    let mut server = server_builder.server().unwrap();

    // let's add a closure to print ip address
    let print_endpoint = move || {

        println!("opc.tcp://{}:{:?}{} \n",
                 &ip_address,
                 port_number,
                 EXAMPLE_ENDPOINT_PATH);
    };
    
    server.add_polling_action(3000, print_endpoint);

    if run_server == true  {
        server.run();
    }
}


fn get_ip_as_str() -> String {

    let my_local_ip = local_ip().unwrap();

    // i can convert it to a string

    let ip_add_string : String = my_local_ip.to_string();


    return ip_add_string;

}

