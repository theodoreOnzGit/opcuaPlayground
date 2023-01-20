extern crate approx;
#[test]
pub fn assert_ctah_behave_ok() {

    let mut mass_flowrate_kg_per_sec_vec: Vec<f64> = 
        vec![];

    mass_flowrate_kg_per_sec_vec.push(0.0);
    mass_flowrate_kg_per_sec_vec.push(0.2);
    mass_flowrate_kg_per_sec_vec.push(0.4);
    mass_flowrate_kg_per_sec_vec.push(0.7);
    mass_flowrate_kg_per_sec_vec.push(1.0);
    mass_flowrate_kg_per_sec_vec.push(-0.2);
    mass_flowrate_kg_per_sec_vec.push(-0.4);
    mass_flowrate_kg_per_sec_vec.push(-0.7);
    mass_flowrate_kg_per_sec_vec.push(-1.0);

    for mass_rate_kg_per_s in mass_flowrate_kg_per_sec_vec.iter() {

        use fluid_mechanics_rust::prelude::*;
        use crate::CTAHBranch;

        // get a version of ctah i know is working
        let temperature_degrees_c = 21.0;

        let pump_pressure_pascals = 0.0;

        let reference_ctah_pressure_change = 
            get_ctah_branch_isothermal_pressure_change_pascals(
                *mass_rate_kg_per_s,
                temperature_degrees_c,
                pump_pressure_pascals);

        // get a test version of ctah, the one based on traits

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
        let pipe_13 = ctah_branch_factory.get_pipe_13();
        let pipe_14 = ctah_branch_factory.get_pipe_14();
        let flowmeter_40_14a = ctah_branch_factory.get_flowmeter_40_14a();
        let pipe_15 = ctah_branch_factory.get_pipe_15();
        let pipe_16 = ctah_branch_factory.get_pipe_16();
        let branch_17 = ctah_branch_factory.get_branch_17();



        let user_specified_pump_pressure = 
            Pressure::new::<pascal>(pump_pressure_pascals);
        let mut mutable_ctah_pump = ctah_branch_factory.get_ctah_pump();
        mutable_ctah_pump.set_internal_pressure_source(user_specified_pump_pressure);


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
        ctah_branch_vector.push(&mutable_ctah_pump);
        // 11
        ctah_branch_vector.push(&pipe_13);
        // 12
        ctah_branch_vector.push(&pipe_14);
        //13
        ctah_branch_vector.push(&flowmeter_40_14a);
        //14
        ctah_branch_vector.push(&pipe_15);
        //15
        ctah_branch_vector.push(&pipe_16);
        //16
        ctah_branch_vector.push(&branch_17);


        let mut ctah_branch = CTAHBranch::new();
        ctah_branch.set_fluid_component_vector(ctah_branch_vector);

        let test_ctah_pressure_change = 
            ctah_branch.get_pressure_change(
                MassRate::new::<kilogram_per_second>(*mass_rate_kg_per_s));


        // assert

        approx::assert_relative_eq!(
            reference_ctah_pressure_change,
            test_ctah_pressure_change.value,
            max_relative = 0.01);
    }


    // now i want to test if the ctah branch 
    // can handle positive, negative and zero
    // pressure drops

    let mut pressure_vec_pa: Vec<f64> = vec![];

    pressure_vec_pa.push(0.0);
    pressure_vec_pa.push(0.2*1000.0);
    pressure_vec_pa.push(0.4*1000.0);
    pressure_vec_pa.push(0.7*1000.0);
    pressure_vec_pa.push(1.0*1000.0);
    pressure_vec_pa.push(-0.2*1000.0);
    pressure_vec_pa.push(-0.4*1000.0);
    pressure_vec_pa.push(-0.7*1000.0);
    pressure_vec_pa.push(-1.0*1000.0);

    for pressure_change_value in pressure_vec_pa.iter(){
        use fluid_mechanics_rust::prelude::*;
        use crate::CTAHBranch;

        // get a version of ctah i know is working
        let temperature_degrees_c = 21.0;

        let pump_pressure_pascals = 1000.0;


        // get a test version of ctah, the one based on traits

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
        let pipe_13 = ctah_branch_factory.get_pipe_13();
        let pipe_14 = ctah_branch_factory.get_pipe_14();
        let flowmeter_40_14a = ctah_branch_factory.get_flowmeter_40_14a();
        let pipe_15 = ctah_branch_factory.get_pipe_15();
        let pipe_16 = ctah_branch_factory.get_pipe_16();
        let branch_17 = ctah_branch_factory.get_branch_17();



        let user_specified_pump_pressure = 
            Pressure::new::<pascal>(pump_pressure_pascals);
        let mut mutable_ctah_pump = ctah_branch_factory.get_ctah_pump();
        mutable_ctah_pump.set_internal_pressure_source(user_specified_pump_pressure);


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
        ctah_branch_vector.push(&mutable_ctah_pump);
        // 11
        ctah_branch_vector.push(&pipe_13);
        // 12
        ctah_branch_vector.push(&pipe_14);
        //13
        ctah_branch_vector.push(&flowmeter_40_14a);
        //14
        ctah_branch_vector.push(&pipe_15);
        //15
        ctah_branch_vector.push(&pipe_16);
        //16
        ctah_branch_vector.push(&branch_17);


        let mut ctah_branch = CTAHBranch::new();
        ctah_branch.set_fluid_component_vector(ctah_branch_vector);

        let test_ctah_mass_flowrate = 
            ctah_branch.get_mass_flowrate_from_pressure_change(
                Pressure::new::<pascal>(*pressure_change_value));

        let reference_ctah_pressure_change: f64 = 
            get_ctah_branch_isothermal_pressure_change_pascals(
                test_ctah_mass_flowrate.value,
                temperature_degrees_c,
                pump_pressure_pascals);

        // assert
        //

        if *pressure_change_value == 0.0 {

            approx::assert_abs_diff_eq!(
                *pressure_change_value,
                reference_ctah_pressure_change,
                epsilon = 10.0);

            return;
        }


        approx::assert_relative_eq!(
            reference_ctah_pressure_change,
            pressure_change_value,
            max_relative = 0.01);

    }

    let mut pressure_vec_pa: Vec<f64> = vec![];

    pressure_vec_pa.push(0.0);
    pressure_vec_pa.push(0.2*1000.0);
    pressure_vec_pa.push(0.4*1000.0);
    pressure_vec_pa.push(0.7*1000.0);
    pressure_vec_pa.push(1.0*1000.0);
    pressure_vec_pa.push(-0.2*1000.0);
    pressure_vec_pa.push(-0.4*1000.0);
    pressure_vec_pa.push(-0.7*1000.0);
    pressure_vec_pa.push(-1.0*1000.0);
    pressure_vec_pa.push(-1.0*10000.0);
    pressure_vec_pa.push(1.0*10000.0);
    pressure_vec_pa.push(-1.0*1000000.0);
    pressure_vec_pa.push(1.0*1000000.0);

    for pressure_change_value in pressure_vec_pa.iter(){

        use fluid_mechanics_rust::prelude::*;
        use crate::CTAHBranch;

        // get a version of ctah i know is working
        let temperature_degrees_c = 21.0;

        let pump_pressure_pascals = *pressure_change_value;


        // get a test version of ctah, the one based on traits

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
        let pipe_13 = ctah_branch_factory.get_pipe_13();
        let pipe_14 = ctah_branch_factory.get_pipe_14();
        let flowmeter_40_14a = ctah_branch_factory.get_flowmeter_40_14a();
        let pipe_15 = ctah_branch_factory.get_pipe_15();
        let pipe_16 = ctah_branch_factory.get_pipe_16();
        let branch_17 = ctah_branch_factory.get_branch_17();



        let user_specified_pump_pressure = 
            Pressure::new::<pascal>(pump_pressure_pascals);
        let mut mutable_ctah_pump = ctah_branch_factory.get_ctah_pump();
        mutable_ctah_pump.set_internal_pressure_source(user_specified_pump_pressure);


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
        ctah_branch_vector.push(&mutable_ctah_pump);
        // 11
        ctah_branch_vector.push(&pipe_13);
        // 12
        ctah_branch_vector.push(&pipe_14);
        //13
        ctah_branch_vector.push(&flowmeter_40_14a);
        //14
        ctah_branch_vector.push(&pipe_15);
        //15
        ctah_branch_vector.push(&pipe_16);
        //16
        ctah_branch_vector.push(&branch_17);


        let mut ctah_branch = CTAHBranch::new();
        ctah_branch.set_fluid_component_vector(ctah_branch_vector);

        let test_ctah_mass_flowrate = 
            ctah_branch.get_mass_flowrate_from_pressure_change(
                Pressure::new::<pascal>(1000.0));

        let reference_ctah_pressure_change: f64 = 
            get_ctah_branch_isothermal_pressure_change_pascals(
                test_ctah_mass_flowrate.value,
                temperature_degrees_c,
                pump_pressure_pascals);

        // assert
        //

        if *pressure_change_value == 0.0 {

            approx::assert_abs_diff_eq!(
                *pressure_change_value,
                reference_ctah_pressure_change,
                epsilon = 10.0);

            return;
        }


        approx::assert_relative_eq!(
            reference_ctah_pressure_change,
            pressure_change_value,
            max_relative = 0.01);

    }

}

pub fn get_ctah_branch_isothermal_pressure_change_pascals(
    mass_rate_kg_per_s: f64,
    temperature_degrees_c: f64,
    pump_pressure_pascals: f64) -> f64 {

    extern crate fluid_mechanics_rust;
    use fluid_mechanics_rust::prelude::*;
    //import necessary things...
    use fluid_mechanics_rust::therminol_component::factory;
    use fluid_mechanics_rust::therminol_component::CalcPressureChange;

    // fluid temp and pressure
    let fluid_temp = ThermodynamicTemperature::new::<
        degree_celsius>(temperature_degrees_c);
    let mass_flowrate = MassRate::new::<
        kilogram_per_second>(mass_rate_kg_per_s);


    // let's get pipe 6a and static mixer 41
    // which is pipe 6 on diagram
    //
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

    // pipe 6a, static mixer 6,
    // ctah 7a, 7b and static mixer 8 and pipe 8a
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
