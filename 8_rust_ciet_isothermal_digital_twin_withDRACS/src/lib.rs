use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn add_four_rust(x: f64) -> PyResult<f64>{
    return Ok(x + 4.0);
}

#[pyfunction]
fn fldk_rust(reynolds_number: f64, roughness_ratio: f64,
             length_to_diameter: f64,
             k: f64) -> PyResult<f64> {
    return Ok(fLDK(reynolds_number, roughness_ratio, length_to_diameter, k));
}

#[pyfunction]
fn moody_rust(reynolds_number: f64,
         roughness_ratio: f64) -> PyResult<f64> {
    return Ok(moody(reynolds_number, roughness_ratio));
}

#[pyfunction]
fn get_ctah_branch_isothermal_pressure_change_pascals_rust(
    mass_rate_kg_per_s: f64,
    temperature_degrees_c: f64,
    pump_pressure_pascals: f64) -> PyResult<f64> {
    return Ok(
        get_ctah_branch_isothermal_pressure_change_pascals(
            mass_rate_kg_per_s,
            temperature_degrees_c,
            pump_pressure_pascals));
}

#[pyfunction]
fn get_heater_branch_isothermal_pressure_change_pascals_rust(
    mass_rate_kg_per_s: f64,
    temperature_degrees_c: f64) -> PyResult<f64> {
    return Ok(
        get_heater_branch_isothermal_pressure_change_pascals(
            mass_rate_kg_per_s,
            temperature_degrees_c
            ));
}

#[pyfunction]
fn get_dhx_branch_isothermal_pressure_change_pascals_rust(
    mass_rate_kg_per_s: f64,
    temperature_degrees_c: f64) -> PyResult<f64> {
    return Ok(
        get_dhx_branch_isothermal_pressure_change_pascals(
            mass_rate_kg_per_s,
            temperature_degrees_c
            ));
}

#[pyfunction]
fn get_heater_branch_isothermal_hydrostatic_pressure_pascals_rust(
    temperature_degrees_c: f64) -> PyResult<f64> {

    return Ok(
        get_heater_branch_isothermal_hydrostatic_pressure_pascals(
            temperature_degrees_c
            ));
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_functions_in_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(add_four_rust, m)?)?;
    m.add_function(wrap_pyfunction!(fldk_rust, m)?)?;
    m.add_function(wrap_pyfunction!(moody_rust, m)?)?;
    m.add_function(wrap_pyfunction!(
            get_ctah_branch_isothermal_pressure_change_pascals_rust,
            m)?)?;
    m.add_function(wrap_pyfunction!(
            get_heater_branch_isothermal_pressure_change_pascals_rust,
            m)?)?;
    m.add_function(wrap_pyfunction!(
            get_heater_branch_isothermal_hydrostatic_pressure_pascals_rust,
            m)?)?;
    m.add_function(wrap_pyfunction!(
            get_dhx_branch_isothermal_pressure_change_pascals_rust,
            m)?)?;
    Ok(())
}


// here are the functions used for friction factor, rather messy but
// for fast prototyping and sandboxing don't really care too much
//
//

/// code for ciet isothermal digital twin
///

fn get_dhx_branch_isothermal_pressure_change_pascals(
    mass_rate_kg_per_s: f64,
    temperature_degrees_c: f64) -> f64 {
    use fluid_mechanics_rust::therminol_component::factory;
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use uom::si::pressure::pascal;
    use fluid_mechanics_rust::therminol_component::CalcPressureChange;

    use uom::si::f64::*;
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

fn get_heater_branch_isothermal_pressure_change_pascals(
    mass_rate_kg_per_s: f64,
    temperature_degrees_c: f64) -> f64 {
    //import necessary things...
    use fluid_mechanics_rust::therminol_component::factory;
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use uom::si::pressure::pascal;
    use fluid_mechanics_rust::therminol_component::CalcPressureChange;

    use uom::si::f64::*;
    // fluid temp and pressure
    let fluid_temp = ThermodynamicTemperature::new::<
        degree_celsius>(temperature_degrees_c);
    let mass_flowrate = MassRate::new::<
        kilogram_per_second>(mass_rate_kg_per_s);

    // let's get branch 5 and pipe 4
    //
    let branch_5 = factory::Branch5::get();
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

    // branch5 and pipe4
    //
    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &branch_5,
            mass_flowrate,
            fluid_temp);

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
    use uom::si::thermodynamic_temperature::degree_celsius;
    use uom::si::pressure::pascal;
    use fluid_mechanics_rust::therminol_component::
        StandardCustomComponentProperties;
    use fluid_mechanics_rust::therminol_component::
        StandardPipeProperties;

    use uom::si::f64::*;
    // fluid temp and pressure
    let fluid_temp = ThermodynamicTemperature::new::<
        degree_celsius>(temperature_degrees_c);

    // let's get branch 5 and pipe 4
    //
    let branch_5 = factory::Branch5::get();
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
    //
    hydrostatic_pressure_change_total =
        hydrostatic_pressure_change_total +
        branch_5.get_hydrostatic_pressure_change(
            fluid_temp);

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
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use uom::si::pressure::pascal;
    use fluid_mechanics_rust::therminol_component::CalcPressureChange;

    use uom::si::f64::*;
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

/// code for friction factor

// first, to allow non snake case names..
#[allow(non_snake_case)]
fn B(Re: f64) -> f64 {
    let numerator = 37530.0_f64.powf(16.0);
    let denominator = Re.powf(16.0);
    return numerator/denominator;
}

#[allow(non_snake_case)]
fn A(Re: f64, roughnessRatio: f64) -> f64 {
    let seven_over_Re = 7.0/Re;
    let reynolds_term = seven_over_Re.powf(0.9);

    let roughness_term = 0.27 * roughnessRatio;

    let log_fraction = 1.0/(reynolds_term + roughness_term);
    // we will need natural logarithms:
    let inner_bracket_term = 2.457*log_fraction.ln();

    let A = inner_bracket_term.powf(16.0);

    return A;


}

#[allow(non_snake_case)]
fn churchillInnerTerm(Re: f64, roughnessRatio: f64) -> f64 {

    let eight_over_Re = 8.0/Re;
    let laminarTerm = eight_over_Re.powf(12.0);

    let Aterm = A(Re,roughnessRatio);
    let Bterm = B(Re);

    let APlusBInverse = 1.0/(Aterm+Bterm);
    let turbulentTerm = APlusBInverse.powf(3.0/2.0);

    return laminarTerm + turbulentTerm;
}

// this particular implementation uses the churchill correlation
#[allow(non_snake_case)]
fn fanning(ReynoldsNumber: f64, roughnessRatio: f64) -> f64{

    if ReynoldsNumber == 0.0 {
        panic!("Re = 0.0");
    }

    if ReynoldsNumber < 0.0 {
        panic!("Re<0.0");
    }

    if roughnessRatio < 0.0 {
        panic!("roughnessRatio<0.0");
    }

    let innerTerm = churchillInnerTerm(ReynoldsNumber, roughnessRatio);
    let powerTerm = innerTerm.powf(1.0/12.0);
    let fanningFrictionFactor = 2.0 * powerTerm;
    return fanningFrictionFactor;
}

#[allow(non_snake_case)]
fn darcy(ReynoldsNumber: f64, roughnessRatio: f64) -> f64 {
    return 4.0*fanning(ReynoldsNumber, roughnessRatio);
}

#[allow(non_snake_case)]
fn moody(ReynoldsNumber: f64, roughnessRatio: f64) -> f64 {
    return 4.0*fanning(ReynoldsNumber, roughnessRatio);
}


#[allow(non_snake_case)]
fn fLDK(ReynoldsNumber: f64,
                   roughnessRatio: f64,
                   lengthToDiameterRatio: f64,
                   K: f64) -> f64{
    if ReynoldsNumber == 0.0 {
        panic!("Re = 0");
    }

    if ReynoldsNumber < 0.0 {
        panic!("Re < 0");
    }

    if roughnessRatio < 0.0 {
        panic!("roughnessRatio<0.0");
    }

    if lengthToDiameterRatio <= 0.0 {
        panic!("lengthToDiameterRatio<=0.0");
    }

    if K < 0.0 {
        panic!("For m loss coefficient K < 0.0");
    }

    let f = darcy(ReynoldsNumber, roughnessRatio);
    let fLDK = f*lengthToDiameterRatio + K;

    return fLDK;
}
