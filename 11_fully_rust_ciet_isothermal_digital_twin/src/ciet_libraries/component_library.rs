extern crate fluid_mechanics_rust;
use fluid_mechanics_rust::prelude::*;

use super::therminol_pipe::*;
use super::therminol_component::*;


pub fn get_pip6a<'pipe_lifetime>(
    therminol_properties: &'pipe_lifetime TherminolVP1Properties) -> TherminolPipe<'pipe_lifetime> {

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
        therminol_properties);

    return pipe_6a;
}
