extern crate fluid_mechanics_rust;
use fluid_mechanics_rust::prelude::*;

// we will implement a few properties here for our therminol pipe
// for clarity we will list them in a
// supertrait
// This makes it easy to see what traits are being implemented here

pub trait TherminolPipeTraits<'trait_lifetime> :
ConstantCompositionSinglePhaseFluidPropertiesAssociatedFunctions<'trait_lifetime>
+ FluidComponent
+ FluidPipeCalcPressureChange
+ FluidPipeCalcPressureLoss
{}

// first we create an therminol pipe struct
// and start implementing it
pub struct TherminolPipe<'pipe_lifetime> {

    therminol_properties_reference: &'pipe_lifetime dyn FluidProperties,
    fluid_temp: ThermodynamicTemperature,
    fluid_mass_flowrate: MassRate,

    internal_pressure: Pressure,
    incline_angle: Angle,
    component_length: Length,
    hydraulic_diameter: Length,

    pressure_loss: Pressure,
    form_loss_k: f64,
    absolute_roughness: Length,
    name: String,

}

impl<'pipe_lifetime> 
TherminolPipeTraits<'pipe_lifetime> for TherminolPipe<'pipe_lifetime> {}

impl<'pipe_lifetime> 
FluidPipeCalcPressureChange for TherminolPipe<'pipe_lifetime> {
}

impl<'pipe_lifetime> 
FluidPipeCalcPressureLoss for TherminolPipe<'pipe_lifetime> {

    fn get_pipe_form_loss_k(&mut self) -> f64 {
        return self.form_loss_k;
    }

    fn get_pipe_form_loss_k_immutable(&self) -> f64 {
        return self.form_loss_k;
    }

    /// return absolute roughness for pipe
    /// for a typical copper pipe
    /// it is 0.002 mm 
    /// i did a web search
    ///
    fn get_pipe_absolute_roughness(&mut self) -> Length {
        return self.absolute_roughness;
    }

    fn get_pipe_absolute_roughness_immutable(&self) -> Length {
        return self.absolute_roughness;
    }

}

impl<'pipe_lifetime> 
FluidComponent for TherminolPipe<'pipe_lifetime>{
    fn get_pressure_loss(&mut self) -> Pressure {


        // get pipe parameters and flow conditions
        // from the get methods
        let form_loss_k = self.get_pipe_form_loss_k();
        let absolute_roughness = self.get_pipe_absolute_roughness();
        let cross_sectional_area = self.get_cross_sectional_area();
        let mass_flowrate = self.fluid_mass_flowrate;
        let hydraulic_diameter = self.get_hydraulic_diameter();
        let viscosity = self.get_fluid_viscosity();
        let density = self.get_fluid_density();
        let pipe_legnth = self.get_component_length();


        // calculate the pressure loss

        let pressure_loss = 
            Self::pipe_calc_pressure_loss(
                mass_flowrate,
                cross_sectional_area,
                hydraulic_diameter,
                viscosity,
                density,
                pipe_legnth,
                absolute_roughness,
                form_loss_k);

        // you can return the pressure loss straightaway
        // or set the struct variable first and then
        // return it

        self.pressure_loss = pressure_loss;

        return self.pressure_loss;
    }

    fn get_pressure_loss_immutable(
        &self,
        mass_flowrate: MassRate) -> Pressure {


        // get pipe parameters and flow conditions
        // from the get methods
        let form_loss_k = self.get_pipe_form_loss_k_immutable();
        let absolute_roughness = self.get_pipe_absolute_roughness_immutable();
        let cross_sectional_area = self.get_cross_sectional_area_immutable();
        let hydraulic_diameter = self.get_hydraulic_diameter_immutable();
        let viscosity = self.get_fluid_viscosity_immutable();
        let density = self.get_fluid_density_immutable();
        let pipe_legnth = self.get_component_length_immutable();


        // calculate the pressure loss

        let pressure_loss = 
            Self::pipe_calc_pressure_loss(
                mass_flowrate,
                cross_sectional_area,
                hydraulic_diameter,
                viscosity,
                density,
                pipe_legnth,
                absolute_roughness,
                form_loss_k);

        // you can return the pressure loss straightaway
        // or set the struct variable first and then
        // return it


        return pressure_loss;
    }
    fn set_pressure_loss(&mut self, pressure_loss: Pressure){
        self.pressure_loss = pressure_loss;
    }

    fn set_mass_flowrate(&mut self, mass_flowrate: MassRate){
        self.fluid_mass_flowrate = mass_flowrate;
    }

    fn get_mass_flowrate(&mut self) -> MassRate {
        // get pipe parameters and flow conditions
        // from the get methods
        let form_loss_k = self.get_pipe_form_loss_k();
        let absolute_roughness = self.get_pipe_absolute_roughness();
        let cross_sectional_area = self.get_cross_sectional_area();
        let hydraulic_diameter = self.get_hydraulic_diameter();
        let fluid_viscosity = self.get_fluid_viscosity();
        let fluid_density = self.get_fluid_density();
        let pipe_length = self.get_component_length();
        let pressure_loss = self.pressure_loss;
        let incline_angle = self.get_incline_angle();
        let internal_pressure_source = self.get_internal_pressure_source();

        let pressure_change = 
            -pressure_loss 
            + internal_pressure_source 
            + self.get_hydrostatic_pressure_change();

        let mass_flowrate = 
            Self::pipe_calculate_mass_flowrate_from_pressure_change(
                pressure_change, 
                cross_sectional_area, 
                hydraulic_diameter, 
                fluid_viscosity, 
                fluid_density, 
                pipe_length, 
                absolute_roughness, 
                form_loss_k,
                incline_angle,
                internal_pressure_source);

        // you can return the mass flowrate straightaway
        // or set the struct variable first and then
        // return it

        self.set_mass_flowrate(mass_flowrate);

        return self.fluid_mass_flowrate;

    }

    fn get_mass_flowrate_from_pressure_loss_immutable(
        &self,
        pressure_loss: Pressure) -> MassRate {
        // get pipe parameters and flow conditions
        // from the get methods
        let form_loss_k = self.get_pipe_form_loss_k_immutable();
        let absolute_roughness = self.get_pipe_absolute_roughness_immutable();
        let cross_sectional_area = self.get_cross_sectional_area_immutable();
        let hydraulic_diameter = self.get_hydraulic_diameter_immutable();
        let fluid_viscosity = self.get_fluid_viscosity_immutable();
        let fluid_density = self.get_fluid_density_immutable();
        let pipe_length = self.get_component_length_immutable();
        let incline_angle = self.get_incline_angle_immutable();
        let internal_pressure_source = self.get_internal_pressure_source_immutable();

        let pressure_change = 
            -pressure_loss 
            + internal_pressure_source 
            + <Self as FluidPipeCalcPressureChange>::
            get_hydrostatic_pressure_change(
                pipe_length,
                incline_angle,
                fluid_density);

        let mass_flowrate = 
            Self::pipe_calculate_mass_flowrate_from_pressure_change(
                pressure_change, 
                cross_sectional_area, 
                hydraulic_diameter, 
                fluid_viscosity, 
                fluid_density, 
                pipe_length, 
                absolute_roughness, 
                form_loss_k,
                incline_angle,
                internal_pressure_source);

        // you can return the mass flowrate straightaway
        // or set the struct variable first and then
        // return it


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
for TherminolPipe<'pipe_lifetime>{

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

impl<'pipe_lifetime> TherminolPipe<'pipe_lifetime>{

    // let's implement a generic constructor
    pub fn new(name: &str,
           fluid_temp: ThermodynamicTemperature,
           incline_angle: Angle,
           component_length: Length,
           hydraulic_diameter: Length,
           form_loss_k: f64,
           absolute_roughness: Length,
           therminol_properties_reference: &'pipe_lifetime TherminolVP1Properties) -> Self {

        return Self { 
            name: name.to_string(),
            therminol_properties_reference: therminol_properties_reference,
            fluid_temp: fluid_temp, 
            fluid_mass_flowrate: MassRate::new::<kilogram_per_second>(0.0), 
            internal_pressure: Pressure::new::<pascal>(0.0), 
            incline_angle: incline_angle, 
            component_length: component_length ,
            hydraulic_diameter: hydraulic_diameter ,
            pressure_loss: Pressure::new::<pascal>(0.0),
            form_loss_k: form_loss_k ,
            absolute_roughness: absolute_roughness,
        };



    }
    pub fn get_name(&self) -> &str {
        return &self.name;
    }

    pub fn set_name(&mut self, name: &str) {

        self.name = name.to_string();
    }
}
