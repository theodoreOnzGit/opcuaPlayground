use opcua::server::{prelude::*, address_space};
use local_ip_address::local_ip;


pub fn demo_4_read_and_write_variables_with_closures(run_server: bool) {


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

    // for adding variables, we need a namespace


    let ns = {
        let address_space = server.address_space();
        let mut address_space = address_space.write();
        address_space
            .register_namespace("urn:simple-server")
            .unwrap()
    };


    let celsius_node = NodeId::new(ns, "celsius_node");

    // let's make a mutable server reference

    let server_reference = &mut server;

    let address_space = server.address_space();

    // add the node to the address space
    {
        // obtain a lock to make this writeable (sort of)
        let mut address_space = address_space.write();

        // Create a sample folder under objects folder
        let sample_folder_id = address_space
            .add_folder("readonly", "readonly", &NodeId::objects_folder_id())
            .unwrap();

        // Add some variables to our sample folder. Values will be overwritten by the timer
        let _ = address_space.add_variables(
            vec![
            Variable::new(&celsius_node, "celsius_variable", "celsius_variable", 
                          3.0 as f64),
            ],
            &sample_folder_id,
            );
    }

    let fahrenheit_node = NodeId::new(ns, "fahrenheit_node");
    // let's add a writeable variable
    {
        // obtain a lock to write to the address space
        let mut address_space = address_space.write();

        // 
        let folder_id = address_space
            .add_folder("writable_variable_folder", "writable_variable_folder", 
                        &NodeId::objects_folder_id())
            .unwrap();

        VariableBuilder::new(&fahrenheit_node, "fahrenheit_variable", "fahrenheit_variable")
            .data_type(DataTypeId::Double)
            .value(0.0_f64)
            .writable() // Important: this is what makes the variable writeable
            .organized_by(&folder_id)
            .insert(&mut address_space);
    }

    // let's add a closure to print ip address
    let print_endpoint = move || {

        println!("opc.tcp://{}:{:?}{} \n",
                 &ip_address,
                 port_number,
                 EXAMPLE_ENDPOINT_PATH);
    };
    
    server.add_polling_action(3000, print_endpoint);

    // add a closure that converts fahrenheit to celsius

    let convert_fahrenheit_to_celsius = move || {


        let mut address_space_local = address_space.write();

        // Step 1: read the fahrenheit node

        let fahrenheit_node = fahrenheit_node.clone();

        // Step 2: get fahrenheit variable
        let fahrenheit_variable = 
            address_space_local.get_variable_value(fahrenheit_node);

        // Step 3: convert to f64

        let fahrenheit_value: f64 = 
            fahrenheit_variable.unwrap()
            .value
            .unwrap()
            .as_f64()
            .unwrap();

        // Step 4: convert to celsius

        let celsius_value: f64 =
            (fahrenheit_value - 32.0)*5.0/9.0;

        // Step 5: set the celsius variable

        let now = DateTime::now();
        let _ = address_space_local.
            set_variable_value(celsius_node.clone(), 
                               celsius_value as f64,
                               &now, 
                               &now);

        // i think we are done!

    };

    server.add_polling_action(300, convert_fahrenheit_to_celsius);


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

