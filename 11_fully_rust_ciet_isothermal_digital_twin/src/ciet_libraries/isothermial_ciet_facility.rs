extern crate fluid_mechanics_rust;
use std::time::{Instant, Duration};

use fluid_mechanics_rust::prelude::*;

/// This is a struct representing the 
/// CIET facility in isothermal operation
///
/// temperature is assumed to be 21C all round
///
/// no heat transfer equations are solved
pub struct CIETIsothermalFacility<'ciet_object_lifetime> {

    ctah_pump_pressure: Pressure,
    ctah_branch_mass_flowrate: MassRate,
    dhx_branch_mass_flowrate: MassRate,
    heater_branch_mass_flowrate: MassRate,

    super_collection_vector_immutable: 
        Vec<&'ciet_object_lifetime dyn FluidComponentCollectionMethods>

}


/// i also "inherit" traits from my supercollection from
/// fluid_mechanics_rust
///
/// basically i need CIETIsothermalFacility to act a as a super
/// collection of components
/// the reason why this is here is so that i can get the 
/// pressure change of the branch given a mass flowrate
pub trait ParallelSuperCollection<'trait_lifetime> : 
FluidComponentSuperCollection<'trait_lifetime> +
FluidComponentSuperCollectionParallelAssociatedFunctions {}


/// for this object,
///
/// i want to be able to 
/// (1) set the ctah pump pressure
/// (2) obtain flowrate readings for all flowmeters
/// (3) obtain calculation time for the calculations as a function
/// (4) the user should be able to execute value calculation via a function is called
///
///
/// i would have liked to, but an not doing:
/// (1) attain manometer readings if possible
/// This is not quite critical for dissertation purposes
/// and would take quite some effort
///
///
impl<'ciet_collection_lifetime> CIETIsothermalFacility<'ciet_collection_lifetime> {


    pub fn get_ctah_pump_pressure(&self) -> Pressure {
        return self.ctah_pump_pressure;
    }

    pub fn set_ctah_pump_pressure(
        &mut self, user_specified_pressure: Pressure){
        self.ctah_pump_pressure = user_specified_pressure;
    }

    pub fn get_ctah_branch_mass_flowrate(&self) -> MassRate {
        return self.ctah_branch_mass_flowrate;
    }

    pub fn get_dhx_branch_mass_flowrate(&self) -> MassRate {
        return self.dhx_branch_mass_flowrate;
    }

    pub fn get_heater_branch_mass_flowrate(&self) -> MassRate {
        return self.heater_branch_mass_flowrate;
    }

    pub fn calculate(&mut self) -> Duration {

        let start = Instant::now();

        // run function here to calculate ciet

        // firstly get a function to calculate mass flowrate given the
        // internal pressure of ctah pump
        //
        // we use a super collection of fluid components
        // ie a parallel collection of three branches
        // feed in a mass flowrate of zero
        // and solve the equation for pressure change


        // second, using the pressure change we found,
        // find the individual branch flowrates
        // so i want concrete branch objects here 
        // to calcualte pressure change and set the flowrates accordingly
        // and pretty much we are done





        // i'll feed this into the loop somehow

        let elapsed_time: Duration= start.elapsed();

        return elapsed_time;

    }


    

}

/// the CIET isothermal facility must implement the parallel super
/// collection supertrait i just defined
impl<'ciet_object_lifetime> 
ParallelSuperCollection<'ciet_object_lifetime> for CIETIsothermalFacility<'ciet_object_lifetime> {}

impl<'ciet_object_lifetime> FluidComponentSuperCollectionParallelAssociatedFunctions for 
CIETIsothermalFacility<'ciet_object_lifetime> {}

impl<'ciet_object_lifetime> FluidComponentSuperCollection<'ciet_object_lifetime> 
for CIETIsothermalFacility<'ciet_object_lifetime> {

    fn get_immutable_vector(&self) 
        -> &Vec<&'ciet_object_lifetime dyn FluidComponentCollectionMethods>{

            unimplemented!();
        }

    fn set_vector(
        &mut self,
        fluid_component_super_vector: 
        Vec<&'ciet_object_lifetime dyn FluidComponentCollectionMethods>){
        unimplemented!();

    }

}

impl<'ciet_object_lifetime> 
FluidComponentCollectionMethods for CIETIsothermalFacility<'ciet_object_lifetime> {


    /// calculates pressure change when given a mass flowrate
    fn get_pressure_change(
        &self, 
        fluid_mass_flowrate: MassRate) -> Pressure{
        unimplemented!();
    }

    /// calculates mass flowrate from pressure change

    fn get_mass_flowrate_from_pressure_change(
        &self,
        pressure_change: Pressure) -> MassRate{

        unimplemented!();
    }

}





