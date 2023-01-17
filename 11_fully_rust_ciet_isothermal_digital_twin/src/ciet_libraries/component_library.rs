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

/// Static mixer pipe 8a
/// adjacent to MX-40 in the CTAH branch
pub struct Pipe8a {
    // pipe 8a
    // otherwise known as the static mixer pipe 8a
    therminol_properties: TherminolVP1Properties,
}

impl Pipe8a {

    /// returns and instance of pipe 8a
    pub fn get(&self) -> TherminolPipe{


        let name = "static_mixer_pipe_8a";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.22245);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(-90.0);
        let form_loss_k = 3.75;


        let pipe_8a = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            &self.therminol_properties);

        return pipe_8a;
    }

    pub fn new() -> Self {

        return Self { therminol_properties: TherminolVP1Properties::new() }

    }
}

/// static mixer 40 (MX-40) on CIET diagram
/// just after CTAH (AKA IHX)
/// from top to bottom
/// label 8 on diagram
///
/// forced convection flow direction is same as top to bottom
/// has a fldk of 21+4000/Re
pub struct StaticMixer40 {
    therminol_properties: TherminolVP1Properties,
}
impl StaticMixer40 {

    /// custom darcy is 0
    /// because fldk does not depend on L/D
    pub fn custom_darcy(_reynolds_number: f64, _roughness_ratio: f64) -> f64 {
        return 0.0;
    }

    
    /// has a fldk of 21+4000/Re
    /// it comes from the custom_k value
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

    /// returns an instance of MX-40
    pub fn get(&self) -> TherminolCustomComponent {

        let name = "static_mixer_40_label_8";

        let therminol_properties_reference = &self.therminol_properties;
        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);

        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.33);
        let cross_sectional_area = Area::new::<square_meter>(6.11e-4);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(-90.0);

        let static_mixer_40: TherminolCustomComponent
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

        return static_mixer_40;
    }

    pub fn new() -> Self {

        return Self { therminol_properties: TherminolVP1Properties::new() }

    }
}


/// pipe number 9 in CIET's CTAH branch
pub struct Pipe9 {
    // pipe 9
    therminol_properties: TherminolVP1Properties,
}

impl Pipe9 {

    /// returns instance of pipe 9
    /// returns and instance of pipe 8a
    pub fn get(&self) -> TherminolPipe{


        let name = "pipe_9";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.7112);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(-42.73211);
        let form_loss_k = 0.8;


        let pipe_9 = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            &self.therminol_properties);

        return pipe_9;
    }

    pub fn new() -> Self {

        return Self { therminol_properties: TherminolVP1Properties::new() }

    }

}

/// pipe number 10 in CIET's CTAH branch
pub struct Pipe10 {
    // pipe 10
    therminol_properties: TherminolVP1Properties,
}

impl Pipe10 {

    /// returns instance of pipe 10
    pub fn get(&self) -> TherminolPipe{


        let name = "pipe_10";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(2.4511);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(-90.0);
        let form_loss_k = 0.45;


        let pipe_10 = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            &self.therminol_properties);

        return pipe_10;
    }

    pub fn new() -> Self {

        return Self { therminol_properties: TherminolVP1Properties::new() }

    }

}


/// pipe number 11 in CIET's CTAH branch
pub struct Pipe11 {
    // pipe 11
    therminol_properties: TherminolVP1Properties,
}

impl Pipe11 {

    /// returns instance of pipe 11
    pub fn get(&self) -> TherminolPipe{


        let name = "pipe_11";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.4826);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(-63.47465);
        let form_loss_k = 2.4;


        let pipe_11 = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            &self.therminol_properties);

        return pipe_11;
    }

    pub fn new() -> Self {

        return Self { therminol_properties: TherminolVP1Properties::new() }

    }

}

/// pipe number 12 in CIET's CTAH branch
pub struct Pipe12 {
    // pipe 12
    therminol_properties: TherminolVP1Properties,
}

impl Pipe12 {

    /// returns instance of pipe 12
    pub fn get(&self) -> TherminolPipe{


        let name = "pipe_12";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.333375);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(0.0);
        let form_loss_k = 21.65;


        let pipe_12 = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            &self.therminol_properties);

        return pipe_12;
    }

    pub fn new() -> Self {

        return Self { therminol_properties: TherminolVP1Properties::new() }

    }

}

/// ctah pump is a custom therminol component with
/// ie no friction factor losses
/// but it provides a source pressure
///
/// it is located between pipe 12 and 13
pub struct CTAHPump {
    therminol_properties: TherminolVP1Properties,
}
impl CTAHPump {

    // let's import everything necessary:

    /// pump has no internal pressure loss
    /// so custom darcy friction factor is 0
    pub fn custom_darcy(_reynolds_number: f64, _roughness_ratio: f64) -> f64 {
        return 0.0;
    }

    /// pump has no internal pressure loss
    /// so custom k is 0
    pub fn custom_k(_reynolds_number: f64) -> f64 {
        return 0.0;
    }

    /// returns an instance of the pump with an internal
    /// pressure term set by the user in the get method
    pub fn get(&self) -> TherminolCustomComponent {

        let name = "ctah_pump";

        let therminol_properties_reference = &self.therminol_properties;
        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);

        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.36);
        let cross_sectional_area = Area::new::<square_meter>(6.11e-4);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(0.0);

        let ctah_pump: TherminolCustomComponent
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

        return ctah_pump;
    }

    pub fn new() -> Self {

        return Self { therminol_properties: TherminolVP1Properties::new() }

    }
}

/// pipe number 13 in CIET's CTAH branch
/// just after the pump
pub struct Pipe13 {
    // pipe 13 on the diagram in Nico Zweibaum nodalisation
    // probably some combination of V-42,
    // F-40 and F-41 on CIET diagram
    therminol_properties: TherminolVP1Properties,
}

impl Pipe13 {

    /// returns an instance of pipe13
    pub fn get(&self) -> TherminolPipe{


        let name = "pipe_13";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(1.273175);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(0.0);
        let form_loss_k = 12.95;


        let pipe_13 = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            &self.therminol_properties);

        return pipe_13;
    }

    pub fn new() -> Self {

        return Self { therminol_properties: TherminolVP1Properties::new() }

    }

}

/// pipe number 14 in CIET's CTAH branch
pub struct Pipe14 {
    // pipe 14 on the diagram in Nico Zweibaum nodalisation
    // probably some combination of V-42,
    // F-40 and F-41 on CIET diagram
    // it is inclined 90 degrees upwards in direction
    // of flow
    //
    // and from a top to bottom direction from pipe 5
    // to pipe 17, the incline angle is also 90 degrees
    therminol_properties: TherminolVP1Properties,
}

impl Pipe14 {

    /// returns an instance of pipe14
    pub fn get(&self) -> TherminolPipe{


        let name = "pipe_14";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.6687);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(90.0);
        let form_loss_k = 2.4;


        let pipe_14 = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            &self.therminol_properties);

        return pipe_14;
    }

    pub fn new() -> Self {

        return Self { therminol_properties: TherminolVP1Properties::new() }

    }
}

/// FM-40 Coriolis Flowmeter in CIET's CTAH branch
/// labelled 14a in simulation schmeatic
pub struct Flowmeter40 {
    // ctah line flowmeter 40
    // label 14a on simulation diagram
    // fldk = 18.0+93000/Re
    therminol_properties: TherminolVP1Properties,
}
impl Flowmeter40 {

    // let's import everything necessary:

    /// custom darcy is 0 because
    /// fldk does not depend on L/D
    pub fn custom_darcy(_reynolds_number: f64, _roughness_ratio: f64) -> f64 {
        return 0.0;
    }

    /// fldk = 18.0+93000/Re
    /// this is implemented by setting 
    /// K = = 18.0+93000/Re
    pub fn custom_k(mut reynolds_number: f64) -> f64 {
        let mut reverse_flow = false;

        // the user account for reverse flow scenarios...
        if reynolds_number < 0.0 {
            reverse_flow = true;
            reynolds_number = reynolds_number * -1.0;
        }

        let custom_k_value =
            18.0 + 93000.0/reynolds_number.powf(1.35);
        // coriolis flowmeter

        if reverse_flow {
            return -custom_k_value;
        }

        return custom_k_value;

    }

    /// returns an instance of FM-40 (14a)
    pub fn get(&self) -> TherminolCustomComponent {

        let name = "flowmeter_40_14a";

        let therminol_properties_reference = &self.therminol_properties;
        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);

        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.36);
        let cross_sectional_area = Area::new::<square_meter>(6.11e-4);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(90.0);

        let flowmeter_40_14a: TherminolCustomComponent
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

        return flowmeter_40_14a;
    }

    pub fn new() -> Self {

        return Self { therminol_properties: TherminolVP1Properties::new() }

    }
}


/// pipe number 15 in CIET's CTAH branch
pub struct Pipe15 {
    // pipe 15 on the diagram in Nico Zweibaum nodalisation
    // probably corresponds of F30 on CIET's P&ID
    //
    // and from a top to bottom direction from pipe 5
    // to pipe 17, the incline angle is also
    // -49.36983 degrees
    therminol_properties: TherminolVP1Properties,
}

impl Pipe15 {

    /// returns an instance of pipe 15
    pub fn get(&self) -> TherminolPipe{


        let name = "pipe_15";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.3556);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(-49.36983);
        let form_loss_k = 0.8;


        let pipe_15 = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            &self.therminol_properties);

        return pipe_15;
    }

    pub fn new() -> Self {

        return Self { therminol_properties: TherminolVP1Properties::new() }

    }
}

/// pipe number 16 in CIET's CTAH branch
pub struct Pipe16 {
    // pipe 16 on the diagram in Nico Zweibaum nodalisation
    // probably corresponds of F30 on CIET's P&ID
    //
    // and from a top to bottom direction from pipe 5
    // to pipe 17, the incline angle is also
    // -49.36983 degrees
    therminol_properties: TherminolVP1Properties,
}

impl Pipe16 {

    /// returns an instance of pipe 16
    pub fn get(&self) -> TherminolPipe{


        let name = "pipe_16";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.644525);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(-90.0);
        let form_loss_k = 1.9;


        let pipe_16 = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            &self.therminol_properties);

        return pipe_16;
    }

    pub fn new() -> Self {

        return Self { therminol_properties: TherminolVP1Properties::new() }

    }
}

/// Branch (or pipe 17) in CIET's CTAH branch
///
/// Approximations were made for this branch though,
/// technically branch 17a is part of CTAH branch
/// while 17b is part of the DHX branch,
/// I combined both for convenience
///
/// This is treated as a single pipe though
pub struct Branch17 {
    // pipe 17 on the diagram in Nico Zweibaum nodalisation
    // probably corresponds of F30 on CIET's P&ID
    //
    // and from a top to bottom direction from pipe 5
    // to pipe 17, the incline angle is 0 degrees
    //
    therminol_properties: TherminolVP1Properties,
}

impl Branch17 {

    /// returns an instance of Branch 17
    pub fn get(&self) -> TherminolPipe{


        let name = "branch_17";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.473075);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(0.0);
        let form_loss_k = 0.0;


        let branch_17 = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            &self.therminol_properties);

        return branch_17;
    }

    pub fn new() -> Self {

        return Self { therminol_properties: TherminolVP1Properties::new() }

    }
}
