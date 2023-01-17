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


    pub fn get(&'pipe_lifetime mut self) -> TherminolPipe<'pipe_lifetime> {


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
