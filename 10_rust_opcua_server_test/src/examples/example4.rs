#[warn(missing_docs)]
use opcua::server::prelude::*;
use local_ip_address::local_ip;
use opcua::server::{state::ServerState, config};

pub fn example_4_timer_server_auto_ip_addr(run_server: bool){

    // in example 4, we add a variable to 
    // the server, it is read only

    let server_builder = ServerBuilder::new();

    let server_builder = 
        server_builder.application_name("test server_builder");

    let server_builder =
        server_builder.application_uri("urn:OPC UA Sample Server");

    // previously we couldn't connect to this address, plus the port 
    // at this place was often used, i'll probably get a better endpoint
    // like /my_rust_opcua_server


    const CUSTOM_ENDPOINT_PATH: &str = "/my_rust_opcua_server";

    // and i'll use a function to automatically get my local ip address
    // (ipv4)
    let ip_address = get_ip_as_str();

    let server_builder = 
        server_builder.host_and_port(&ip_address, 4840);


    let server_builder =
        server_builder.discovery_urls(
            vec![
            CUSTOM_ENDPOINT_PATH.into(),
            ]);


    // username and password

    let user_id_anonymous = config::ANONYMOUS_USER_TOKEN_ID;


    let user_id_vector = 
        vec![user_id_anonymous]
        .iter()
        .map(|u| u.to_string())
        .collect::<Vec<String>>();

    //let server_builder = 
    //    server_builder.create_sample_keypair(true);



    // previously we couldn't connect to this address, plus the port 
    // at this place was often used, i'll probably get a better endpoint

    let path = CUSTOM_ENDPOINT_PATH;

    // for learning purposes, i am only making ONE endpoint with
    // no security

    let my_endpoints = vec![
        ("custom_path", ServerEndpoint::new_none(path,&user_id_vector)),
    ];


    let server_builder = 
        server_builder.endpoints(my_endpoints);


    let is_server_valid = 
        server_builder.is_valid();

    println!("checking server validity..., found to be:");
    println!("{}",is_server_valid);




    // then we build the server

    let mut server = server_builder.server().unwrap();
    
    // step 2, u can add variables or nodes

    // i just copied and pasted these codes from the simple_server

    let ns = {
        let address_space = server.address_space();
        let mut address_space = address_space.write();
        address_space
            .register_namespace("urn:simple-server")
            .unwrap()
    };

    let v1_node = NodeId::new(ns, "v1");

    let address_space = server.address_space();

    // The address space is guarded so obtain a lock to change it
    {
        let mut address_space = address_space.write();

        // Create a sample folder under objects folder
        let sample_folder_id = address_space
            .add_folder("Sample", "Sample", &NodeId::objects_folder_id())
            .unwrap();

        // Add some variables to our sample folder. Values will be overwritten by the timer
        let _ = address_space.add_variables(
            vec![
                Variable::new(&v1_node, "v1", "v1", 0 as f64),
            ],
            &sample_folder_id,
        );
    }




    // step 3: when you finish configuring the server, tasks and etc
    // run the server
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

        println!("\n opc.tcp://{}:{}{} \n",ip_add,4840,CUSTOM_ENDPOINT_PATH);
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
