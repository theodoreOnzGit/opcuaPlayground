extern crate fluid_mechanics_rust;
use fluid_mechanics_rust::prelude::*;

use crate::{Branch5, 
    therminol_pipe::TherminolPipe, therminol_component::TherminolCustomComponent, 
    Pipe4, Pipe3, StaticMixer10, Pipe2a, HeaterTopHead1a, 
    CietHeaterVersion1, HeaterBottomHead1b, Pipe18};
extern crate roots;
use roots::find_root_brent;
use roots::SimpleConvergency;

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

    /// constructor, returns an instance of the heater branch
    pub fn new() -> Self {

        let empty_vec: Vec<&'heater_branch_lifetime dyn FluidComponent> = vec![];
        

        // constructor will return the heater branch with all its factories
        // but the vector will be empty
        //

        Self {
            branch5: Branch5::new(),
            pipe4: Pipe4::new(),
            pipe3: Pipe3::new(),
            mixer10: StaticMixer10::new(),
            pipe2a: Pipe2a::new(),
            heater_top_head_1a: HeaterTopHead1a::new(),
            ciet_heater: CietHeaterVersion1::new(),
            heater_bottom_head_1b: HeaterBottomHead1b::new(),
            pipe18: Pipe18::new(),
            fluid_component_vector_immutable: empty_vec
        }
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


        // i'm keeping bounds artificially low for ciet
        // -1 or +1 kg/s
        let upper_bound = MassRate::new::<kilogram_per_second>(1.0);


        let lower_bound = MassRate::new::<kilogram_per_second>(-1.0);


        // now we have a function comparing the pressure change
        // to the pressure change of the calculated value

        let mass_flow_from_pressure_chg_root = 
            |mass_flow_kg_per_s: f64| -> f64 {

            let mass_flow_kg_per_s_double = mass_flow_kg_per_s; 

            let mass_rate = 
                MassRate::new::<kilogram_per_second>(
                    mass_flow_kg_per_s_double);


            let pressure_change_tested = 
                Self::calculate_pressure_change_from_mass_flowrate(
                mass_rate, 
                fluid_component_collection_vector);

            // now i've obtained the pressure change, i convert it to f64

            let pressure_change_user_stipulated_pascals_f64 = 
                pressure_change.value;

            // since we are finding root, then we must also
            // subtract it from our pressure change value


            let pressure_change_error: f64 =
                pressure_change_user_stipulated_pascals_f64 - 
                pressure_change_tested.value;

            return pressure_change_error;

        };

        let mut convergency = SimpleConvergency { eps:1e-9_f64, max_iter:30 };

        let mass_flowrate_result 
            = find_root_brent(
                upper_bound.value,
                lower_bound.value,
                &mass_flow_from_pressure_chg_root,
                &mut convergency);

        return MassRate::new::<kilogram_per_second>(mass_flowrate_result.unwrap());
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

