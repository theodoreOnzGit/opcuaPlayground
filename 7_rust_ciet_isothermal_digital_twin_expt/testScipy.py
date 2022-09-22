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
import numpy as np

# this is for timing purposes

import time


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
                pump_pressure_pascals) + get_heater_branch_mass_flowrate(
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
    # just want to check if the ctah pressure loss values make sense
    print("hello world")
    tempC = 21.0



    # for timing
    # https://stackoverflow.com/questions/7370801/how-do-i-measure-elapsed-time-in-python

    start = time.time()
    hydrostaticPressureChg = rust_get_heater_branch_hydrostatic_pressure(
            tempC)

    print(hydrostaticPressureChg)

    pumpPressure_pascals = 15000
    mass_flowrate_kg_per_s = 0.18
    ctahPressureChg = rust_get_ctah_branch_pressure_change(
            mass_flowrate_kg_per_s,
            tempC,
            0.0)


    ctahBranchPressureLoss_Pa = -ctahPressureChg + hydrostaticPressureChg
    print("ctah branch pressure loss (Pa): ", 
            ctahBranchPressureLoss_Pa)

    heaterPressureChg = rust_get_heater_branch_pressure_change(
            mass_rate_kg_per_s=-mass_flowrate_kg_per_s,
            temperature_degrees_c=tempC)

    heaterBranchPressureLoss_Pa = -heaterPressureChg + hydrostaticPressureChg
    print("heater branch pressure loss (Pa): ", 
            -heaterBranchPressureLoss_Pa)

    totalPressureLossPa = -heaterBranchPressureLoss_Pa + ctahBranchPressureLoss_Pa
    print("total pressure loss Pa : ", totalPressureLossPa)

    totalHeadLoss_meters_approx = totalPressureLossPa/9.81/1070
    print("\n approx total head loss (m): ", totalHeadLoss_meters_approx)


    end = time.time()

    elapsed = end - start

    def root(x):
        # the expression is x^2 + 2x + 1
        # roots are at x = -1
        square_term = np.power(x,2.0) 
        linear_term = np.multiply(2.0,x)

        total = np.add(square_term, linear_term)
        total = np.add(total,1.0)

        return total
    x_root = scipy.optimize.brentq(root,-1.0, 2.0)
    
    print("root: ", x_root)
    
    heater_branch_mass_flowrate_kg_per_s = get_heater_branch_mass_flowrate(
            hydrostaticPressureChg - 500.0,
            tempC)

    print("heater branch mass flowrate kg per s: ", 
            heater_branch_mass_flowrate_kg_per_s)

    print("\n time taken: (ms)", elapsed*1000)










if __name__ == '__main__':

    logging.basicConfig(level=logging.DEBUG)

    asyncio.run(main(), debug=True)
