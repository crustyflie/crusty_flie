# crustyflie
This projects goal is to run the Crazyflie 2.1 from Bitcraze on the programming language Rust. The project main goal is to learn Rust and Cargo. It will do so using the cargo build system and trying to integrate nicly with the Rust eco system.

Using https://github.com/timbod7/rust-stm32f4-examples/blob/master/blinky-hal as reference

# How to debug on Ubuntu 18.10
The debug settings are borrowed from: https://github.com/rust-embedded/cortex-m-quickstart

In a terminal run the following command:
```console
openocd -f openocd.cfg
```

This should have a output similar to this:
```console
Open On-Chip Debugger 0.10.0
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
adapter speed: 2000 kHz
adapter_nsrst_delay: 100
none separate
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : clock speed 1800 kHz
Info : STLINK v2 JTAG v29 API v2 SWIM v7 VID 0x0483 PID 0x3748
Info : using stlink api v2
Info : Target voltage: 2.923522
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : accepting 'gdb' connection on tcp/3333
Info : device id = 0x10076413
Info : flash size = 1024kbytes
```
In another termianal run:
```console
cargo run
```
This will automatically load the binary and start debugging at main
