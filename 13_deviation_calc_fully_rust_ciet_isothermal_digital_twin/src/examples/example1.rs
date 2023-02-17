#[warn(missing_docs)]
use opcua::server::prelude::*;


/// example 1, read source code for more info
pub fn example_1_timer_server_no_connection(run_server: bool){

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


    // next, discovery urls, here is the default value used
    // you will need to state the opc address like so:
    // "opc.tcp://ip-address:port-number/"
    //
    // so a localhost endpoint at port 4840 using IPv4 is as follows:
    const EXAMPLE_ENDPOINT_PATH: &str = "opc.tcp://127.0.0.1:4840/";

    // the second way to create a discovery url is just to leave it as a default
    // value, which is a forward slash
    // and the server builder will configure it for you
    const DEFAULT_ENDPOINT_PATH: &str = "/";

    // the only things you need here are host and port address

    let server_builder = 
        server_builder.host_and_port("192.168.10.177", 4840);


    let server_builder =
        server_builder.discovery_urls(
            vec![
            EXAMPLE_ENDPOINT_PATH.into(),
            DEFAULT_ENDPOINT_PATH.into()]);

    // some other config stuff for security i don't want to dive into
    // it's a boolean here
    //
    // you don't really need these for a valid server builder config
    // https://github.com/locka99/opcua/blob/master/lib/src/server/config.rs
    //

    //let server_builder = 
    //    server_builder.create_sample_keypair(true);

    //let server_builder = 
    //    server_builder.pki_dir("./pki-server");

    //let server_builder = 
    //    server_builder.discovery_server_url(None);

    // lastly (optional) we can check if the server config is valid

    // we can configure username and password
    // the first way is kinda complex

    let user_id_1 = "my_user";

    let user_token = ServerUserToken { 
        user: "my_user".to_string(),
        pass: Some("my_password".to_string()), 
        x509: None, 
        thumbprint: None 
    };

    let server_builder = 
        server_builder.user_token(user_id_1, user_token);

    // of course, there is an easier way

    let user_id_2 = "hello_user";
    let user_token = ServerUserToken::user_pass("hello", "hellow2");

    let server_builder = 
        server_builder.user_token(user_id_2, user_token);

    // now i have two users added
    // hello with password hellow2 and
    // my_user with password my_password
    // 
    // the last step is to make server endpoints
    //
    // for now i ignore security
    // the endpoint needs a list of all users
    //
    // which basically, we convert all the string slices
    // into String types

    let user_id_vector = 
        vec![user_id_1, user_id_1]
        .iter()
        .map(|u| u.to_string())
        .collect::<Vec<String>>();

    // we'll use the user_id_vector and endpoint path
    // to add it to our list of endpoints

    let path = DEFAULT_ENDPOINT_PATH;

    // for learning purposes, i am only making ONE endpoint with
    // no security

    let my_endpoints = vec![
        ("none", ServerEndpoint::new_none(path,&user_id_vector))
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

    // step 3: when you finish configuring the server, tasks and etc
    // run the server
    //
    // the server should run, but it does not really allow connections
    //

    // runs server if the user wants to
    if run_server == true {
        server.run();
    }
    

}

