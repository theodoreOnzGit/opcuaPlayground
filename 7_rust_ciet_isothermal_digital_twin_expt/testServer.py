from asyncua import ua, Server
from asyncua.common.methods import uamethod

import logging
import asyncio
import sys
sys.path.insert(0, "..")

# importing rust functions in maturin
import rust_functions_in_python


# get ip address
import socket

# this is for iterative methods
import scipy
import numpy


# uamethods are methods meant for clients to invoke,
# not necessarily on the server side
@uamethod
def func(parent, value):
    return value * 2

# add two to whatever the value is
def add2(value):
    return value + 2.0


# calling rust functions
def rustAdd4(value):
    return rust_functions_in_python.add_four_rust(value)

# frictionFactor in rust
def rustDarcy(ReynoldsNumber,roughnessRatio):
    return rust_functions_in_python.moody_rust(ReynoldsNumber,
            roughnessRatio)

# Digital Twin Isothermal Ciet functions
# implemented in rust

# these are functions straight from rust
# they get a pressure change given a set mass flowrate
# and temperature
def rust_get_heater_branch_pressure_change(
        mass_rate_kg_per_s,
        temperature_degrees_c):
    return rust_functions_in_python.get_heater_branch_isothermal_pressure_change_pascals_rust(
            mass_rate_kg_per_s,
            temperature_degrees_c)
def rust_get_ctah_branch_pressure_change(
    mass_rate_kg_per_s,
    temperature_degrees_c,
    pump_pressure_pascals):
    return rust_functions_in_python.get_ctah_branch_isothermal_pressure_change_pascals_rust(
            mass_rate_kg_per_s,
            temperature_degrees_c,
            pump_pressure_pascals)

def rust_get_heater_branch_hydrostatic_pressure(
        temperature_degrees_c):
    return rust_functions_in_python.get_heater_branch_isothermal_hydrostatic_pressure_pascals_rust(
            temperature_degrees_c)

# these are functions utilising the rust ciet functions
# but they will give mass flowrate given a pressure change
# using brent method from scipy
# which should be fast
def get_heater_branch_mass_flowrate(
        pressure_change_pascals,
        temperature_degrees_c):

    # basically im solving for the mass rate which
    # returns the correct pressure change
    def heater_pressure_chg_root(
            mass_rate_kg_per_s):
        return rust_get_heater_branch_pressure_change(
                mass_rate_kg_per_s,
                temperature_degrees_c) - pressure_change_pascals

    mass_flowrate_kg_per_s_solution = scipy.optimize.brentq(
            heater_pressure_chg_root,
            -1.0,
            1.0)


def get_ctah_branch_mass_flowrate(
        pressure_change_pascals,
        temperature_degrees_c,
        pump_pressure_pascals):

    # basically im solving for the mass rate which
    # returns the correct pressure change
    def ctah_pressure_chg_root(
            mass_rate_kg_per_s):
        return rust_get_ctah_branch_pressure_change(
                mass_rate_kg_per_s,
                temperature_degrees_c,
                pump_pressure_pascals) - pressure_change_pascals

    mass_flowrate_kg_per_s_solution = scipy.optimize.brentq(
            ctah_pressure_chg_root,
            -1.0,
            1.0)

    return mass_flowrate_kg_per_s_solution

# these methods solve for mass flowrate given a pump pressure
# and system temperature


def get_ciet_isothermal_mass_flowrate(
        pump_pressure_pascals,
        temperature_degrees_c):
    # the job of this function is to sum up the mass
    # flowrate of the branches in ciet
    # and solve for the value where the branch flowrates
    # sum up to zero
    # the convention is positive flowrate leaving the
    # top and negative flowrate entering the top

    def pressure_change_root(pressure_change_pascals):
        # both branches must be subject to the same
        # pressure change since they are in parallel
        return get_ctah_branch_mass_flowrate(pressure_change_pascals,
                temperature_degrees_c, 
                pump_pressure_pascals) - get_heater_branch_mass_flowrate(
                        pressure_change_pascals,
                        temperature_degrees_c)


    # this scipy module will solve for the correct pressure change
    # given the mass flowrate
    # the intervals will be 50000 pascals plus or minus
    # the hydrostatic pressure change of the heater

    upper_bound = rust_get_heater_branch_hydrostatic_pressure(
            temperature_degrees_c) + 50000

    lower_bound = rust_get_heater_branch_hydrostatic_pressure(
            temperature_degrees_c) - 50000

    pressure_change_value = scipy.optimize.brentq(
            pressure_change_root,
            lower_bound,
            upper_bound)

    # once we get the pressure change root value,
    # we can get mass flowrate

    ctah_branch_mass_flowrate = get_ctah_branch_mass_flowrate(
            pressure_change_value,
            temperature_degrees_c,
            pump_pressure_pascals)


    return ctah_branch_mass_flowrate





# get ip address automatically
def getIPAddress():
    hostname = socket.gethostname()
    IPaddr = socket.gethostbyname(hostname)
    return IPaddr



async def main():
    _logger = logging.getLogger('asyncua')
    # setup our server
    server = Server()
    await server.init()
    server.set_endpoint('opc.tcp://'+getIPAddress()+':4840/freeopcua/server/')

    # setup our own namespace, not really necessary but should as spec
    uri = 'http://examples.freeopcua.github.io'
    idx = await server.register_namespace(uri)

    # populating our address space
    # server.nodes, contains links to very common nodes like objects and root
    myobj = await server.nodes.objects.add_object(idx, 'MyObject')
    myvar = await myobj.add_variable(idx, 'MyVariable', 6.7)
    # Set MyVariable to be writable by clients
    await myvar.set_writable()
    await server.nodes.objects.add_method(
            ua.NodeId('ServerMethod', 2), 
            ua.QualifiedName('ServerMethod', 2), 
            func, 
            [ua.VariantType.Int64], 
            [ua.VariantType.Int64])


    ## here's where i add my reynolds number
    pipeObj = await server.nodes.objects.add_object(idx, 'pipeObj')
    ReynoldsNumber = await pipeObj.add_variable(idx, 'Re',160.01)
    await ReynoldsNumber.set_writable()

    ctah_pump_pressure = await pipeObj.add_variable(
            idx, 'ctah_pump_pressure_pascal',0.0)
    await ctah_pump_pressure.set_writable()

    ciet_temperature_degC = await pipeObj.add_variable(
            idx, 'ciet_temperature_degC',
            20.0)

    ctah_flowrate = await pipeObj.add_variable(
            idx,'ctah_mass_flowrate_kg_per_s',0.0)

    roughnessRatio = 0.00015

    _logger.info('Starting server!')
    
    # server loop
    async with server:
        while True:
            await asyncio.sleep(1)
            new_val = await myvar.get_value() + rustAdd4(0.1)
            _logger.info('Set value of %s to %.1f', myvar, new_val)
            Re = await ReynoldsNumber.get_value()

            darcyFrictionFactor = rustDarcy(Re,
                    roughnessRatio)
            _logger.info('reynolds number is %.1f, darcy = %.7f',Re,darcyFrictionFactor)

            await myvar.write_value(new_val)

            # this is the main ciet digital twin calculation
            pump_pressure_pascals = await ctah_pump_pressure.get_value()
            fluid_temp_degC = await ciet_temperature_degC.get_value()
            mass_flowrate_kg_per_s = get_ciet_isothermal_mass_flowrate(
                    pump_pressure_pascals,
                    fluid_temp_degC)

            await ctah_flowrate.write_value(mass_flowrate_kg_per_s)

            _logger.info('pump_pressure_pascals: %.1f',
                    await pump_pressure_pascals.get_value())
            _logger.info('ctah_mass_flowrate_kg_per_s: %.4f',
                    await ctah_flowrate.get_value())








if __name__ == '__main__':

    logging.basicConfig(level=logging.DEBUG)

    asyncio.run(main(), debug=True)
