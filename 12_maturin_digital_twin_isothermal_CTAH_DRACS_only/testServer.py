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

# this is for timing
import time


# uamethods are methods meant for clients to invoke,
# not necessarily on the server side
@uamethod
def func(parent, value):
    return value * 2

# add two to whatever the value is
def add2(value):
    return value + 2.0

def m42_height_meters(mass_flowrate_kg_per_s):
    return -20.4218536540637* mass_flowrate_kg_per_s**2 - 0.874137365300828 * mass_flowrate_kg_per_s + 1

def m43_height_meters(mass_flowrate_kg_per_s):
    return 10.1573390552631* mass_flowrate_kg_per_s**2 + 2.33678270779408 * mass_flowrate_kg_per_s + 1

def referenceDensity20C_kg_per_m3():
    # this is in kg per m3
    return 1061

def expt_pressure_loss_pascals(mass_rate_kg_per_s):
    delta_h = (m43_height_meters(mass_rate_kg_per_s) - m42_height_meters(mass_rate_kg_per_s))
    rho = referenceDensity20C_kg_per_m3()
    g = 9.81
    return delta_h * rho * g

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

def rust_get_dhx_branch_pressure_change(
    mass_rate_kg_per_s,
    temperature_degrees_c):
    return rust_functions_in_python.get_dhx_branch_isothermal_pressure_change_pascals_rust(
            mass_rate_kg_per_s,
            temperature_degrees_c)

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

    return mass_flowrate_kg_per_s_solution


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


def get_dhx_branch_mass_flowrate(
        pressure_change_pascals,
        temperature_degrees_c):

    # first let's check for reverse flow and return 0
    # flow if reverse, it's computationally cheaper
    hydrostatic_pressure = rust_get_heater_branch_hydrostatic_pressure(
            temperature_degrees_c)

    #if (pressure_change_pascals > hydrostatic_pressure ):
    #    return 0.0

    # basically im solving for the mass rate which
    # returns the correct pressure change
    def dhx_pressure_chg_root(
            mass_rate_kg_per_s):
        return rust_get_dhx_branch_pressure_change(
                mass_rate_kg_per_s,
                temperature_degrees_c) - pressure_change_pascals

    mass_flowrate_kg_per_s_solution = scipy.optimize.brentq(
            dhx_pressure_chg_root,
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


        dhx_branch_mass_flowrate = get_dhx_branch_mass_flowrate(
                        pressure_change_pascals,
                        temperature_degrees_c)

        ctah_branch_flowrate = get_ctah_branch_mass_flowrate(
                pressure_change_pascals,
                temperature_degrees_c,
                pump_pressure_pascals)


        total_mass_flowrate = numpy.add(dhx_branch_mass_flowrate,
                ctah_branch_flowrate)

        return total_mass_flowrate

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
    ## this IP address is for lab
    #server.set_endpoint('opc.tcp://192.168.10.177:4840/freeopcua/server/')

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

    ctah_pump_pressure = await pipeObj.add_variable(
            idx, 'ctah_pump_pressure_pascal',16000.0)
    await ctah_pump_pressure.set_writable()

    ciet_temperature_degC = await pipeObj.add_variable(
            idx, 'ciet_temperature_degC',
            20.0)

    ctah_flowrate = await pipeObj.add_variable(
            idx,'ctah_mass_flowrate_kg_per_s',0.0)


    # i want to add a simulation time variable
    simulation_time_ms = await pipeObj.add_variable(
            idx, 'simulation_time_ms',
            0.0)


    roughnessRatio = 0.00015

    _logger.info('Starting server!')

    # server loop
    async with server:
        while True:

            start = time.time()


            _logger.info('\n \n \n')
            _logger.info('to connect:')

            _logger.info('opc.tcp://'+getIPAddress()+':4840/freeopcua/server/')
            _logger.info('\n')


            # this is the main ciet digital twin calculation
            pump_pressure_pascals = await ctah_pump_pressure.get_value()
            fluid_temp_degC = await ciet_temperature_degC.get_value()
            mass_flowrate_kg_per_s = get_ciet_isothermal_mass_flowrate(
                    pump_pressure_pascals,
                    fluid_temp_degC)

            await ctah_flowrate.write_value(mass_flowrate_kg_per_s)

            _logger.info('pump_pressure_pascals: %.1f',
                    pump_pressure_pascals)
            _logger.info('ctah_mass_flowrate_kg_per_s: %.4f',
                    await ctah_flowrate.get_value())

            # this is the experimental ciet pressure loss at
            # 20C
            end = time.time()

            elapsed_loop_time_seconds = (end - start)
            await simulation_time_ms.write_value(
                    elapsed_loop_time_seconds*1000)
            _logger.info('elapsed_loop_time_seconds (ms) %.1f',
                    await simulation_time_ms.get_value())

            wait_time = 1.0


            sleep_time = wait_time - elapsed_loop_time_seconds



            await asyncio.sleep(sleep_time)







if __name__ == '__main__':

    logging.basicConfig(level=logging.DEBUG)

    asyncio.run(main(), debug=True)
