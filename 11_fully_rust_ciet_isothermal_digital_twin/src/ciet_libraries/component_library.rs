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

/// Vertical part of Coiled Tube Air Heater (CTAH)
/// label component 7a
/// in Compact Integral Effects Test (CIET)
/// CTAH branch 
///
pub struct CTAHVertical {

    // coiled tube air heater,
    // uses pipe friction factors but has a constant K value
    // also pipe isn't circular
    // so we'll have to use custom fldk to help
    // label 7a
    therminol_properties: TherminolVP1Properties,
}

/// CTAH vertical is actually an fldk type pipe
///
/// but because I was quickly copying templates from
/// other fldk components, it became easy just
/// to force the vertical CTAH to be a custom fldk component
///
impl CTAHVertical {


    /// CTAH has a darcy friction factor from churchill
    /// correlation
    pub fn custom_darcy(mut reynolds_number: f64, roughness_ratio: f64) -> f64 {

        if roughness_ratio < 0.0 {
            panic!("roughness_ratio < 0.0");
        }

        use fluid_mechanics_rust::churchill_friction_factor;
        let mut reverse_flow = false;

        // the user account for reverse flow scenarios...
        if reynolds_number < 0.0 {
            reverse_flow = true;
            reynolds_number = reynolds_number * -1.0;
        }

        let darcy = churchill_friction_factor::darcy(reynolds_number,
                                                     roughness_ratio);

        if reverse_flow {
            return -darcy;
        }
        return darcy;
    }

    /// CTAH has a fixed K value of 3.9 
    pub fn custom_k(reynolds_number: f64) -> f64 {

        let custom_k_value = 3.9;

        if reynolds_number < 0.0 {
            return -custom_k_value
        }

        return custom_k_value;

    }

    pub fn get(&self) -> TherminolCustomComponent {

        let name = "ctah_vertical_label_7a";

        let therminol_properties_reference = &self.therminol_properties;
        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);

        let hydraulic_diameter = Length::new::<meter>(1.19e-2);
        let component_length = Length::new::<meter>(0.3302);
        let cross_sectional_area = Area::new::<square_meter>(1.33e-3);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(-90.0);

        let ctah_vertical: TherminolCustomComponent
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

        return ctah_vertical;
    }
    pub fn new() -> Self {

        return Self { therminol_properties: TherminolVP1Properties::new() }

    }
}

/// Horizontal part of Coiled Tube Air Heater (CTAH)
/// label component 7b
/// in Compact Integral Effects Test (CIET)
/// CTAH branch 
pub struct CTAHHorizontal {

    // coiled tube air heater
    // has fldk = 400 + 52,000/Re
    //
    // label is 7b
    // empirical data in page 48 on pdf viewer in Dr
    // Zweibaum thesis shows reverse flow has same
    // pressure drop characteristics as forward flow
    therminol_properties: TherminolVP1Properties,
}

impl CTAHHorizontal {


    /// custom darcy friction factor is 0
    /// the horizontal CTAH correlation does not depend on L/D
    /// for friction factor
    pub fn custom_darcy(_reynolds_number: f64, _roughness_ratio: f64) -> f64 {
        return 0.0;
    }


    /// coiled tube air heater (CTAH) horizontal component
    /// has fldk = 400 + 52,000/Re
    pub fn custom_k(mut reynolds_number: f64) -> f64 {

        let mut reverse_flow = false;

        // the user account for reverse flow scenarios...
        if reynolds_number < 0.0 {
            reverse_flow = true;
            reynolds_number = reynolds_number * -1.0;
        }

        let custom_k_value =
            400.0 + 52000.0/reynolds_number;

        if reverse_flow {
            return -custom_k_value;
        }

        return custom_k_value;

    }

    /// returns an instance of the
    /// horizontal portion of CTAH

    pub fn get(&self) -> TherminolCustomComponent {

        let name = "ctah_horizontal_label_7b";

        let therminol_properties_reference = &self.therminol_properties;
        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);

        let hydraulic_diameter = Length::new::<meter>(1.19e-2);
        let component_length = Length::new::<meter>(1.2342);
        let cross_sectional_area = Area::new::<square_meter>(1.33e-3);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(0.0);

        let ctah_horizontal: TherminolCustomComponent
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

        return ctah_horizontal;
    }
    pub fn new() -> Self {

        return Self { therminol_properties: TherminolVP1Properties::new() }

    }
}

