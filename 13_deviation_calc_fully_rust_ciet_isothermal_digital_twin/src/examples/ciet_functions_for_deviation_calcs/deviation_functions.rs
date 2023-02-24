use fluid_mechanics_rust::prelude::*;
//use roots::find_root_brent;
//use roots::SimpleConvergency;
use super::*;

/// Loop pressure drop error due to 
/// ctah and heater branch in one loop
///
/// this only works if DHX loop is closed
pub fn get_loop_pressure_drop_error_due_to_flowmeter_ctah_heater_specific(
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
    ///// in one branch and negative in the other
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

/// Loop pressure drop error due only to CTAH loop and Heater Loop
///
/// This should work regardless of whether the DHX and Heater
/// or CTAH is valved on or off
pub fn get_loop_pressure_drop_error_due_to_flowmeter_ctah_heater(
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
    ///// in one branch and negative in the other
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

/// The algorithm here does a few things
/// to help ctah obtain the estimated loop pressure drop
/// regardless of parallel connections
///
/// Firstly, mass flowrate through CTAH is supplied,
/// That will give a pressure change value across ctah branch
///
/// using this pressure change value, calculate mass flowrates
/// across the DHX and Heater branch flow paths
///
/// after that, i can calculate the mass flowrate in the ctah branch
/// simple addition.
///
/// Thereafter, i can calculate the pressure change across the ctah
/// branch assuming ctah pump pressure or loop pressure drop equals zero
/// 
/// This will yield some pressure change
/// The difference between this pressure change and the actual pressure
/// change used to obtain the flowrates across the other two branches
/// will become the ctah pump pressure at operating point
///
/// This will yield a mass flowrate through the ctah (m1) and a ctah
/// pump pressure (p1). This p1 should equal the ctah pump pressure
/// that was supplied
///
/// repeat this for another two data points sets by increasing and decreasing
/// the loop pressure drop by some small value. eg 10 Pa or 50 Pa.
///
/// of course, if mass flowrate is zero or close to zero, then we 
/// just return zero for the error. How close? Depends how far we deviate 
/// pressure.
///
/// Otherwise, we will be having check valve behaviour issues
/// The ideal amount for 10 Pa seems to be 0.0004 kg/s
/// 
/// I manually tested if it works compared to the code that
/// works for single branch.
///
/// It works. (2:35pm 23 feb 2023);
///
pub fn parameterically_estimate_ctah_loop_pressure_drop_error_due_to_flowrate(
    ctah_branch_mass_flowrate: MassRate,
    ctah_pump_pressure: Pressure,
    heater_branch_valve_open: bool,
    dhx_branch_valve_open: bool,
    ctah_branch_valve_open: bool,
    temperature_degrees_c: f64,
    error_fraction: f64) -> Pressure {

    // check if mass flowrate value is less than 0.0004 kg/s 
    // if so, return 0 Pa (negligible pressure error)
    // this is the amount of deviation expected for 10 Pa
    // of pressure 
    //
    // 10 Pa is the branch pressure change I will use 
    // for finite differencing.
    if ctah_branch_mass_flowrate.value.abs() <= 0.0004_f64 {
        return Pressure::new::<pascal>(0.0);
    }

    // if ctah branch is closed, or if dhx and heater branch are
    // both closed, return zero, prevents crashes
    // or panics due to nonconvergence
    if ctah_branch_valve_open == false {
        return Pressure::new::<pascal>(0.0);
    }

    if !dhx_branch_valve_open && !heater_branch_valve_open == true {
        return Pressure::new::<pascal>(0.0);
    }
    // now let me obtain the branch pressure change for the
    // fixed ctah_branch_mass_flowrate

    // now we get the standard ctah branch pressure change
    let ctah_branch_pressure_change_pascals = 
        get_ctah_branch_isothermal_pressure_change_pascals(
            ctah_branch_mass_flowrate.value,
            temperature_degrees_c,
            ctah_pump_pressure.value);

    // we can then apply this pressure change to other branches

    let heater_branch_mass_flowrate: f64 =
        get_heater_branch_mass_flowrate(
            ctah_branch_pressure_change_pascals, 
            temperature_degrees_c, 
            heater_branch_valve_open);

    let dhx_branch_mass_flowrate: f64 = 
        get_dhx_branch_mass_flowrate(
            ctah_branch_pressure_change_pascals, 
            temperature_degrees_c, 
            dhx_branch_valve_open);

    let mass_conservation_error: f64 = 
        ctah_branch_mass_flowrate.value
        + heater_branch_mass_flowrate
        + dhx_branch_mass_flowrate;

    if mass_conservation_error >= 0.0004_f64 {
        panic!("mass conservation not fulfilled for error measurement");
    }
    // the first part was merely validation, just wanted to check
    // if the errors were within tolerance
    // now we can vary the branch pressure parametrically

    let branch_pressure_increased = 
        ctah_branch_pressure_change_pascals + 10.0;

    let branch_pressure_decreased = 
        ctah_branch_pressure_change_pascals - 10.0;

    // from the increased branch pressure, i want
    // to obtain a set of data: mass flowrate through ctah
    // and then the corresponding ctah_pump_pressure 
    // i will just nest a function here

    fn obtain_mass_flowrate_and_ctah_loop_pressure_drop_pair(
        branch_pressure_change_pascals: f64,
        temperature_degrees_c: f64,
        heater_branch_valve_open: bool,
        dhx_branch_valve_open: bool,
        ctah_branch_valve_open: bool) -> (MassRate, Pressure) {
        // step 1, obtain mass flowrate for dhx and heater branch

        let heater_branch_flowrate_kg_per_s: f64
            = get_heater_branch_mass_flowrate(
                branch_pressure_change_pascals, 
                temperature_degrees_c, 
                heater_branch_valve_open);

        let dhx_branch_mass_flowrate_kg_per_s: f64 
            = get_dhx_branch_mass_flowrate(
                branch_pressure_change_pascals, 
                temperature_degrees_c, 
                dhx_branch_valve_open);

        // now obtain mass flowrate for ctah_branch
        // normally 
        // mass flowrate formula is like this
        //
        // ctah_mass_flow + dhx_mass_flow + heater_mass_flow = 0
        // 
        // so ctah_mass_flow = -(dhx_mass_flow + heater_mass_flow)

        let ctah_branch_mass_flowrate_kg_per_s: f64
            = -(heater_branch_flowrate_kg_per_s
                + dhx_branch_mass_flowrate_kg_per_s);

        // we can feed this into the ctah branch pressure change

        // if ctah branch valve is closed, we should not be calling this function

        if ctah_branch_valve_open == false {
            panic!("ctah valve should be open for mass flow calcs");
        }


        let ctah_branch_pressure_change_without_pump_pascals: f64 
            = get_ctah_branch_isothermal_pressure_change_pascals(
                ctah_branch_mass_flowrate_kg_per_s, 
                temperature_degrees_c, 
                0.0);

        // now we can estimate the loop pressure drop change using
        // the actual ctah branch pressure change
        // minus the pressure change without pump
        //
        // This is because
        // pressure change = - pressure drop + hydrostatic pressure chg + pump pressure
        //
        // in the branch pressure case, the pump pressure term is included
        // in the branch pressure change without pump, the pump pressure is not included
        // or zero
        //

        let ctah_pump_pressure_pascals: f64 = 
            branch_pressure_change_pascals 
            - ctah_branch_pressure_change_without_pump_pascals;

        let ctah_mass_flow = 
            MassRate::new::<kilogram_per_second>(ctah_branch_mass_flowrate_kg_per_s);
        let ctah_pump_pressure = 
            Pressure::new::<pascal>(ctah_pump_pressure_pascals);

        return (ctah_mass_flow,ctah_pump_pressure);

    }

    let (ctah_mass_flow_1, ctah_pump_pressure_1) = 
        obtain_mass_flowrate_and_ctah_loop_pressure_drop_pair(
            branch_pressure_increased, 
            temperature_degrees_c, 
            heater_branch_valve_open, 
            dhx_branch_valve_open, 
            ctah_branch_valve_open);

    let (ctah_mass_flow_2, ctah_pump_pressure_2) = 
        obtain_mass_flowrate_and_ctah_loop_pressure_drop_pair(
            branch_pressure_decreased, 
            temperature_degrees_c, 
            heater_branch_valve_open, 
            dhx_branch_valve_open, 
            ctah_branch_valve_open);

    let pressure_error_due_to_flowrate_error: Pressure = 
        ctah_branch_mass_flowrate * 
        error_fraction *
        (ctah_pump_pressure_2 - ctah_pump_pressure_1)/
        (ctah_mass_flow_2 - ctah_mass_flow_1);

    return pressure_error_due_to_flowrate_error;
}





/// returns manometer reading error obtained
/// from subtracting one manometer reading from the other
/// 1mm of error is assumed for random reading errors
pub fn get_manometer_reading_error_pascals() -> Pressure {
    // this is the error of
    //
    // sqrt(1mm^2 + 1mm^2) of dowtherm A at 20C
    return Pressure::new::<pascal>(14.7);
}

/// obtains the pressure loss coefficient errors due
/// to deviation of correlation from experimental data for
/// ctah branch
pub fn get_fldk_error_pascals_ctah_branch(mass_flowrate: MassRate,
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

/// obtains the pressure loss coefficient errors due
/// to deviation of correlation from experimental data for
/// heater branch
pub fn get_fldk_error_pascals_heater_branch(mass_flowrate: MassRate,
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

/// obtains the pressure loss coefficient errors due
/// to deviation of correlation from experimental data for
/// dhx branch
pub fn get_fldk_error_pascals_dhx_branch(mass_flowrate: MassRate,
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



    // static mixer 20 and 21
    let mx20_23 = factory::StaticMixer20::get();
    let mx21_25 = factory::StaticMixer21::get();

    // flowmeter 21a (FM-20)
    let flowmeter_20_21a = factory::Flowmeter20::get();

    // now that we've gotten our items, we can
    // then sum up the pressure change contributions
    // given



    // static mixer 20 and 21 pressure

    let pressure_drop_mx20 = 
        CalcPressureChange::from_mass_rate(
            &mx20_23,
            mass_flowrate,
            fluid_temp) - 
        CalcPressureChange::from_mass_rate(
            &mx20_23,
            zero_mass_flow,
            fluid_temp);


    let pressure_drop_mx21 = 
        CalcPressureChange::from_mass_rate(
            &mx21_25,
            mass_flowrate,
            fluid_temp) - 
        CalcPressureChange::from_mass_rate(
            &mx21_25,
            zero_mass_flow,
            fluid_temp);

    // FM20


    let pressure_drop_fm20 = 
        CalcPressureChange::from_mass_rate(
            &flowmeter_20_21a,
            mass_flowrate,
            fluid_temp) 
        - CalcPressureChange::from_mass_rate(
            &flowmeter_20_21a,
            zero_mass_flow,
            fluid_temp);


    // now we calculate the sum of squares
    let pressure_sq_deviation = 
        (error_fraction * pressure_drop_fm20) * (error_fraction * pressure_drop_fm20)
        + (error_fraction * pressure_drop_mx21) * (error_fraction * pressure_drop_mx21)
        + (error_fraction * pressure_drop_mx20) * (error_fraction * pressure_drop_mx20);

    let pressure_deviation_dhx_branch = 
        pressure_sq_deviation.sqrt();

    // convert the object to f64 and return
    return pressure_deviation_dhx_branch;
}
