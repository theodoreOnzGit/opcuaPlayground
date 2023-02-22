use fluid_mechanics_rust::prelude::*;
//use roots::find_root_brent;
//use roots::SimpleConvergency;
use super::*;

/// Loop pressure drop error due to 
/// ctah and heater branch in one loop
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

