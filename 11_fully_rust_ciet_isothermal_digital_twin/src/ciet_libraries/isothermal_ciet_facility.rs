extern crate fluid_mechanics_rust;
use std::time::{Instant, Duration};
use crate::{ctah_branch::*, therminol_component::TherminolCustomComponent, HeaterBranch, 
    DHXBranch, heater_branch, dhx_branch};

use fluid_mechanics_rust::prelude::*;
extern crate roots;
use roots::find_root_brent;
use roots::SimpleConvergency;

/// This is a struct representing the 
/// CIET facility in isothermal operation
///
/// temperature is assumed to be 21C all round
///
/// no heat transfer equations are solved
pub struct CIETIsothermalFacility<'ciet_collection_lifetime> {

    pub ctah_pump_pressure: Pressure,
    pub ctah_branch_mass_flowrate: MassRate,
    pub dhx_branch_mass_flowrate: MassRate,
    pub heater_branch_mass_flowrate: MassRate,

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


    pub fn calculate(&'ciet_collection_lifetime mut self) -> 
        (Duration,MassRate,MassRate,MassRate)
        {

            // start the timer
            let start = Instant::now();

            // i'm using the same algorithm from ciet digital twin v1

            let pressure_change_root = 
                |pressure_change_pascals: f64| -> f64 {

                    // let's get heater branch mass flowrate
                    // given the iterated pressure change
                    //
                    let test_pressure_change = 
                        Pressure::new::<pascal>(pressure_change_pascals);

                    let heater_branch_mass_flowrate = 
                        self.heater_branch.
                        get_mass_flowrate_from_pressure_change(
                            test_pressure_change);

                    // here is where i implement the check valve behaviour

                    let dhx_branch_hydrostatic_pressure = 
                        self.dhx_branch.
                        get_pressure_change(MassRate::new::<kilogram_per_second>(0.0));

                    let mut dhx_branch_mass_flowrate = 
                        MassRate::new::<kilogram_per_second>(0.0);

                    if test_pressure_change.value > dhx_branch_hydrostatic_pressure.value {

                        // check valve behaviour here, if the pressure change is more than
                        // hydrostatic pressure
                        // mass flowrate is set to zero
                        // or rather, left at zero
                    } else {

                        dhx_branch_mass_flowrate = 
                            self.dhx_branch.
                            get_mass_flowrate_from_pressure_change(
                                test_pressure_change);

                    }



                    
                    let ctah_branch_mass_flowrate = 
                        self.ctah_branch.
                        get_mass_flowrate_from_pressure_change(
                            test_pressure_change);

                    let total_mass_flowrate = 
                        heater_branch_mass_flowrate 
                        + dhx_branch_mass_flowrate
                        + ctah_branch_mass_flowrate;

                    return total_mass_flowrate.value;


                };

            let zero_flowrate = MassRate::new::<kilogram_per_second>(0.0);

            let upper_bound = self.heater_branch.
                get_pressure_change(zero_flowrate) +
                Pressure::new::<pascal>(50000_f64);

            let lower_bound = self.heater_branch.
                get_pressure_change(zero_flowrate) +
                Pressure::new::<pascal>(-50000_f64);


            let mut convergency = SimpleConvergency { eps:1e-9_f64, max_iter:30 };

            // the buggy bit...
            let pressure_change_value 
                = find_root_brent(
                    upper_bound.value,
                    lower_bound.value,
                    &pressure_change_root,
                    &mut convergency).unwrap();

            //let pressure_change_value = 0.0;

            let pressure_change = 
                Pressure::new::<pascal>(pressure_change_value);

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

            return (elapsed_time,
                    ctah_branch_flowrate,
                    heater_branch_flowrate,
                    dhx_branch_flowrate);

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
    fn ciet_v1_algorithm_incomplete(&self){
        let pressure_change = Pressure::new::<pascal>(0.0);
        // i'm using the same algorithm from ciet digital twin v1

        let pressure_change_root = 
            |pressure_change_pascals: f64| -> f64 {

                // let's get heater branch mass flowrate
                // given the iterated pressure change
                //
                let test_pressure_change = 
                    Pressure::new::<pascal>(pressure_change_pascals);

                let heater_branch_mass_flowrate = 
                    self.heater_branch.
                    get_mass_flowrate_from_pressure_change(
                        test_pressure_change);

                let dhx_branch_mass_flowrate = 
                    self.dhx_branch.
                    get_mass_flowrate_from_pressure_change(
                        test_pressure_change);

                let ctah_branch_mass_flowrate = 
                    self.ctah_branch.
                    get_mass_flowrate_from_pressure_change(
                        test_pressure_change);

                let total_mass_flowrate = 
                    heater_branch_mass_flowrate 
                    + dhx_branch_mass_flowrate
                    + ctah_branch_mass_flowrate;

                return total_mass_flowrate.value;


            };

        let zero_flowrate = MassRate::new::<kilogram_per_second>(0.0);

        let upper_bound = self.heater_branch.
            get_pressure_change(zero_flowrate) +
            Pressure::new::<pascal>(50000_f64);

        let lower_bound = self.heater_branch.
            get_pressure_change(zero_flowrate) +
            Pressure::new::<pascal>(-50000_f64);

        let mut convergency = SimpleConvergency { eps:1e-9_f64, max_iter:30 };

        let mass_flowrate_result 
            = find_root_brent(
                upper_bound.value,
                lower_bound.value,
                &pressure_change_root,
                &mut convergency);

        //return MassRate::new::<kilogram_per_second>(
        //    mass_flowrate_result.unwrap());
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





