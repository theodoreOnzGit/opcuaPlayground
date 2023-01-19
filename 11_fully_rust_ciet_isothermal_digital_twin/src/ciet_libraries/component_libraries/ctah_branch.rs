extern crate fluid_mechanics_rust;
use fluid_mechanics_rust::prelude::*;

use crate::{Pipe6a, StaticMixer41, CTAHVertical, CTAHHorizontal, 
    Pipe8a, StaticMixer40, Pipe9, Pipe10, Pipe11, Pipe12, CTAHPump, Pipe13, Pipe14, 
    therminol_pipe::TherminolPipe, therminol_component::TherminolCustomComponent, Pipe16, Pipe15, Branch17, Flowmeter40};

pub struct CTAHBranch<'ctah_branch_lifetime> {

    pipe6a: Pipe6a, 
    // component 6
    static_mixer_41: StaticMixer41, 
    // 7a
    ctah_vertical: CTAHVertical, 
    // 7b
    ctah_horizontal: CTAHHorizontal, 
    // 8a
    pipe_8a: Pipe8a,
    // 8
    static_mixer_40: StaticMixer40, // 8
    // 9
    pipe_9: Pipe9,
    // 10
    pipe_10: Pipe10,
    // 11
    pipe_11: Pipe11,
    //12
    pipe_12: Pipe12,
    // between 12 and 13 
    ctah_pump: CTAHPump,
    //13
    pipe_13: Pipe13,
    //14
    pipe_14: Pipe14,
    //15 
    flowmeter_40_14a: Flowmeter40,
    //16
    pipe_15: Pipe15,
    //17
    pipe_16: Pipe16,
    //18
    branch_17: Branch17,

    fluid_component_vector_immutable: 
        Vec<&'ctah_branch_lifetime dyn FluidComponent>
}

impl<'ctah_branch_lifetime> CTAHBranch<'ctah_branch_lifetime> {

    /// constructor, returns an instance of the ctah branch
    pub fn new() -> Self {

        

        // constructor will return the CTAH branch with all its items
        // but the vector will be empty
        let ctah_branch_vector_empty: Vec<&'ctah_branch_lifetime dyn FluidComponent>
            = vec![];

        return Self { 
            pipe6a: Pipe6a::new(),
            static_mixer_41: StaticMixer41::new(),
            ctah_vertical: CTAHVertical::new(),
            ctah_horizontal: CTAHHorizontal::new(),
            pipe_8a: Pipe8a::new(),
            static_mixer_40: StaticMixer40::new(),
            pipe_9: Pipe9::new(),
            pipe_10: Pipe10::new(),
            pipe_11: Pipe11::new(),
            pipe_12: Pipe12::new(),
            ctah_pump: CTAHPump::new(), 
            pipe_13: Pipe13::new(),
            pipe_14: Pipe14::new(),
            flowmeter_40_14a: Flowmeter40::new(),
            pipe_15: Pipe15::new(),
            pipe_16: Pipe16::new(),
            branch_17: Branch17::new(),
            fluid_component_vector_immutable: ctah_branch_vector_empty,
        }
    }




    /// sets the ctah pump pressure to whatever value the user specifies
    ///
    /// it basically deletes the existing ctah pump and instantiates a
    /// new one at the correct position
    ///
    pub fn set_ctah_pump_pressure(&mut self,
                                  user_specified_pressure: Pressure,
                                  ctah_pump: &'ctah_branch_lifetime mut 
                                  TherminolCustomComponent){

        // should we do max/min pressure??? IDK
        // i'll just have an actual ctah pump object
        
        // the ctah pump will be usually at 10th element of the vector starting
        // from 0


        ctah_pump.set_internal_pressure_source(user_specified_pressure);

        self.fluid_component_vector_immutable[10] = ctah_pump;
        // inside the CTAH branch i should have all my components
        // so for ease of use and readability, i may want to nest the 
        // actual component objects within the ctah branch

    }

    /// these help to return the components to the environment
    ///
    /// what you are supposed to do is first
    /// to 

    pub fn get_pipe6a(&self) -> TherminolPipe {
        return self.pipe6a.get();
    }

    pub fn get_static_mixer_41(&self) -> TherminolCustomComponent {
        return self.static_mixer_41.get();
    }

    pub fn get_ctah_vertical(&self) -> TherminolCustomComponent {
        return self.ctah_vertical.get();
    }

    pub fn get_ctah_horizontal(&self) -> TherminolCustomComponent{
        return self.ctah_horizontal.get();
    }

    pub fn get_pipe_8a(&self) -> TherminolPipe {
        return self.pipe_8a.get();
    }

    pub fn get_static_mixer_40(&self) -> TherminolCustomComponent {
        return self.static_mixer_40.get();
    }
    pub fn get_pipe_9(&self) -> TherminolPipe {
        return self.pipe_9.get();
    }

    pub fn get_pipe_10(&self) -> TherminolPipe {
        return self.pipe_10.get();
    }

    pub fn get_pipe_11(&self) -> TherminolPipe {
        return self.pipe_11.get();
    }

    pub fn get_pipe_12(&self) -> TherminolPipe {
        return self.pipe_12.get();
    }

    pub fn get_ctah_pump(&self) -> TherminolCustomComponent {
        return self.ctah_pump.get();
    }

    pub fn get_pipe_13(&self) -> TherminolPipe {
        return self.pipe_13.get();
    }
    pub fn get_pipe_14(&self) -> TherminolPipe {
        return self.pipe_14.get();
    }
    pub fn get_flowmeter_40_14a(&self) -> TherminolCustomComponent {
        return self.flowmeter_40_14a.get();
    }

    pub fn get_pipe_15(&self) -> TherminolPipe {
        return self.pipe_15.get();
    }
    pub fn get_pipe_16(&self) -> TherminolPipe {
        return self.pipe_16.get();
    }
    pub fn get_branch_17(&self) -> TherminolPipe {
        return self.branch_17.get();
    }

}

impl<'ctah_branch_lifetime> FluidComponentCollectionMethods for CTAHBranch<'ctah_branch_lifetime> {

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

impl<'ctah_branch_lifetime> FluidComponentCollection<'ctah_branch_lifetime> 
for CTAHBranch<'ctah_branch_lifetime> {

            fn get_immutable_fluid_component_vector(&self)
                -> &Vec<&'ctah_branch_lifetime dyn FluidComponent> {

                    return &self.fluid_component_vector_immutable;
                }

            fn set_fluid_component_vector(
                &mut self, 
                fluid_component_vector: 
                Vec<&'ctah_branch_lifetime dyn FluidComponent>){

                self.fluid_component_vector_immutable = 
                    fluid_component_vector;

            }

}

impl<'ctah_branch_lifetime> 
FluidComponentCollectionSeriesAssociatedFunctions for CTAHBranch<'ctah_branch_lifetime> {}

