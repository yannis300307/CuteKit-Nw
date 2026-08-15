# CuteKit (Nw)

A rendering toolkit for extremelly limited hardware featuring a 2D drawing abstraction layer, a layout system for GUIs and a fully textured 3D renderer. Made for the Numworks calculator.

The 3D renderer is based on my Numcraft project but has been highly reworked.

## DISCLAIMER

### My engine is not production ready yet! Please wait for this notice to be removed before using the engine in your games as it will be updated very often with breaking changes. Moreover, some features are not usable yet.

### The code also contains temporary testing code and is not usable as a lib.

## Setup the build environment

### For Debian based Linux distros

You can use the installer. Run `bash ./setup.sh` and the installer should install everything for you. (Not widely tested. Use it at your own risk.)

### Other distros and Macos

To build this app, you will need to install an embedded ARM rust compiler, the [Arm GCC compiler](https://developer.arm.com/downloads/-/gnu-rm) as well as [Node.js](https://nodejs.org/en/). 
The SDK for Epsilon apps is shipped as a npm module called [nwlink](https://www.npmjs.com/package/nwlink) that will automatically be installed at compile time.
**Make sure that `arm-none-eabi-gcc`is in your path.**

For more explanations on how to install the c sdk, follow [this guide](https://www.numworks.com/engineering/software/build/).

You might need to create a Python venv in the `epsilon_simulator` folder to install the pip packages on certain Linux distros. 

Then, you can set up the dependencies as follows :
```shell
brew install rustup node # Or equivalent on your OS
rustup-init
rustup target add thumbv7em-none-eabihf
cargo install just # Similar to makefile
```

## Build the app
```shell
just build
```

## Build and send the app to an actual calculator

Connect the calculator to your computer and run
```shell
just send-epsilon
```
for Epsilon or
```shell
just send-upsilon
```
for Upsilon.

## Run the app on the simulator

```shell
just sim
```
The simulator inputs will be automatically remapped for a better experience.

Use `w`, `s`, `a` and `d` to move the player, `shift` and `space` to go up and down, arrows to turn the camera, `return` to place a block or select in a menu and `backspace` to break a block or to go back in a menu.

You can speed up the simulator build by setting the job number.
```shell
just sim 5
```

### AI usage
AI was only used for debugging purpose. Nearly 100% of the code is handwritten.

## Legal info
NumWorks is a registered trademark.


Thanks to [Daemo](https://github.com/DaemonicGh) for helping me finding a name for the project.
