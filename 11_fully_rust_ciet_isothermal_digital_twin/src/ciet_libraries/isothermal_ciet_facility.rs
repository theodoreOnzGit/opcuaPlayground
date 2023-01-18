extern crate fluid_mechanics_rust;
use std::time::{Instant, Duration};
use crate::{ctah_branch::*, therminol_component::TherminolCustomComponent, HeaterBranch, DHXBranch, heater_branch, dhx_branch};

use fluid_mechanics_rust::prelude::*;

/// This is a struct representing the 
/// CIET facility in isothermal operation
///
/// temperature is assumed to be 21C all round
///
/// no heat transfer equations are solved
pub struct CIETIsothermalFacility<'ciet_collection_lifetime> {

    ctah_pump_pressure: Pressure,
    ctah_branch_mass_flowrate: MassRate,
    dhx_branch_mass_flowrate: MassRate,
    heater_branch_mass_flowrate: MassRate,

    super_collection_vector_immutable: 
        Vec<&'ciet_collection_lifetime dyn FluidComponentCollectionMethods>,

    ctah_branch: CTAHBranch<'ciet_collection_lifetime>,
    heater_branch: HeaterBranch<'ciet_collection_lifetime>,
    dhx_branch: DHXBranch<'ciet_collection_lifetime>




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

    #[inline]
    pub fn set_ctah_pump_pressure(
        &mut self, 
        user_specified_pressure: Pressure,
        mutable_ctah_pump: &'ciet_collection_lifetime mut TherminolCustomComponent){

        self.ctah_pump_pressure = user_specified_pressure;
        self.ctah_branch.set_ctah_pump_pressure(
            user_specified_pressure, mutable_ctah_pump);

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

    pub fn calculate(&'ciet_collection_lifetime mut self,
                     ctah_pump_pressure: Pressure,
                     mutable_ctah_pump: 
                     &'ciet_collection_lifetime mut TherminolCustomComponent) -> Duration {

        let start = Instant::now();

        // run function here to calculate ciet

        // firstly get a function to calculate mass flowrate given the
        // internal pressure of ctah pump
        //
        // we use a super collection of fluid components
        // ie a parallel collection of three branches
        // feed in a mass flowrate of zero
        // and solve the equation for pressure change

        

        // (a) set the internal pressure of the ctah pump
        // use some specialised set method to get this done
        
        self.set_ctah_pump_pressure(ctah_pump_pressure, 
                                    mutable_ctah_pump);

        // i'll then prep the vectors
        let ciet_branch_vectors:
            Vec<&'ciet_collection_lifetime dyn FluidComponentCollectionMethods>
            = vec![
            &self.ctah_branch,
            &self.heater_branch,
            &self.dhx_branch ];

        self.super_collection_vector_immutable.clear();
        self.super_collection_vector_immutable = ciet_branch_vectors;



        // (b) set the mass flowrate over the entire parallel super collection to be
        // zero and obtain the pressure change
        //
        // i'm basically treating ciet's branches as one parallel collection of three
        // branches
        // and i'm saying the net flowrate through the branches is zero
        
        let zero_mass_flow = MassRate::new::<kilogram_per_second>(0.0);

        let pressure_change = self.get_pressure_change(zero_mass_flow);


        // second, using the pressure change we found,
        // find the individual branch flowrates
        // so i want concrete branch objects here 
        // to calcualte pressure change and set the flowrates accordingly
        // and pretty much we are done
        //
        // so i will be getting my branch objects and invoking the
        // get mass flowrate function from them

        let ctah_branch_flowrate = self.ctah_branch.
            get_mass_flowrate_from_pressure_change(pressure_change);

        let heater_branch_flowrate = self.heater_branch.
            get_mass_flowrate_from_pressure_change(pressure_change);

        let dhx_branch_flowrate = self.dhx_branch.
            get_mass_flowrate_from_pressure_change(pressure_change);

        self.ctah_branch_mass_flowrate = ctah_branch_flowrate;
        self.heater_branch_mass_flowrate = heater_branch_flowrate;
        self.dhx_branch_mass_flowrate = dhx_branch_flowrate;



        // now that i've gotten all the calculations, i can return the
        // elapsed time to the environment
        

        let elapsed_time: Duration= start.elapsed();

        return elapsed_time;

    }


    // constructor

    pub fn new(ctah_branch: CTAHBranch<'ciet_collection_lifetime>,
               heater_branch: HeaterBranch<'ciet_collection_lifetime>,
               dhx_branch: DHXBranch<'ciet_collection_lifetime>) -> Self {

        // again here we have an empty vector and we move ownership of
        // the dhx branch to this vector
        //

        
        return Self { 
            ctah_pump_pressure: Pressure::new::<pascal>(0.0), 
            ctah_branch_mass_flowrate: MassRate::new::<kilogram_per_second>(0.0), 
            dhx_branch_mass_flowrate: MassRate::new::<kilogram_per_second>(0.0), 
            heater_branch_mass_flowrate: MassRate::new::<kilogram_per_second>(0.0), 
            super_collection_vector_immutable: vec![], 
            ctah_branch: ctah_branch, 
            heater_branch: heater_branch, 
            dhx_branch: dhx_branch 
        }

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

            return &self.super_collection_vector_immutable;
        }

    fn set_vector(
        &mut self,
        fluid_component_super_vector: 
        Vec<&'ciet_object_lifetime dyn FluidComponentCollectionMethods>){
        self.super_collection_vector_immutable = fluid_component_super_vector;

    }

}

impl<'ciet_object_lifetime> 
FluidComponentCollectionMethods for CIETIsothermalFacility<'ciet_object_lifetime> {


    /// calculates pressure change when given a mass flowrate
    fn get_pressure_change(
        &self, 
        fluid_mass_flowrate: MassRate) -> Pressure{
        let fluid_component_collection_vector = 
            self.get_immutable_vector();

        let pressure_change = 
            <Self as FluidComponentSuperCollectionParallelAssociatedFunctions>
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
            self.get_immutable_vector();

        let mass_flowrate = 
            <Self as FluidComponentSuperCollectionParallelAssociatedFunctions>
            ::calculate_mass_flowrate_from_pressure_change(
                pressure_change, 
                fluid_component_collection_vector);

        return mass_flowrate;
    }

}





