#[warn(missing_docs)]
use opcua::server::prelude::*;
use local_ip_address::local_ip;
use opcua::server::{state::ServerState, config};

pub fn example_5_read_and_write_variables(run_server: bool){

    // in example 5, we add two variables to the server, 
    //
    // one is read only,
    // one is write only

    let server_builder = ServerBuilder::new();

    let server_builder = 
        server_builder.application_name("test server_builder");

    let server_builder =
        server_builder.application_uri("urn:OPC UA Sample Server");



    const CUSTOM_ENDPOINT_PATH: &str = "/my_rust_opcua_server";

    let ip_address = get_ip_as_str();

    let server_builder = 
        server_builder.host_and_port(&ip_address, 4840);


    let server_builder =
        server_builder.discovery_urls(
            vec![
            CUSTOM_ENDPOINT_PATH.into(),
            ]);


    // username and password is just anonymous

    let user_id_anonymous = config::ANONYMOUS_USER_TOKEN_ID;


    let user_id_vector = 
        vec![user_id_anonymous]
        .iter()
        .map(|u| u.to_string())
        .collect::<Vec<String>>();




    let path = CUSTOM_ENDPOINT_PATH;


    let my_endpoints = vec![
        ("custom_path", ServerEndpoint::new_none(path,&user_id_vector)),
    ];


    let server_builder = 
        server_builder.endpoints(my_endpoints);


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

    // i'll then make a read only and write only node

    let readonly_variable_node = NodeId::new(ns, "readonly_variable");
    let writeable_variable_node = NodeId::new(ns, "writeable_variable");

    let address_space = server.address_space();

    // this is a piece of code to write the readonly variable
    {
        let mut address_space = address_space.write();

        // Create a sample folder under objects folder
        let sample_folder_id = address_space
            .add_folder("readonly", "readonly", &NodeId::objects_folder_id())
            .unwrap();

        // Add some variables to our sample folder. Values will be overwritten by the timer
        let _ = address_space.add_variables(
            vec![
                Variable::new(&readonly_variable_node, 
                              "readonly_variable", "readonly_variable", 0 as f64),
            ],
            &sample_folder_id,
        );
    }

    // this is the piece of code for the writeonly variable
    // we can use booleans or floats
    {
        let mut address_space = address_space.write();
        let folder_id = address_space
            .add_folder("Writeable", "Writeable", &NodeId::objects_folder_id())
            .unwrap();

        VariableBuilder::new(&writeable_variable_node, 
                             "writeable_variable", "writeable_variable")
            .data_type(DataTypeId::Float)
            .value(0 as f64)
            .writable()
            .organized_by(&folder_id)
            .insert(&mut address_space);
    }

    // this is the writeonly variable
    //


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

    // i can convert it to a string

    let ip_add_string : String = my_local_ip.to_string();

    return ip_add_string;

}
