extern crate fluid_mechanics_rust;
use std::time::{Instant, Duration};

use fluid_mechanics_rust::prelude::*;

/// This is a struct representing the 
/// CIET facility in isothermal operation
///
/// temperature is assumed to be 21C all round
///
/// no heat transfer equations are solved
pub struct CIETIsothermalFacility {}


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
/// One simple implementation is to 
impl CIETIsothermalFacility {


    pub fn get_ctah_pump_pressure(&self) -> Pressure {
        unimplemented!();
    }

    pub fn set_ctah_pump_pressure(&mut self){
        unimplemented!();
    }

    pub fn get_ctah_branch_mass_flowrate(&self) -> MassRate {
        unimplemented!();
    }

    pub fn get_dhx_branch_mass_flowrate(&self) -> MassRate {
        unimplemented!();
    }

    pub fn get_heater_branch_mass_flowrate(&self) -> MassRate {
        unimplemented!();
    }

    pub fn calculate(&mut self) -> Duration {

        let start = Instant::now();

        // run function here to calculate ciet

        let elapsed_time: Duration= start.elapsed();

        return elapsed_time;

    }

}
