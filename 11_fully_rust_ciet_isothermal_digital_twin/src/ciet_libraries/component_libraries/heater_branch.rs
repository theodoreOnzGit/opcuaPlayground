extern crate fluid_mechanics_rust;
use fluid_mechanics_rust::prelude::*;

use crate::{Branch5, 
    therminol_pipe::TherminolPipe, therminol_component::TherminolCustomComponent, 
    Pipe4, Pipe3, StaticMixer10, Pipe2a, HeaterTopHead1a, CietHeaterVersion1, HeaterBottomHead1b, Pipe18};

pub struct HeaterBranch<'heater_branch_lifetime> {

    branch5: Branch5,
    pipe4: Pipe4,
    pipe3: Pipe3,
    mixer10: StaticMixer10,
    pipe2a: Pipe2a,
    heater_top_head_1a: HeaterTopHead1a,
    ciet_heater: CietHeaterVersion1,
    heater_bottom_head_1b: HeaterBottomHead1b,
    pipe18: Pipe18,


    fluid_component_vector_immutable: 
        Vec<&'heater_branch_lifetime dyn FluidComponent>
}

impl<'heater_branch_lifetime> HeaterBranch<'heater_branch_lifetime> {

    /// constructor, returns an instance of the ctah branch
    pub fn new() -> Self {

        
        unimplemented!();
        // constructor will return the CTAH branch with all its items
        // but the vector will be empty
    }

    pub fn get_branch5(&self) -> TherminolPipe {
        return self.branch5.get();
    }

    pub fn get_pipe4(&self) -> TherminolPipe {
        return self.pipe4.get();
    }
    pub fn get_pipe3(&self) -> TherminolPipe {
        return self.pipe3.get();
    }
    pub fn get_mixer10(&self) -> TherminolCustomComponent {
        return self.mixer10.get();
    }
    pub fn get_pipe2a(&self) -> TherminolPipe {
        return self.pipe2a.get();
    }
    pub fn get_heater_top_head_1a(&self) -> TherminolCustomComponent {
        return self.heater_top_head_1a.get();
    }
    pub fn get_ciet_heater(&self) -> TherminolCustomComponent {
        return self.ciet_heater.get();
    }
    pub fn get_heater_bottom_head_1b(&self) -> TherminolCustomComponent {
        return self.heater_bottom_head_1b.get();
    }
    pub fn get_pipe18(&self) -> TherminolPipe {
        return self.pipe18.get();
    }



}

impl<'heater_branch_lifetime> FluidComponentCollectionMethods for HeaterBranch<'heater_branch_lifetime> {

    /// calculates pressure change when given a mass flowrate
    fn get_pressure_change(
        &self, 
        fluid_mass_flowrate: MassRate) -> Pressure{
        let fluid_component_collection_vector = 
            self.get_immutable_fluid_component_vector();

        let pressure_change = 
            <Self as FluidComponentCollectionSeriesAssociatedFunctions>
            ::calculate_pressure_change_from_mass_flowrate(
                fluid_mass_flowrate, 
                fluid_component_collection_vector);

        return pressure_change;
    }

    /// calculates mass flowrate from pressure change

    fn get_mass_flowrate_from_pressure_change(
        &self,
        pressure_change: Pressure) -> MassRate{


        let fluid_component_collection_vector = 
            self.get_immutable_fluid_component_vector();

        let mass_flowrate = 
            <Self as FluidComponentCollectionSeriesAssociatedFunctions>
            ::calculate_mass_flowrate_from_pressure_change(
                pressure_change, 
                fluid_component_collection_vector);

        return mass_flowrate;
    }


}

impl<'heater_branch_lifetime> FluidComponentCollection<'heater_branch_lifetime> 
for HeaterBranch<'heater_branch_lifetime> {

            fn get_immutable_fluid_component_vector(&self)
                -> &Vec<&'heater_branch_lifetime dyn FluidComponent> {

                    return &self.fluid_component_vector_immutable;
                }

            fn set_fluid_component_vector(
                &mut self, 
                fluid_component_vector: 
                Vec<&'heater_branch_lifetime dyn FluidComponent>){

                self.fluid_component_vector_immutable = 
                    fluid_component_vector;

            }

}

impl<'heater_branch_lifetime> 
FluidComponentCollectionSeriesAssociatedFunctions for HeaterBranch<'heater_branch_lifetime> {}

