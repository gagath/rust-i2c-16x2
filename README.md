# 16x2 I2C screen library for Rust

Still WIP, inspired from [a Python snippet from Denis Pleic](https://gist.github.com/DenisFromHR/cc863375a6e19dce359d).
Datasheet used for reference [can be found here](https://www.sparkfun.com/datasheets/LCD/ADM1602K-NSW-FBS-3.3v.pdf).

## Description

This library aims at controlling 16x2 cheap screens using I2C from Linux. It
primary target is ARM devices such as RaspberryPi or FriendlyARM's NanoPi Neo.
It should nonetheless work on other Linux distributions with access to an I2C
bus.

## Building for Raspberry Pi

First setup your Rust cross compilation using the
[rust-cross](https://github.com/japaric/rust-cross) guide.

If you are using Archlinux like me you want to install
[arm-linux-gnueabihf-gcc](https://aur.archlinux.org/packages/arm-linux-gnueabihf-gcc/)
from AUR.

Then you should be good with the following commands

    $ cargo build --target=arm-unknown-linux-gnueabihf
    $ scp target/arm-unknown-linux-gnueabihf/debug/i2c-16x2 pi@raspberrypi.local:screen
    $ ssh pi@raspberrypi.local
    pi@raspberry$ ./screen

## License

Released under Apache 2.0.
