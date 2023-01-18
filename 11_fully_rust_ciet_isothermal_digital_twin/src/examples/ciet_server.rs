use std::thread;
use std::time;
use std::time::Instant;

#[warn(missing_docs)]
use opcua::server::prelude::*;
use local_ip_address::local_ip;
use opcua::server::{config};

use fluid_mechanics_rust::prelude::*;

use crate::CIETIsothermalFacility;
use crate::CTAHBranch;
use crate::DHXBranch;
use crate::HeaterBranch;

/// in example 8,     
/// we want to check if the server runs polling actions synchronously 
/// or asynchronously
pub fn construct_and_run_ciet_server(run_server: bool){

    let mut server = build_standard_server();

    let ns = {
        let address_space = server.address_space();
        let mut address_space = address_space.write();
        address_space
            .register_namespace("urn:simple-server")
            .unwrap()
    };

    // I'll have 4 variables here
    // note that each variable needs a separate node ID
    // this is how the user will interact with ciet: through these
    // flowrates and the pump pressure

    let ctah_branch_mass_flowrate_node = NodeId::new(ns, "ctah_branch_mass_flowrate");
    let heater_branch_mass_flowrate_node = NodeId::new(ns, "heater_branch_flowrate");
    let dhx_branch_mass_flowrate_node = NodeId::new(ns, "dhx_branch_flowrate");
    let calculation_time_node = NodeId::new(ns, "calculation_time");
    let ctah_pump_pressure_node = NodeId::new(ns, "ctah_pump_pressure");

    let address_space = server.address_space();

    // this part is responsible for sensor data
    {
        let mut address_space = address_space.write();

        // Create a sample folder under objects folder
        let sample_folder_id = address_space
            .add_folder("sensor data", "sensor data", &NodeId::objects_folder_id())
            .unwrap();

        // Add some variables to our sample folder. Values will be overwritten by the timer
        let _ = address_space.add_variables(
            vec![
                Variable::new(&ctah_branch_mass_flowrate_node, 
                              "ctah_branch_mass_flowrate_kg_per_s", 
                              "ctah_branch_mass_flowrate_kg_per_s", 0 as f64),
                Variable::new(&heater_branch_mass_flowrate_node, 
                              "heater_branch_mass_flowrate_kg_per_s", 
                              "heater_branch_mass_flowrate_kg_per_s", 0 as f64),
                Variable::new(&dhx_branch_mass_flowrate_node, 
                              "dhx_branch_mass_flowrate_kg_per_s", 
                              "dhx_branch_mass_flowrate_kg_per_s", 0 as f64),
                Variable::new(&calculation_time_node, 
                              "calculation_time_s", 
                              "calculation_time_s", 0 as u16),
            ],
            &sample_folder_id,
        );
    }

    // this is the piece of code for the writeonly variable
    // we can use booleans or floats
    {
        let mut address_space = address_space.write();
        let folder_id = address_space
            .add_folder("Controller", "Controller", &NodeId::objects_folder_id())
            .unwrap();


        VariableBuilder::new(&ctah_pump_pressure_node, 
                             "ctah_branch_pressure_pa", "ctah_branch_pressure_pa")
            .data_type(DataTypeId::Float)
            .value(0 as f64)
            .writable()
            .organized_by(&folder_id)
            .insert(&mut address_space);
    }




    // adding functions to ciet's server now...
    //
    // this one prints the endpoint every 5s so the user knows
    // how to connect to ciet

    let print_endpoint_simple = || {
        let ip_add = get_ip_as_str();

        println!("\n opc.tcp://{}:{}{} \n",ip_add,4840,CUSTOM_ENDPOINT_PATH);
    };

    //server.add_polling_action(5000, print_endpoint);
    server.add_polling_action(5000, print_endpoint_simple);

    // here i will start storing all of ciet's component objects
    // and constructing them into vectors
    //
    // firstly, the ctah branch

    let ctah_branch_factory = CTAHBranch::new();
    let pipe6a = ctah_branch_factory.get_pipe6a();
    let static_mixer_41 = ctah_branch_factory.get_static_mixer_41();
    let ctah_vertical = ctah_branch_factory.get_ctah_vertical();
    let ctah_horizontal = ctah_branch_factory.get_ctah_horizontal();
    let pipe_8a = ctah_branch_factory.get_pipe_8a();
    let static_mixer_40 = ctah_branch_factory.get_static_mixer_40();
    let pipe_9 = ctah_branch_factory.get_pipe_9();
    let pipe_10 = ctah_branch_factory.get_pipe_10();
    let pipe_11 = ctah_branch_factory.get_pipe_11();
    let pipe_12 = ctah_branch_factory.get_pipe_12();
    let ctah_pump = ctah_branch_factory.get_ctah_pump();
    let pipe_13 = ctah_branch_factory.get_pipe_13();
    let pipe_14 = ctah_branch_factory.get_pipe_14();

    // now push them into a vector

    let mut ctah_branch_vector :Vec<&dyn FluidComponent> = vec![];

    // element number: 0 
    ctah_branch_vector.push(&pipe6a); 
    // 1
    ctah_branch_vector.push(&static_mixer_41);
    // 2
    ctah_branch_vector.push(&ctah_vertical);
    // 3
    ctah_branch_vector.push(&ctah_horizontal);
    // 4
    ctah_branch_vector.push(&pipe_8a);
    // 5
    ctah_branch_vector.push(&static_mixer_40);
    // 6
    ctah_branch_vector.push(&pipe_9);
    // 7
    ctah_branch_vector.push(&pipe_10);
    // 8
    ctah_branch_vector.push(&pipe_11);
    // 9
    ctah_branch_vector.push(&pipe_12);
    // 10
    ctah_branch_vector.push(&ctah_pump);
    // 11
    ctah_branch_vector.push(&pipe_13);
    // 12
    ctah_branch_vector.push(&pipe_14);

    // now set the vector in the CTAHBranch Object

    let mut ctah_branch = CTAHBranch::new();
    ctah_branch.set_fluid_component_vector(ctah_branch_vector);

    // let's try setting ctah branch pressure to about 500 Pa
    //
    // This is basically how you set ctah pump pressure... kind of cumbersome
    // but oh well

    let mut mutable_ctah_pump = ctah_branch_factory.get_ctah_pump();
    
    ctah_branch.set_ctah_pump_pressure(
        Pressure::new::<pascal>(500_f64), 
        &mut mutable_ctah_pump);

    // now for the heater branch

    let heater_branch_factory = HeaterBranch::new();

    let branch5 = heater_branch_factory.get_branch5();
    let pipe4 = heater_branch_factory.get_pipe4();
    let pipe3 = heater_branch_factory.get_pipe3();
    let mixer10 = heater_branch_factory.get_mixer10();
    let pipe2a = heater_branch_factory.get_pipe2a();
    let heater_top_head_1a = heater_branch_factory.get_heater_top_head_1a();
    let ciet_heater = heater_branch_factory.get_ciet_heater();
    let heater_bottom_head_1b = heater_branch_factory.get_heater_bottom_head_1b();
    let pipe18 = heater_branch_factory.get_pipe18();

    let mut heater_branch_vector :Vec<&dyn FluidComponent> = vec![];

    heater_branch_vector.push(&branch5);
    heater_branch_vector.push(&pipe4);
    heater_branch_vector.push(&pipe3);
    heater_branch_vector.push(&mixer10);
    heater_branch_vector.push(&pipe2a);
    heater_branch_vector.push(&heater_top_head_1a);
    heater_branch_vector.push(&ciet_heater);
    heater_branch_vector.push(&heater_bottom_head_1b);
    heater_branch_vector.push(&pipe18);

    let mut heater_branch = HeaterBranch::new();
    heater_branch.set_fluid_component_vector(heater_branch_vector);

    // last but not least the dhx branch
    let dhx_branch_factory = DHXBranch::new();

    let pipe26 = dhx_branch_factory.get_pipe26();
    // item 25
    let static_mixer_21 = dhx_branch_factory.get_static_mixer_21();
    let pipe25a = dhx_branch_factory.get_pipe25a();
    // item 24
    let dhx_shell_side_heat_exchanger = dhx_branch_factory.get_dhx_shell_side_heat_exchanger();
    // item 23
    let static_mixer_20 = dhx_branch_factory.get_static_mixer_20();
    let pipe23a = dhx_branch_factory.get_pipe23a();
    let pipe22 = dhx_branch_factory.get_pipe22();
    // item 21a
    let flowmeter20 = dhx_branch_factory.get_flowmeter20();
    let pipe21 = dhx_branch_factory.get_pipe21();
    let pipe20 = dhx_branch_factory.get_pipe20();
    let pipe19 = dhx_branch_factory.get_pipe19();

    let mut dhx_branch_vector :Vec<&dyn FluidComponent> = vec![];

    dhx_branch_vector.push(&pipe26);
    dhx_branch_vector.push(&static_mixer_21);
    dhx_branch_vector.push(&pipe25a);
    dhx_branch_vector.push(&dhx_shell_side_heat_exchanger);
    dhx_branch_vector.push(&static_mixer_20);
    dhx_branch_vector.push(&pipe23a);
    dhx_branch_vector.push(&pipe22);
    dhx_branch_vector.push(&flowmeter20);
    dhx_branch_vector.push(&pipe21);
    dhx_branch_vector.push(&pipe20);
    dhx_branch_vector.push(&pipe19);

    let mut dhx_branch = DHXBranch::new();
    dhx_branch.set_fluid_component_vector(dhx_branch_vector);

    // now we are ready for ciet

    let mut ciet_isothermal_facility = 
        CIETIsothermalFacility::new(ctah_branch, heater_branch, dhx_branch);

    let calculate_flowrate_and_pressure_loss = move || {
        // first let's get the address space
        // i want to first set my ciet ctah branch pressure to the user specified
        // value
        let mut address_space = address_space.write();

        // we first find a variable by nodeID
        // i'm trying to find the lbm node and return its value
        //
        
        // step 1, find the correct node object
        let ctah_pump_node = ctah_pump_pressure_node.clone();

        // step 2, find the variable using this node object
        let pump_pressure_value = address_space.
            get_variable_value(ctah_pump_node).unwrap();

        // step 3, convert variable value into f64
        let pump_pressure_value: f64 = pump_pressure_value.
            value.unwrap().as_f64().unwrap();

        // step 4 convert f64 to Pressure
        let user_specified_pump_pressure = 
            Pressure::new::<pascal>(pump_pressure_value);

        // step 5, set the pump pressure to the correct value
        // and calculate everything

        // this mutable ctah pump cannot be used safely
        // as it cannot be shared between threads
        //let mutable_ctah_pump = ctah_branch_factory.get_ctah_pump();

        // this mutable ciet_isothermal_facility 
        // also cannot be shared safely between threads
        //let time_taken = 
        //    ciet_isothermal_facility.calculate(
        //    user_specified_pump_pressure, &mut mutable_ctah_pump);

        let start = Instant::now();

        let time_taken = start.elapsed();

        // step 6 set the time variable

        let time_taken_milleseconds: u16 = 
            time_taken.as_millis().try_into().unwrap();

        let now = DateTime::now();
        let _ = address_space.set_variable_value(
            calculation_time_node.clone(), 
            time_taken_milleseconds as u16,
            &now, 
            &now);

        // i think we are done!




    };

    server.add_polling_action(100, calculate_flowrate_and_pressure_loss);

    // to check if polling server adds the polling time to
    // the execution time
    // i will get it to run every 2500 ms (2.5s)
    // and sleep will be 2.5s
    //
    // if it adds to the polling time, it will print as often as
    // the endpoint prints (every 5s)
    // otherwise it will print twice as often




}

const CUSTOM_ENDPOINT_PATH: &str = "/rust_ciet_opcua_server";
fn build_standard_server() -> Server {

    let server_builder = ServerBuilder::new();

    let server_builder = 
        server_builder.application_name("test server_builder");

    let server_builder =
        server_builder.application_uri("urn:OPC UA Sample Server");




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
    return server;

}

fn get_ip_as_str() -> String {

    let my_local_ip = local_ip().unwrap();

    // i can convert it to a string

    let ip_add_string : String = my_local_ip.to_string();

    return ip_add_string;

}
