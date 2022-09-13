# python with rust code in server

## Auto Detect Ip address

Normally in OPCUA you'd have to specify ip address either by using
ipconfig in windows or ip a in linux

however, python has a way to auto detect ip address

```python
import socket
# get ip address automatically
def getIPAddress():
    hostname = socket.gethostname()
    IPaddr = socket.gethostbyname(hostname)
    return IPaddr
```
This ip address can then be used to host the opcua server.

### virtual environment

```zsh
python3 -m venv .env
```

source the file:

```zsh
source .env/bin/activate
```
https://python.land/virtual-environments/virtualenv

the virtual environment changes the path variable to one in the virtual environment.

This makes it such that the packages installed are only installed in
the virtual environment.

to deactivate:
```zsh
deactivate
```

### Installation of Async OPCUA and maturin in virtual environment
```zsh
pip install asyncua
```

For rust bindings:

```zsh
pip install maturin
```

use pyO3...
credit to:
https://www.youtube.com/watch?v=DpUlfWP_gtg


### building rust code

Now once you have installed maturin, you can use maturin init
in order to create the template files and cargo toml files.

You can then develop the rust libraries accordingly.

```zsh
cargo watch
```

When developing code in rust, i use cargo watch to watch for compiler errors.

### building python bindings

```zsh
maturin develop
```

Once the rust code is done, we can then use maturin to develop a python module
we can import into python directly.

### importing rust code in python

```python
# importing rust functions in maturin
import rust_functions_in_python

# calling rust functions
def rustAdd4(value):
    return rust_functions_in_python.add_four_rust(value)

```

When the python function is defined, you can use it like any other
python function. 

### Upstream Repository

[Opcua Asyncio](https://github.com/FreeOpcUa/opcua-asyncio)


### python pyqt5 client

in Arch linux, i did this:
i used the AUR helper paru and pip
```zsh
paru -S pyqt5
pip3 install opcua-client numpy pyqtgraph
```

if you don't have an aur helper, you can use:

```zsh
sudo pacman -S pyqt5
pip3 install opcua-client numpy pyqtgraph
```

go to 
```zsh
cd /home/$USER/.local/bin
```
and find the opcua-client binary file
If you are in a virtual environment, the opcua GUI client should be on your path
anyway. So the above step isn't valid.
