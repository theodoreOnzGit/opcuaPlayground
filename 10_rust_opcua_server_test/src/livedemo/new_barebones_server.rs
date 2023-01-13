use opcua::server::prelude::*;


pub fn demo_1_barebones_server(run_server: bool) {


    let server_builder = ServerBuilder::new();

    let server_builder = 
        server_builder.application_name("my opcua demo server");

    // we will write it the truncated way from now on...

    // you can use a localhost ip address if you don't want to
    // conenct to another pc:
    // ipv4:
    // 127.0.0.1
    let server_builder = 
        server_builder.application_uri("urn:OPC UA Sample Server")
        .host_and_port("192.168.7.227",4840);


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

    let server = server_builder.server().unwrap();

    if run_server == true  {
        server.run();
    }
}
