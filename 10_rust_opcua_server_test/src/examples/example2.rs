#[warn(missing_docs)]
use opcua::server::prelude::*;
use local_ip_address::local_ip;
use opcua::server::{state::ServerState, config};

pub fn example_2_timer_server_auto_ip_addr_no_connection(run_server: bool){

    // now we build off example 1
    // in example 1, we needed to manually specify our ip address, 
    // This is not quite desirable,
    // hence, we are going to use a new function for our ip address
    // 
    // i'm going to repeat the same steps as before

    let server_builder = ServerBuilder::new();

    let server_builder = 
        server_builder.application_name("test server_builder");

    let server_builder =
        server_builder.application_uri("urn:OPC UA Sample Server");

    // previously we couldn't connect to this address, plus the port 
    // at this place was often used, i'll probably get a better endpoint
    // like /my_rust_opcua_server


    const EXAMPLE_ENDPOINT_PATH: &str = "opc.tcp://127.0.0.1:4840/";
    const DEFAULT_ENDPOINT_PATH: &str = "/";
    const CUSTOM_ENDPOINT_PATH: &str = "/my_rust_opcua_server";

    // and i'll use a function to automatically get my local ip address
    // (ipv4)
    let ip_address = get_ip_as_str();

    let server_builder = 
        server_builder.host_and_port(&ip_address, 4840);


    let server_builder =
        server_builder.discovery_urls(
            vec![
            EXAMPLE_ENDPOINT_PATH.into(),
            DEFAULT_ENDPOINT_PATH.into(),
            CUSTOM_ENDPOINT_PATH.into(),
            ]);


    // username and password

    let user_id_1 = "my_user";

    let user_token = ServerUserToken { 
        user: "my_user".to_string(),
        pass: Some("my_password".to_string()), 
        x509: None, 
        thumbprint: None 
    };

    let server_builder = 
        server_builder.user_token(user_id_1, user_token);


    let user_id_2 = "hello_user";
    let user_token = ServerUserToken::user_pass("hello", "hellow2");

    let server_builder = 
        server_builder.user_token(user_id_2, user_token);

    let user_id_anonymous = config::ANONYMOUS_USER_TOKEN_ID;


    let user_id_vector = 
        vec![user_id_1, user_id_1, user_id_anonymous]
        .iter()
        .map(|u| u.to_string())
        .collect::<Vec<String>>();

    //let server_builder = 
    //    server_builder.create_sample_keypair(true);



    // previously we couldn't connect to this address, plus the port 
    // at this place was often used, i'll probably get a better endpoint

    let path = DEFAULT_ENDPOINT_PATH;
    let custom_path = CUSTOM_ENDPOINT_PATH;

    // for learning purposes, i am only making ONE endpoint with
    // no security

    let my_endpoints = vec![
        ("custom_path", ServerEndpoint::new_none(custom_path,&user_id_vector)),
        ("custom_path_security", ServerEndpoint::new_basic256_sign(custom_path,&user_id_vector))
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


    let print_endpoint = || {

        let server_state = 
            server.server_state();

        let server_state_read = server_state.try_read().unwrap();

        let endpoint: &str = &server_state_read.base_endpoint;

        println!("\n connect at this endpoint \n");
        println!("{}",endpoint);


    };

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
