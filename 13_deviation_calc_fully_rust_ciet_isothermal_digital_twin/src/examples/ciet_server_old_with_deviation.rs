//use std::time;
use std::time::Instant;

use opcua::server::address_space;
#[warn(missing_docs)]
use opcua::server::prelude::*;
use local_ip_address::local_ip;
use opcua::server::{config};

use fluid_mechanics_rust::prelude::*;

use roots::find_root_brent;
use roots::SimpleConvergency;

/// In this example, we use the legacy ciet server codes used in maturin
/// to generate the results
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
    let ctah_pump_pressure_node = NodeId::new(ns, "ctah_pump_pressure");

    // I'll have another two here to close off the Heater and DHX branch respectively

    let heater_branch_valve_node = NodeId::new(ns, "heater_branch_valve_open");
    let dhx_branch_valve_node = NodeId::new(ns, "dhx_branch_valve_open");

    // Here are an additional 3 variables for calculation time
    let calculation_time_node = NodeId::new(ns, "calculation_time");
    let initiation_time_node = NodeId::new(ns, "ciet_obj_construction_time");
    let total_calc_time_node = NodeId::new(ns, "construction_time_plus_calc_time");

    // And then some more variables for 
    // (1) manometer reading error
    // (2) loop pressure drop error due to flowrate error of 2\%
    // (3) fldk error
    // (4) total error (sqrt sum of them)
    
    let manometer_reading_error_pascals_node 
        = NodeId::new(ns, "manometer_reading_error_pascals");
    let loop_pressure_drop_error_due_to_coriolis_flowmeter_pascals_node
        = NodeId::new(ns, "loop_pressure_drop_error_due_to_coriolis_flowmeter_pascals");
    let loop_pressure_drop_error_due_to_fldk_pascals_node
        = NodeId::new(ns, "loop_pressure_drop_error_due_to_fldk_pascals");
    let loop_pressure_drop_error_total_node
        = NodeId::new(ns, "loop_pressure_drop_error_total");



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
                              "calculation_time_ms", 
                              "calculation_time_ms", 0 as f64),
                Variable::new(&initiation_time_node, 
                              "ciet_obj_construction_time_ms", 
                              "ciet_obj_construction_time_ms", 0 as f64),
                Variable::new(&total_calc_time_node, 
                              "construction_time_plus_calc_time_ms", 
                              "construction_time_plus_calc_time_ms", 0 as f64),
            ],
            &sample_folder_id,
        );
    }

    // this part is responsible for errors of pressure drop
    {
        let mut address_space = address_space.write();

        // Create a sample folder under objects folder
        let sample_folder_id = address_space
            .add_folder("deviation and error", "deviation and error", &NodeId::objects_folder_id())
            .unwrap();

        // Add some variables to our sample folder. Values will be overwritten by the timer
        let _ = address_space.add_variables(
            vec![
                Variable::new(&manometer_reading_error_pascals_node, 
                              "manometer_reading_error_pascals", 
                              "manometer_reading_error_pascals", 0 as f64),
                Variable::new(&loop_pressure_drop_error_due_to_coriolis_flowmeter_pascals_node, 
                              "loop_pressure_drop_error_due_to_coriolis_flowmeter_pascals", 
                              "loop_pressure_drop_error_due_to_coriolis_flowmeter_pascals", 0 as f64),
                Variable::new(&loop_pressure_drop_error_due_to_fldk_pascals_node, 
                              "loop_pressure_drop_error_due_to_fldk_pascals", 
                              "loop_pressure_drop_error_due_to_fldk_pascals", 0 as f64),
                Variable::new(&loop_pressure_drop_error_total_node, 
                              "loop_pressure_drop_error_total", 
                              "loop_pressure_drop_error_total", 0 as f64),
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

        VariableBuilder::new(&heater_branch_valve_node,
                             "heater_branch_valve_open", "heater_branch_valve_open")
            .data_type(DataTypeId::Boolean)
            .value(true as bool)
            .writable()
            .organized_by(&folder_id)
            .insert(&mut address_space);

        VariableBuilder::new(&dhx_branch_valve_node,
                             "dhx_branch_valve_open", "dhx_branch_valve_open")
            .data_type(DataTypeId::Boolean)
            .value(true as bool)
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


    // we need to prepare transmitters and receivers for the
    // ciet isothermal facility

    //let (tx, rx) = mpsc::channel();


    // now this algorithm is REALLY inefficient, i am instantiating CIET at
    // EVERY timestep in addition to calculation
    //
    // but if it works, it works
    let calculate_flowrate_and_pressure_loss = move || {

        // construct CIET
        let start_of_object_init = Instant::now();
        let initiation_duration = start_of_object_init.elapsed();

        let start_of_calc_time = Instant::now();

        let mut address_space = address_space.write();
        
        // step 1, find the correct node object for 
        // pump pressure and the
        // boolean for valve control open or close
        let ctah_pump_node = ctah_pump_pressure_node.clone();
        let pump_pressure_value = address_space.
            get_variable_value(ctah_pump_node).unwrap();
        let pump_pressure_value: f64 = pump_pressure_value.
            value.unwrap().as_f64().unwrap();

        // now for heater valve and dhx valve
        let heater_valve_open = address_space.
            get_variable_value(heater_branch_valve_node.clone()).unwrap();
        //let heater_valve_open: bool = 
        //    heater_valve_open.value.unwrap().try_into::<bool>().unwrap();
        let heater_valve_open = true;

        let dhx_valve_open = address_space.
            get_variable_value(dhx_branch_valve_node.clone()).unwrap();
        //let dhx_valve_open: bool = 
        //    dhx_valve_open.value.unwrap().try_into::<bool>().unwrap();
        let dhx_valve_open = true;
        

        let ciet_temp_deg_c: f64 = 20.0;
        // step 2 calculate mass flowrate for ctah,
        // heater and dhx branch
        let (ctah_branch_flowrate,
             ctah_branch_pressure_change) = 
            get_ciet_isothermal_mass_flowrate(
                pump_pressure_value,
                ciet_temp_deg_c,
                dhx_valve_open,
                heater_valve_open
                );

        let heater_branch_flowrate = 
            get_heater_branch_mass_flowrate(
                ctah_branch_pressure_change.value,
                ciet_temp_deg_c,
                heater_valve_open);

        let dhx_branch_flowrate = 
            get_dhx_branch_mass_flowrate(
                ctah_branch_pressure_change.value,
                ciet_temp_deg_c,
                dhx_valve_open);

        // step 3, calc time
        let calc_time = start_of_calc_time.elapsed();


        let calc_time_taken_milleseconds: u16 = 
            calc_time.as_millis().try_into().unwrap();

        // step 4, update values into nodes
        let now = DateTime::now();
        let _ = address_space.set_variable_value(
            calculation_time_node.clone(), 
            calc_time_taken_milleseconds as f64,
            &now, 
            &now);

        let initiation_time_taken_millseconds: u16 =
            initiation_duration.as_millis().try_into().unwrap();

        let now = DateTime::now();
        let _ = address_space.set_variable_value(
            initiation_time_node.clone(), 
            initiation_time_taken_millseconds as f64,
            &now, 
            &now);
        let total_time_taken: u16 =
            calc_time_taken_milleseconds + initiation_time_taken_millseconds;

        let now = DateTime::now();
        let _ = address_space.set_variable_value(
            total_calc_time_node.clone(), 
            total_time_taken as f64,
            &now, 
            &now);

        
        let now = DateTime::now();
        let _ = address_space.set_variable_value(
            ctah_branch_mass_flowrate_node.clone(), 
            ctah_branch_flowrate as f64,
            &now, 
            &now);

        let now = DateTime::now();
        let _ = address_space.set_variable_value(
            heater_branch_mass_flowrate_node.clone(), 
            heater_branch_flowrate as f64,
            &now, 
            &now);

        let now = DateTime::now();
        let _ = address_space.set_variable_value(
            dhx_branch_mass_flowrate_node.clone(), 
            dhx_branch_flowrate as f64,
            &now, 
            &now);

        // step 5, calculate errors and print

        //(1) 2\% flowrate error
        let two_percent_flowrate_error_ctah_heater_only_flow = 
            get_loop_pressure_drop_error_due_to_flowmeter_ctah_heater(
                MassRate::new::<kilogram_per_second>(ctah_branch_flowrate),
                Pressure::new::<pascal>(pump_pressure_value),
                0.02);

        let now = DateTime::now();
        let _ = address_space.set_variable_value(
            loop_pressure_drop_error_due_to_coriolis_flowmeter_pascals_node.clone(), 
            two_percent_flowrate_error_ctah_heater_only_flow.value as f64,
            &now, 
            &now);

        //(2) 14.7 Pa manometer error
        let manometer_reading_error_pascals = 
            get_manometer_reading_error_pascals();

        let now = DateTime::now();
        let _ = address_space.set_variable_value(
            manometer_reading_error_pascals_node.clone(), 
            manometer_reading_error_pascals.value as f64,
            &now, 
            &now);

        //(3) 10\% fldk error

        let fldk_error_pascals_squared = 
            get_fldk_error_pascals_ctah_branch(
                MassRate::new::<kilogram_per_second>(ctah_branch_flowrate),
                0.10)
            * get_fldk_error_pascals_ctah_branch(
                MassRate::new::<kilogram_per_second>(ctah_branch_flowrate),
                0.10)
            + get_fldk_error_pascals_heater_branch(
                MassRate::new::<kilogram_per_second>(ctah_branch_flowrate),
                0.10)
            * get_fldk_error_pascals_heater_branch(
                MassRate::new::<kilogram_per_second>(ctah_branch_flowrate),
                0.10);

        let fldk_error_pascals = 
            fldk_error_pascals_squared.sqrt();

        let now = DateTime::now();
        let _ = address_space.set_variable_value(
            loop_pressure_drop_error_due_to_fldk_pascals_node.clone(), 
            fldk_error_pascals.value as f64,
            &now, 
            &now);

        //(4) total error

        let total_pressure_error_estimate_pascals_squared = 
            two_percent_flowrate_error_ctah_heater_only_flow * 
            two_percent_flowrate_error_ctah_heater_only_flow
            + manometer_reading_error_pascals *
            manometer_reading_error_pascals
            + fldk_error_pascals_squared;

        let total_pressure_error_estimate = 
            total_pressure_error_estimate_pascals_squared.sqrt();


        let now = DateTime::now();
        let _ = address_space.set_variable_value(
            loop_pressure_drop_error_total_node.clone(), 
            total_pressure_error_estimate.value as f64,
            &now, 
            &now);


        // i think we are done!




    };

    server.add_polling_action(500, calculate_flowrate_and_pressure_loss);

    // to check if polling server adds the polling time to
    // the execution time
    // i will get it to run every 2500 ms (2.5s)
    // and sleep will be 2.5s
    //
    // if it adds to the polling time, it will print as often as
    // the endpoint prints (every 5s)
    // otherwise it will print twice as often

    if run_server { server.run(); }

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

    let server = server_builder.server().unwrap();
    return server;

}

fn get_ip_as_str() -> String {

    let my_local_ip = local_ip().unwrap();

    // i can convert it to a string

    let ip_add_string : String = my_local_ip.to_string();

    return ip_add_string;

}

fn get_manometer_reading_error_pascals() -> Pressure {
    // this is the error of
    //
    // sqrt(1mm^2 + 1mm^2) of dowtherm A at 20C
    return Pressure::new::<pascal>(14.7);
}

fn get_fldk_error_pascals_ctah_branch(mass_flowrate: MassRate,
    error_fraction: f64) -> Pressure {

    //import necessary things...
    use fluid_mechanics_rust::therminol_component::factory;
    use fluid_mechanics_rust::prelude::*;
    use fluid_mechanics_rust::therminol_component::CalcPressureChange;

    let temperature_degrees_c: f64 = 20.0;
    // for this, we are taking a blanket 10\% error
    // but we can customise the error fraction
    // for all components along the CTAH Heater loop
    let fluid_temp = ThermodynamicTemperature::new::<
        degree_celsius>(temperature_degrees_c);

    let zero_mass_flow = 
        MassRate::new::<kilogram_per_second>(0.0);


    let mx10_2 = factory::StaticMixer10::get();

    let heater_version_1_1 =
        factory::CietHeaterVersion1::get();



    //MX41
    let static_mixer_41_6 = factory::StaticMixer41::get();

    // let's get the component for ctah
    // only the horizontal bit has experimentally determined
    // fldk values
    let ctah_horizontal_7b = factory::CTAHHorizontal::get();

    // MX40
    let static_mixer_40_8 = factory::StaticMixer40::get();

    // let's get flowmeter 14a
    let flowmeter_40_14a = factory::Flowmeter40::get();

    // now that we've gotten our items, we can
    // then sum up the pressure change contributions
    // given



    // static mixer 41 pressure

    let pressure_drop_mx41 =
        CalcPressureChange::from_mass_rate(
            &static_mixer_41_6,
            mass_flowrate,
            fluid_temp) - 
        CalcPressureChange::from_mass_rate(
            &static_mixer_41_6,
            zero_mass_flow,
            fluid_temp);


    // ctah pressure drop

    let pressure_drop_ctah = 
        CalcPressureChange::from_mass_rate(
            &ctah_horizontal_7b,
            mass_flowrate,
            fluid_temp) 
        - CalcPressureChange::from_mass_rate(
            &ctah_horizontal_7b,
            zero_mass_flow,
            fluid_temp);

    // static mixer 40 pressure
    let pressure_drop_mx40 =
        CalcPressureChange::from_mass_rate(
            &static_mixer_40_8,
            mass_flowrate,
            fluid_temp) - 
        CalcPressureChange::from_mass_rate(
            &static_mixer_40_8,
            zero_mass_flow,
            fluid_temp);

    // fm40 pressure drop
    let pressure_drop_fm40 =
        CalcPressureChange::from_mass_rate(
            &flowmeter_40_14a,
            mass_flowrate,
            fluid_temp) - 
        CalcPressureChange::from_mass_rate(
            &flowmeter_40_14a,
            zero_mass_flow,
            fluid_temp);


    // now we calculate the sum of squares
    let pressure_sq_deviation = 
        (error_fraction * pressure_drop_mx41) * (error_fraction * pressure_drop_mx41)
        + (error_fraction * pressure_drop_mx40) * (error_fraction * pressure_drop_mx40)
        + (error_fraction * pressure_drop_fm40) * (error_fraction * pressure_drop_fm40)
        + (error_fraction * pressure_drop_ctah) * (error_fraction * pressure_drop_ctah);

    let pressure_deviation_heater_branch = 
        pressure_sq_deviation.sqrt();

    // convert the object to f64 and return
    return pressure_deviation_heater_branch;
}

fn get_fldk_error_pascals_heater_branch(mass_flowrate: MassRate,
    error_fraction: f64) -> Pressure {

    //import necessary things...
    use fluid_mechanics_rust::therminol_component::factory;
    use fluid_mechanics_rust::prelude::*;
    use fluid_mechanics_rust::therminol_component::CalcPressureChange;

    let temperature_degrees_c: f64 = 20.0;
    // for this, we are taking a blanket 10\% error
    // but we can customise the error fraction
    // for all components along the CTAH Heater loop
    let fluid_temp = ThermodynamicTemperature::new::<
        degree_celsius>(temperature_degrees_c);

    let zero_mass_flow = 
        MassRate::new::<kilogram_per_second>(0.0);


    let mx10_2 = factory::StaticMixer10::get();

    let heater_version_1_1 =
        factory::CietHeaterVersion1::get();



    // now that we've gotten our items, we can
    // then sum up the pressure change contributions
    // given



    // static mixer 2 pressure

    let pressure_drop_mx10 =
        CalcPressureChange::from_mass_rate(
            &mx10_2,
            mass_flowrate,
            fluid_temp) - 
        CalcPressureChange::from_mass_rate(
            &mx10_2,
            zero_mass_flow,
            fluid_temp);


    // heater

    let pressure_drop_heater = 
        CalcPressureChange::from_mass_rate(
            &heater_version_1_1,
            mass_flowrate,
            fluid_temp) 
        - CalcPressureChange::from_mass_rate(
            &heater_version_1_1,
            zero_mass_flow,
            fluid_temp);

    // now we calculate the sum of squares
    let pressure_sq_deviation = 
        (error_fraction * pressure_drop_mx10) * (error_fraction * pressure_drop_mx10)
        + (error_fraction * pressure_drop_heater) * (error_fraction * pressure_drop_heater);

    let pressure_deviation_heater_branch = 
        pressure_sq_deviation.sqrt();

    // convert the object to f64 and return
    return pressure_deviation_heater_branch;
}

fn get_loop_pressure_drop_error_due_to_flowmeter_ctah_heater(
    mass_flowrate: MassRate,
    ctah_pump_pressure: Pressure,
    error_fraction: f64) -> Pressure {

    // convert to f64
    let x_value = mass_flowrate.value;
    let x_error = x_value*error_fraction;
    let temperature_degrees_c: f64 = 20.0;
    let pump_pressure_pascals = ctah_pump_pressure.value;


    // first, we compute a finite difference 
    // we use the ctah branch flowrate as the function with
    // which to perform sensitivity analysis
    //
    // on the loop pressure drop
    let x_upper = x_value + x_error;
    let x_lower = x_value - x_error;

    // loop pressure drop calcs
    //
    // we go from top to bottom of ctah and heater branch
    // when calculating pressure chg
    // flow goes top of branch to bottom
    //
    // to make it go in a loop,
    // we need to make the mass flowrate be positive
    // in one branch and negative in the other
    //
    // then to find the pressure drop after going through a loop
    // we need to trace from top to bottom of ctah branch
    // and bottom to top of heater branch
    let y_upper  = 
        get_ctah_branch_isothermal_pressure_change_pascals(
            x_upper,
            temperature_degrees_c,
            pump_pressure_pascals) -
        get_heater_branch_isothermal_pressure_change_pascals(
            -x_upper,
            temperature_degrees_c);



    let y_lower = 
        get_ctah_branch_isothermal_pressure_change_pascals(
            x_lower,
            temperature_degrees_c,
            pump_pressure_pascals) -
        get_heater_branch_isothermal_pressure_change_pascals(
            -x_lower,
            temperature_degrees_c);

    let gradient_estimate = 
        (y_upper - y_lower)/(x_upper - x_lower);

    let deviation_estimate_pascals = 
        x_error * gradient_estimate;

    // now lets return the pressure error
    return Pressure::new::<pascal>(deviation_estimate_pascals);

}


fn get_dhx_branch_isothermal_pressure_change_pascals(
    mass_rate_kg_per_s: f64,
    temperature_degrees_c: f64) -> f64 {
    use fluid_mechanics_rust::therminol_component::factory;
    use fluid_mechanics_rust::therminol_component::CalcPressureChange;
    use fluid_mechanics_rust::prelude::*;

    // fluid temp and pressure
    let fluid_temp = ThermodynamicTemperature::new::<
        degree_celsius>(temperature_degrees_c);
    let mass_flowrate = MassRate::new::<
        kilogram_per_second>(mass_rate_kg_per_s);

    // pipe 26, static mixer pipe 25a and static mixer 25
    let pipe_26 = factory::Pipe26::get();
    let mx21_25 = factory::StaticMixer21::get();
    let pipe_25a = factory::Pipe25a::get();


    // DHX heat exchanger
    let dhx_shell_side_24 = factory::DHXShellSideHeatExchanger::get();


    // static mixer pipe 23a, static mixer 23, and pipe 22
    let mx20_23 = factory::StaticMixer20::get();
    let pipe_23a = factory::Pipe23a::get();
    let pipe_22 = factory::Pipe22::get();

    // flowmeter 21a (FM-20)
    let flowmeter_20_21a = factory::Flowmeter20::get();

    // pipe 21 to 19
    let pipe_21 = factory::Pipe21::get();
    let pipe_20 = factory::Pipe20::get();
    let pipe_19 = factory::Pipe19::get();


    let mut pressure_change_total =
        Pressure::new::<pascal>(0.0);

    // pipe 26, static mixer pipe 25a and static mixer 25
    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_26,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &mx21_25,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_25a,
            mass_flowrate,
            fluid_temp);

    // DHX heat exchanger
    //
    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &dhx_shell_side_24,
            mass_flowrate,
            fluid_temp);

    // static mixer pipe 23a, static mixer 23, and pipe 22
    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &mx20_23,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_23a,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_22,
            mass_flowrate,
            fluid_temp);

    // flowmeter 21a (FM-20)

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &flowmeter_20_21a,
            mass_flowrate,
            fluid_temp);

    // pipe 21 to 19

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_21,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_20,
            mass_flowrate,
            fluid_temp);
    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_19,
            mass_flowrate,
            fluid_temp);


    // convert to f64 and return
    return pressure_change_total.get::<pascal>();
}

fn get_heater_branch_mass_flowrate(
        pressure_change_pascals: f64,
        temperature_degrees_c: f64,
        heater_branch_valve_open: bool) -> f64 {

    if heater_branch_valve_open == false {
        return 0.0;
    }

    // basically im solving for the mass rate which
    // returns the correct pressure change
    let upper_bound = MassRate::new::<kilogram_per_second>(1.0);
    let lower_bound = MassRate::new::<kilogram_per_second>(-1.0);

    let heater_pressure_chg_root = |mass_rate_kg_per_s: f64| -> f64 {

        return get_heater_branch_isothermal_pressure_change_pascals(
            mass_rate_kg_per_s,
            temperature_degrees_c) - pressure_change_pascals;
    };

    let mut convergency = SimpleConvergency { eps:1e-9_f64, max_iter:30 };

    let mass_flowrate_kg_per_s_solution = find_root_brent(
        upper_bound.value,
        lower_bound.value,
        &heater_pressure_chg_root,
        &mut convergency).unwrap();

    return mass_flowrate_kg_per_s_solution;
}


fn get_ctah_branch_mass_flowrate(
        pressure_change_pascals: f64,
        temperature_degrees_c: f64,
        pump_pressure_pascals: f64) -> f64 {

    let upper_bound = MassRate::new::<kilogram_per_second>(1.0);
    let lower_bound = MassRate::new::<kilogram_per_second>(-1.0);
    // basically im solving for the mass rate which
    // returns the correct pressure change
    let ctah_pressure_chg_root = |
            mass_rate_kg_per_s: f64| -> f64  {
        return get_ctah_branch_isothermal_pressure_change_pascals(
                mass_rate_kg_per_s,
                temperature_degrees_c,
                pump_pressure_pascals) - pressure_change_pascals;
            };

    let mut convergency = SimpleConvergency { eps:1e-9_f64, max_iter:30 };
    let mass_flowrate_kg_per_s_solution = find_root_brent(
        upper_bound.value,
        lower_bound.value,
        &ctah_pressure_chg_root,
        &mut convergency).unwrap();

    return mass_flowrate_kg_per_s_solution;

}


fn get_dhx_branch_mass_flowrate(
        pressure_change_pascals: f64,
        temperature_degrees_c: f64,
        dhx_branch_valve_open: bool) -> f64 {

    if dhx_branch_valve_open == false {
        return 0.0;
    }
    //# first let's check for reverse flow and return 0
    //# flow if reverse, it's computationally cheaper
    // here is where i implement the check valve behaviour
    let zero_mass_flow_value: f64 = 0.0;
    let hydrostatic_pressure = 
        get_dhx_branch_isothermal_pressure_change_pascals(
            zero_mass_flow_value,
            temperature_degrees_c);

    if pressure_change_pascals > hydrostatic_pressure {
        return 0.0;
    }

    let upper_bound = MassRate::new::<kilogram_per_second>(1.0);
    let lower_bound = MassRate::new::<kilogram_per_second>(-1.0);
    //# basically im solving for the mass rate which
    //# returns the correct pressure change
    let dhx_pressure_chg_root = | mass_rate_kg_per_s:f64 | -> f64 {
            return get_dhx_branch_isothermal_pressure_change_pascals(
                mass_rate_kg_per_s,
                temperature_degrees_c) - pressure_change_pascals;
        };

    let mut convergency = SimpleConvergency { eps:1e-9_f64, max_iter:30 };
    let mass_flowrate_kg_per_s_solution = find_root_brent(
        upper_bound.value,
        lower_bound.value,
        &dhx_pressure_chg_root,
        &mut convergency).unwrap();

    return mass_flowrate_kg_per_s_solution;
}

//# these methods solve for mass flowrate given a pump pressure
//# and system temperature


fn get_ciet_isothermal_mass_flowrate(
        pump_pressure_pascals: f64,
        temperature_degrees_c: f64,
        dhx_branch_valve_open: bool,
        heater_branch_valve_open: bool) -> (f64,Pressure) {
    //# the job of this function is to sum up the mass
    //# flowrate of the branches in ciet
    //# and solve for the value where the branch flowrates
    //# sum up to zero
    //# the convention is positive flowrate leaving the
    //# top and negative flowrate entering the top

    let pressure_change_root = |pressure_change_pascals: f64| -> f64 {
        //# both branches must be subject to the same
        //# pressure change since they are in parallel

        let heater_branch_mass_flowrate = get_heater_branch_mass_flowrate(
                        pressure_change_pascals,
                        temperature_degrees_c,
                        heater_branch_valve_open);

        let dhx_branch_mass_flowrate = get_dhx_branch_mass_flowrate(
                        pressure_change_pascals,
                        temperature_degrees_c,
                        dhx_branch_valve_open);

        let ctah_branch_mass_flowrate = get_ctah_branch_mass_flowrate(
                pressure_change_pascals,
                temperature_degrees_c,
                pump_pressure_pascals);

        let total_mass_flowrate = 
            heater_branch_mass_flowrate
            + dhx_branch_mass_flowrate
            + ctah_branch_mass_flowrate;

        return total_mass_flowrate;
    };

    //# we will solve for the correct pressure change
    //# given the mass flowrate
    //# the intervals will be 50000 pascals plus or minus
    //# the hydrostatic pressure change of the heater
    let zero_mass_flow_value = 0.0;
    let hydrostatic_pressure = 
        get_dhx_branch_isothermal_pressure_change_pascals(
            zero_mass_flow_value,
            temperature_degrees_c);

    let upper_bound = 
        hydrostatic_pressure +
        Pressure::new::<pascal>(50000_f64).value;

    let lower_bound = 
        hydrostatic_pressure +
        Pressure::new::<pascal>(-50000_f64).value;


    let mut convergency = SimpleConvergency { eps:1e-9_f64, max_iter:30 };


    let pressure_change_value 
        = find_root_brent(
            upper_bound,
            lower_bound,
            &pressure_change_root,
            &mut convergency).unwrap();

    //# once we get the pressure change root value,
    //# we can get mass flowrate

    let ctah_branch_mass_flowrate = get_ctah_branch_mass_flowrate(
            pressure_change_value,
            temperature_degrees_c,
            pump_pressure_pascals);


    return (ctah_branch_mass_flowrate,
            Pressure::new::<pascal>(pressure_change_value));
}

fn get_heater_branch_isothermal_pressure_change_pascals(
    mass_rate_kg_per_s: f64,
    temperature_degrees_c: f64) -> f64 {
    //import necessary things...
    use fluid_mechanics_rust::therminol_component::factory;
    use fluid_mechanics_rust::prelude::*;
    use fluid_mechanics_rust::therminol_component::CalcPressureChange;

    // fluid temp and pressure
    let fluid_temp = ThermodynamicTemperature::new::<
        degree_celsius>(temperature_degrees_c);
    let mass_flowrate = MassRate::new::<
        kilogram_per_second>(mass_rate_kg_per_s);

    // let's get and pipe 4
    //
    let pipe_4 = factory::Pipe4::get();

    // lets get pipe 3 and static mixer 2 and pipe 2a
    let pipe_3 = factory::Pipe3::get();
    let mx10_2 = factory::StaticMixer10::get();
    let pipe_2a = factory::Pipe2a::get();

    // let's get the heater components 1a, 1 and 1b
    let heater_top_head_1a =
        factory::HeaterTopHead1a::get();
    let heater_version_1_1 =
        factory::CietHeaterVersion1::get();
    let heater_bottom_head_label_1b =
        factory::HeaterBottomHead1b::get();

    // now we'll get pipe 18

    let pipe_18 = factory::Pipe18::get();

    // now that we've gotten our items, we can
    // then sum up the pressure change contributions
    // given

    let mut pressure_change_total =
        Pressure::new::<pascal>(0.0);

    // pipe4
    //

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_4,
            mass_flowrate,
            fluid_temp);

    // pipe 3 and static mixer 2 and pipe 2a
    //
    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_3,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &mx10_2,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_2a,
            mass_flowrate,
            fluid_temp);

    // heater
    //
    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &heater_top_head_1a,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &heater_version_1_1,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &heater_bottom_head_label_1b,
            mass_flowrate,
            fluid_temp);

    //pipe 18
    //
    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_18,
            mass_flowrate,
            fluid_temp);

    // convert the object to f64 and return
    return pressure_change_total.get::<pascal>();
}

// get hydrostatic pressure change
// for heater branch
//
//


fn get_heater_branch_isothermal_hydrostatic_pressure_pascals(
    temperature_degrees_c: f64) -> f64 {
    //import necessary things...
    use fluid_mechanics_rust::therminol_component::factory;
    use fluid_mechanics_rust::prelude::*;
    use fluid_mechanics_rust::therminol_component::
        StandardCustomComponentProperties;
    use fluid_mechanics_rust::therminol_component::
        StandardPipeProperties;

    // fluid temp and pressure
    let fluid_temp = ThermodynamicTemperature::new::<
        degree_celsius>(temperature_degrees_c);

    // let's get pipe 4
    //
    let pipe_4 = factory::Pipe4::get();

    // lets get pipe 3 and static mixer 2 and pipe 2a
    let pipe_3 = factory::Pipe3::get();
    let mx10_2 = factory::StaticMixer10::get();
    let pipe_2a = factory::Pipe2a::get();

    // let's get the heater components 1a, 1 and 1b
    let heater_top_head_1a =
        factory::HeaterTopHead1a::get();
    let heater_version_1_1 =
        factory::CietHeaterVersion1::get();
    let heater_bottom_head_label_1b =
        factory::HeaterBottomHead1b::get();

    // now we'll get pipe 18

    let pipe_18 = factory::Pipe18::get();

    // now that we've gotten our items, we can
    // then sum up the pressure change contributions
    // given

    let mut hydrostatic_pressure_change_total =
        Pressure::new::<pascal>(0.0);

    // branch5 and pipe4

    hydrostatic_pressure_change_total =
        hydrostatic_pressure_change_total +
        pipe_4.get_hydrostatic_pressure_change(
            fluid_temp);

    // pipe 3 and static mixer 2 and pipe 2a
    //
    hydrostatic_pressure_change_total =
        hydrostatic_pressure_change_total +
        pipe_3.get_hydrostatic_pressure_change(
            fluid_temp);

    hydrostatic_pressure_change_total =
        hydrostatic_pressure_change_total +
        mx10_2.get_hydrostatic_pressure_change(
            fluid_temp);

    hydrostatic_pressure_change_total =
        hydrostatic_pressure_change_total +
        pipe_2a.get_hydrostatic_pressure_change(
            fluid_temp);

    // heater
    //
    hydrostatic_pressure_change_total =
        hydrostatic_pressure_change_total +
        heater_top_head_1a.get_hydrostatic_pressure_change(
            fluid_temp);

    hydrostatic_pressure_change_total =
        hydrostatic_pressure_change_total +
        heater_version_1_1.get_hydrostatic_pressure_change(
            fluid_temp);

    hydrostatic_pressure_change_total =
        hydrostatic_pressure_change_total +
        heater_bottom_head_label_1b.get_hydrostatic_pressure_change(
            fluid_temp);

    //pipe 18
    //
    hydrostatic_pressure_change_total =
        hydrostatic_pressure_change_total +
        pipe_18.get_hydrostatic_pressure_change(
            fluid_temp);

    // convert the object to f64 and return
    return hydrostatic_pressure_change_total.get::<pascal>();
}

// get ctah branch pressure

fn get_ctah_branch_isothermal_pressure_change_pascals(
    mass_rate_kg_per_s: f64,
    temperature_degrees_c: f64,
    pump_pressure_pascals: f64) -> f64 {

    //import necessary things...
    use fluid_mechanics_rust::therminol_component::factory;
    use fluid_mechanics_rust::therminol_component::CalcPressureChange;

    use fluid_mechanics_rust::prelude::*;
    // fluid temp and pressure
    let fluid_temp = ThermodynamicTemperature::new::<
        degree_celsius>(temperature_degrees_c);
    let mass_flowrate = MassRate::new::<
        kilogram_per_second>(mass_rate_kg_per_s);


    // let's get branch 5 and pipe 6a and static mixer 41
    // which is pipe 6 on diagram
    //
    let branch_5 = factory::Branch5::get();
    let pipe_6a = factory::Pipe6a::get();
    let static_mixer_41_6 = factory::StaticMixer41::get();

    // let's get the component for ctah
    let ctah_vertical_7a = factory::CTAHVertical::get();
    let ctah_horizontal_7b = factory::CTAHHorizontal::get();

    // let's get the static mixer and pipe 8 and 8a

    let pipe_8a = factory::Pipe8a::get();
    let static_mixer_40_8 = factory::StaticMixer40::get();

    // now let's get pipe 9 to 12

    let pipe_9 = factory::Pipe9::get();
    let pipe_10 = factory::Pipe10::get();
    let pipe_11 = factory::Pipe11::get();
    let pipe_12 = factory::Pipe12::get();

    // and now for the pump
    let ctah_pump = factory::CTAHPump::get(
        pump_pressure_pascals);

    // let's now get pipe 13 and 14
    let pipe_13 = factory::Pipe13::get();
    let pipe_14 = factory::Pipe14::get();

    // let's get flowmeter 14a
    let flowmeter_40_14a = factory::Flowmeter40::get();

    // let's get pipe 15 and 16
    let pipe_15 = factory::Pipe15::get();
    let pipe_16 = factory::Pipe16::get();

    // let's now get branch 17
    let branch_17 = factory::Branch17::get();

    // now that we've gotten our items, we can
    // then sum up the pressure change contributions
    // given

    let mut pressure_change_total =
        Pressure::new::<pascal>(0.0);

    // branch 5 pipe 6a, static mixer 6,
    // ctah 7a, 7b and static mixer 8 and pipe 8a
    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &branch_5,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_6a,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &static_mixer_41_6,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &ctah_vertical_7a,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &ctah_horizontal_7b,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_8a,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &static_mixer_40_8,
            mass_flowrate,
            fluid_temp);

    // pipe 9 tp 12

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_9,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_10,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_11,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_12,
            mass_flowrate,
            fluid_temp);

    // ctah pump
    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &ctah_pump,
            mass_flowrate,
            fluid_temp);

    // pipe 13 to 17

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_13,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_14,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &flowmeter_40_14a,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_15,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_16,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &branch_17,
            mass_flowrate,
            fluid_temp);

    return pressure_change_total.get::<pascal>();
}
