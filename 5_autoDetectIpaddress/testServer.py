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
    _logger.info('Starting server!')
    
    # server loop
    async with server:
        while True:
            await asyncio.sleep(1)
            new_val = await myvar.get_value()  + rustAdd4(0.1)
            _logger.info('Set value of %s to %.1f', myvar, new_val)
            await myvar.write_value(new_val)



if __name__ == '__main__':

    logging.basicConfig(level=logging.DEBUG)

    asyncio.run(main(), debug=True)
