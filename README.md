# 16x2 I2C screen library for Rust

Still WIP, inspired from [a Python snippet from Denis Pleic](https://gist.github.com/DenisFromHR/cc863375a6e19dce359d).
Datasheet used for reference [can be found here](https://www.sparkfun.com/datasheets/LCD/ADM1602K-NSW-FBS-3.3v.pdf).

## Building for Raspberry Pi

    $ cargo build --target=arm-unknown-linux-gnueabihf
    $ scp target/arm-unknown-linux-gnueabihf/debug/i2c-16x2 pi@raspberrypi.local:screen
    $ ssh pi@raspberrypi.local
    pi@raspberry$ ./screen

## License

Released under Apache 2.0.
