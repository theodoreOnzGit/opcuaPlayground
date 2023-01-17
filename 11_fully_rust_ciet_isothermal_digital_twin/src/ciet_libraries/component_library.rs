extern crate fluid_mechanics_rust;
use fluid_mechanics_rust::prelude::*;

use super::therminol_pipe::*;
use super::therminol_component::*;


/// Pipe6a in Compact Integral Effects Test (CIET)
/// CTAH branch 
///
/// It is a static mixer pipe
pub struct Pipe6a {

    // pipe 6a
    // otherwise known as the static mixer pipe 6a
    therminol_properties: TherminolVP1Properties,

}

impl<'pipe_lifetime> Pipe6a{

    pub fn new() -> Self {

        return Self { therminol_properties: TherminolVP1Properties::new() }

    }


    pub fn get(&self) -> TherminolPipe{


        let name = "pipe_6a";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.1526);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(51.526384);
        let form_loss_k = 5.05;


        let pipe_6a = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            &self.therminol_properties);

        return pipe_6a;
    }
}

/// static mixer 41
/// label component 6 
/// in Compact Integral Effects Test (CIET)
/// CTAH branch 
///
pub struct StaticMixer41 {
    // static mixer 41 (MX-41) on CIET diagram
    // in the pump and CTAH branch
    // just before CTAH (AKA IHX)
    // from top to bottom
    //
    // label 6 on diagram
    therminol_properties: TherminolVP1Properties,
}

impl StaticMixer41 {

    pub fn new() -> Self {

        return Self { therminol_properties: TherminolVP1Properties::new() }

    }

    /// custom darcy friction factor is 0
    /// MX-41 does not depend on L/D
    /// for friction factor
    pub fn custom_darcy(_reynolds_number: f64, _roughness_ratio: f64) -> f64 {
        return 0.0;
    }

    /// custom K value for static mixer 41
    pub fn custom_k(mut reynolds_number: f64) -> f64 {
        let mut reverse_flow = false;

        // the user account for reverse flow scenarios...
        if reynolds_number < 0.0 {
            reverse_flow = true;
            reynolds_number = reynolds_number * -1.0;
        }

        let custom_k_value =
            21.0 + 4000.0/reynolds_number;

        if reverse_flow {
            return -custom_k_value;
        }

        return custom_k_value;

    }

    /// returns an instance of MX-41
    /// or component no.6
    pub fn get(&self) -> TherminolCustomComponent {

        let name = "static_mixer_41_label_6";

        let therminol_properties_reference = &self.therminol_properties;
        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.1526);
        let cross_sectional_area = Area::new::<square_meter>(6.11e-4);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(51.526384);

        let static_mixer_41: TherminolCustomComponent
            = TherminolCustomComponent::new(
                name, 
                fluid_temp, 
                incline_angle, 
                component_length, 
                cross_sectional_area, 
                hydraulic_diameter, 
                absolute_roughness, 
                therminol_properties_reference, 
                &Self::custom_k, 
                &Self::custom_darcy);

        return static_mixer_41;
    }
}
