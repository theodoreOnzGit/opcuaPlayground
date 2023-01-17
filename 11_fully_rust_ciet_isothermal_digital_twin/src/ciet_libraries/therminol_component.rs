extern crate fluid_mechanics_rust;
use fluid_mechanics_rust::prelude::*;




// we will implement a few properties here for our therminol pipe
// for clarity we will list them in a
// supertrait
// This makes it easy to see what traits are being implemented here

pub trait TherminolCustomComponentTraits<'trait_lifetime> :
ConstantCompositionSinglePhaseFluidPropertiesAssociatedFunctions<'trait_lifetime>
+ FluidComponent
+ FluidCustomComponentCalcPressureLoss<'trait_lifetime>
+ FluidCustomComponentCalcPressureChange<'trait_lifetime>
{}

// first we create an therminol pipe struct
// and start implementing it
pub struct TherminolCustomComponent<'pipe_lifetime> {

    therminol_properties_reference: &'pipe_lifetime dyn FluidProperties,
    fluid_temp: ThermodynamicTemperature,
    fluid_mass_flowrate: MassRate,

    internal_pressure: Pressure,
    incline_angle: Angle,
    component_length: Length,
    hydraulic_diameter: Length,

    pressure_loss: Pressure,
    absolute_roughness: Length,
    name: String,

    custom_k: &'pipe_lifetime dyn Fn(f64) -> f64,
    custom_darcy: &'pipe_lifetime dyn Fn(f64,f64) ->f64,

}

impl<'pipe_lifetime> 
TherminolCustomComponentTraits<'pipe_lifetime> for TherminolCustomComponent<'pipe_lifetime> {}

impl<'pipe_lifetime> 
FluidCustomComponentCalcPressureChange<'pipe_lifetime> 
for TherminolCustomComponent<'pipe_lifetime> {
}

impl<'pipe_lifetime> 
FluidCustomComponentCalcPressureLoss<'pipe_lifetime> 
for TherminolCustomComponent<'pipe_lifetime> {

    fn get_custom_component_absolute_roughness(
        &mut self) -> Length {

        return self.absolute_roughness;
    }

    fn get_custom_component_absolute_roughness_immutable(
        &self) -> Length {

        return self.absolute_roughness;
    }

    fn get_custom_darcy(&mut self) 
        -> &dyn Fn(f64, f64) -> f64 {

            return self.custom_darcy.clone();

        }


    fn get_custom_darcy_immutable(&self) 
        -> &dyn Fn(f64, f64) -> f64 {

            return self.custom_darcy.clone();

        }

    fn get_custom_k(&mut self) 
        -> &dyn Fn(f64) -> f64 {

            return self.custom_k.clone();

        }

    fn get_custom_k_immutable(&self) 
        -> &dyn Fn(f64) -> f64 {

            return self.custom_k.clone();

        }

    fn set_custom_k(
        &mut self,
        custom_k: &'pipe_lifetime dyn Fn(f64) -> f64){

        self.custom_k = custom_k;

    }

    fn set_custom_darcy(
        &mut self,
        custom_darcy: &'pipe_lifetime dyn Fn(f64,f64) -> f64){

        self.custom_darcy = custom_darcy;
    }




}

impl<'pipe_lifetime> 
FluidComponent for TherminolCustomComponent<'pipe_lifetime>{
    fn get_pressure_loss(&mut self) -> Pressure {

        let fluid_mass_flowrate = 
            self.fluid_mass_flowrate;

        let cross_sectional_area = 
            self.get_cross_sectional_area();

        let hydraulic_diameter = 
            self.get_hydraulic_diameter();

        let fluid_viscosity = 
            self.get_fluid_viscosity();

        let fluid_density = 
            self.get_fluid_density();

        let component_length = 
            self.get_component_length();

        let absolute_roughness = 
            self.get_custom_component_absolute_roughness();

        // i need to make some immutable borrows here...
        let custom_darcy: &dyn Fn(f64, f64) -> f64 = 
            self.custom_darcy;

        let custom_k : &dyn Fn(f64) -> f64 =
            self.custom_k;

        let pressure_loss =
            Self::
            fluid_custom_component_calc_pressure_loss(
                fluid_mass_flowrate, 
                cross_sectional_area, 
                hydraulic_diameter, 
                fluid_viscosity, 
                fluid_density, 
                component_length, 
                absolute_roughness, 
                custom_darcy, custom_k);

        self.pressure_loss = pressure_loss;

        return pressure_loss;


    }

    fn get_pressure_loss_immutable(
        &self,
        mass_flowrate: MassRate) -> Pressure {

        let fluid_mass_flowrate = 
            mass_flowrate;

        let cross_sectional_area = 
            self.get_cross_sectional_area_immutable();

        let hydraulic_diameter = 
            self.get_hydraulic_diameter_immutable();

        let fluid_viscosity = 
            self.get_fluid_viscosity_immutable();

        let fluid_density = 
            self.get_fluid_density_immutable();

        let component_length = 
            self.get_component_length_immutable();

        let absolute_roughness = 
            self.get_custom_component_absolute_roughness_immutable();

        // i need to make some immutable borrows here...
        let custom_darcy: &dyn Fn(f64, f64) -> f64 = 
            self.custom_darcy;

        let custom_k : &dyn Fn(f64) -> f64 =
            self.custom_k;

        let pressure_loss =
            Self:: fluid_custom_component_calc_pressure_loss(
                fluid_mass_flowrate, 
                cross_sectional_area, 
                hydraulic_diameter, 
                fluid_viscosity, 
                fluid_density, 
                component_length, 
                absolute_roughness, 
                custom_darcy, custom_k);


        return pressure_loss;

    }
    fn set_pressure_loss(&mut self, pressure_loss: Pressure){
        self.pressure_loss = pressure_loss;
    }

    fn set_mass_flowrate(&mut self, mass_flowrate: MassRate){
        self.fluid_mass_flowrate = mass_flowrate;
    }

    fn get_mass_flowrate(&mut self) -> MassRate {


        //i'll have to get the pressure change
        //
        // pressure_change = 
        // - pressure_change
        // + hydrostatic pressure change
        // + internal pressure source
        //

        // internal pressure source
        let internal_pressure_source = 
            self.get_internal_pressure_source();

        // hydrostatic pressure
        let incline_angle = 
            self.get_incline_angle();

        let hydrostatic_pressure_change =
            self.get_hydrostatic_pressure_change();

        // pressure_loss term
        //
        //
        let pressure_loss = 
            self.get_pressure_loss();

        // now we get pressure change

        let pressure_change =
            - pressure_loss
            + hydrostatic_pressure_change
            + internal_pressure_source;

        let custom_darcy : &dyn Fn(f64, f64) -> f64 = 
            self.custom_darcy;

        let custom_k : &dyn Fn(f64) -> f64 =
            self.custom_k;


        let cross_sectional_area = 
            self.get_cross_sectional_area();

        let hydraulic_diameter = 
            self.get_hydraulic_diameter();

        let fluid_viscosity = 
            self.get_fluid_viscosity();

        let fluid_density = 
            self.get_fluid_density();

        let component_length = 
            self.get_component_length();

        let absolute_roughness = 
            self.get_custom_component_absolute_roughness();

        let source_pressure = 
            self.get_internal_pressure_source();

        let mass_flowrate =
            Self::
            fluid_custom_component_calc_mass_flowrate_from_pressure_change(
                pressure_change, 
                cross_sectional_area, 
                hydraulic_diameter, 
                fluid_viscosity, 
                fluid_density, 
                component_length, 
                absolute_roughness, 
                incline_angle, 
                source_pressure, 
                custom_darcy, 
                custom_k);

        self.fluid_mass_flowrate = mass_flowrate;

        return mass_flowrate;
    }

    fn get_mass_flowrate_from_pressure_loss_immutable(
        &self,
        pressure_loss: Pressure) -> MassRate {


        //i'll have to get the pressure change
        //
        // pressure_change = 
        // - pressure_change
        // + hydrostatic pressure change
        // + internal pressure source
        //

        // internal pressure source
        let internal_pressure_source = 
            self.get_internal_pressure_source_immutable();

        // hydrostatic pressure

        let incline_angle = 
            self.get_incline_angle_immutable();


        let hydrostatic_pressure_change =
            self.get_hydrostatic_pressure_change_immutable();


        // now we get pressure change

        let pressure_change =
            - pressure_loss
            + hydrostatic_pressure_change
            + internal_pressure_source;

        let custom_darcy : &dyn Fn(f64, f64) -> f64 = 
            self.custom_darcy;

        let custom_k : &dyn Fn(f64) -> f64 =
            self.custom_k;


        let cross_sectional_area = 
            self.get_cross_sectional_area_immutable();

        let hydraulic_diameter = 
            self.get_hydraulic_diameter_immutable();

        let fluid_viscosity = 
            self.get_fluid_viscosity_immutable();

        let fluid_density = 
            self.get_fluid_density_immutable();

        let component_length = 
            self.get_component_length_immutable();

        let absolute_roughness = 
            self.get_custom_component_absolute_roughness_immutable();

        let source_pressure = 
            self.get_internal_pressure_source_immutable();

        let mass_flowrate =
            Self::
            fluid_custom_component_calc_mass_flowrate_from_pressure_change(
                pressure_change, 
                cross_sectional_area, 
                hydraulic_diameter, 
                fluid_viscosity, 
                fluid_density, 
                component_length, 
                absolute_roughness, 
                incline_angle, 
                source_pressure, 
                custom_darcy, 
                custom_k);

        return mass_flowrate;
    }

    fn get_cross_sectional_area(&mut self) -> Area {
        return self.get_hydraulic_diameter()*
            self.get_hydraulic_diameter()*
            PI/4.0_f64;
    }

    fn get_cross_sectional_area_immutable(&self) -> Area {
        return self.get_hydraulic_diameter_immutable()*
            self.get_hydraulic_diameter_immutable()*
            PI/4.0_f64;
    }

    fn get_hydraulic_diameter(&mut self) -> Length {

        return self.hydraulic_diameter;

    }

    fn get_hydraulic_diameter_immutable(&self) -> Length {


        return self.hydraulic_diameter;

    }


    fn get_fluid_viscosity(&mut self) -> DynamicViscosity {

        // get fluid temp first
        let fluid_temp = self.get_fluid_temp();

        // then the fluid properties

        let fluid_properties = self.get_fluid_properties();

        // let's get viscosity

        let fluid_viscosity = 
            Self::viscosity(fluid_temp, fluid_properties);

        return fluid_viscosity;


    }

    fn get_fluid_viscosity_immutable(&self) -> DynamicViscosity {


        // get fluid temp first
        let fluid_temp = self.get_fluid_temp();

        // then the fluid properties

        let fluid_properties = self.get_fluid_properties();

        // let's get viscosity

        let fluid_viscosity = 
            Self::viscosity(fluid_temp, fluid_properties);

        return fluid_viscosity;



    }

    fn get_fluid_density(&mut self) -> MassDensity {

        // get fluid temp first
        let fluid_temp = self.get_fluid_temp();

        // then the fluid properties

        let fluid_properties = self.get_fluid_properties();

        // let's get density

        let fluid_density = 
            Self::density(fluid_temp, fluid_properties);

        return fluid_density;


    }

    fn get_fluid_density_immutable(&self) -> MassDensity {


        // get fluid temp first
        let fluid_temp = self.get_fluid_temp();

        // then the fluid properties

        let fluid_properties = self.get_fluid_properties();

        // let's get density

        let fluid_density = 
            Self::density(fluid_temp, fluid_properties);

        return fluid_density;



    }

    fn get_component_length(&mut self) -> Length {

        return self.component_length;
    }

    fn get_component_length_immutable(&self) -> Length {

        return self.component_length;
    }

    fn get_incline_angle(&mut self) -> Angle {

        return self.incline_angle;
    }

    fn get_incline_angle_immutable(&self) -> Angle {

        return self.incline_angle;
    }



    fn get_internal_pressure_source(&mut self) -> Pressure {

        return self.internal_pressure;
    }

    fn get_internal_pressure_source_immutable(&self) -> Pressure {

        return self.internal_pressure;
    }

    fn set_internal_pressure_source(&mut self,
                                    internal_pressure: Pressure){

        self.internal_pressure = internal_pressure;
    }

}

impl<'pipe_lifetime> 
ConstantCompositionSinglePhaseFluidPropertiesAssociatedFunctions<'pipe_lifetime>
for TherminolCustomComponent<'pipe_lifetime>{

    fn get_fluid_properties(&self) -> &'pipe_lifetime dyn FluidProperties {

        return self.therminol_properties_reference;

    }

    fn set_fluid_properties(&mut self,
                            fluid_properties: &'pipe_lifetime dyn FluidProperties){

        self.therminol_properties_reference = fluid_properties;

    }

    fn get_fluid_temp(&self) -> ThermodynamicTemperature {

        return self.fluid_temp;

    }

    fn set_fluid_temp(&mut self,
                      fluid_temp: ThermodynamicTemperature){

        self.fluid_temp = fluid_temp;

    }
}

impl<'pipe_lifetime> TherminolCustomComponent<'pipe_lifetime>{

    // let's implement a generic constructor
    pub fn new(fluid_temp: ThermodynamicTemperature,
           incline_angle: Angle,
           component_length: Length,
           hydraulic_diameter: Length,
           absolute_roughness: Length,
           therminol_properties_reference: &'pipe_lifetime TherminolVP1Properties,
           custom_k: &'pipe_lifetime dyn Fn(f64)-> f64 ,
           custom_darcy: &'pipe_lifetime dyn Fn(f64,f64) -> f64 ) -> Self {

        return Self { 
            name: "pipe_1".to_string(),
            therminol_properties_reference: therminol_properties_reference,
            fluid_temp: fluid_temp, 
            fluid_mass_flowrate: MassRate::new::<kilogram_per_second>(0.0), 
            internal_pressure: Pressure::new::<pascal>(0.0), 
            incline_angle: incline_angle, 
            component_length: component_length ,
            hydraulic_diameter: hydraulic_diameter ,
            pressure_loss: Pressure::new::<pascal>(0.0),
            absolute_roughness: absolute_roughness,
            custom_k: custom_k,
            custom_darcy: custom_darcy,
        };

    }

    pub fn get_name(&self) -> &str {
        return &self.name;
    }

    pub fn set_name(&mut self, name: &str) {

        self.name = name.to_string();
    }

}

