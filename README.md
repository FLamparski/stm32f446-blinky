# a stm32f446 blinky in rust

It blinks an LED and is also debuggable with OpenOCD.

## Compiling

You'll need to add the `thumbv7em-none-eabi` target for this:

```
$ rustup component add rust-std-thumbv7em-none-eabi
```

Then run:

```
$ cargo build --target thumbv7em-none-eabi
```

## Running

Similar to instructions in [here](https://rust-embedded.github.io/discovery/05-led-roulette/flash-it.html). To start OpenOCD, use `openocd -f interface/stlink.cfg -f target/stm32f4x.cfg`.

## Memory layout

There's a file, `memory.x`, which defines the Flash and RAM addresses of the target MCU.
It's currently configured for 128K RAM and 512K Flash. It's a linker script fragment.

There's also a file, `.cargo/config`, which contains some magic commands and switches -
as far as I know, it basically tells the compiler to put together a linker script which also
includes `memory.x`.

## Literature

This example was put together from several sources:

* [Discovery - Rust Embedded - Ch.5 LED Roulette](https://rust-embedded.github.io/discovery/05-led-roulette/index.html)
* [Various](https://github.com/stm32-rs/stm32f407g-disc/blob/master/examples/gpio_hal_blinky.rs) [blinky](https://github.com/stm32-rs/stm32l4xx-hal/blob/master/examples/blinky.rs) [examples](https://github.com/stm32-rs/stm32f1xx-hal/blob/master/examples/blinky.rs) from the [stm32-rs project](https://github.com/stm32-rs)
* [This blinky](https://github.com/japaric/stm32f103xx-hal/blob/master/examples/delay.rs) from @japaric
* [This blog post describing how to flash rust firmware with OpenOCD](https://medium.com/coinmonks/coding-the-stm32-blue-pill-with-rust-and-visual-studio-code-b21615d8a20)

## Visual Studio Code `launch.json`

`gdbpath` should point to your copy of `arm-none-eabi-gdb`.

**TODO:** Configure the preLaunchTask to run `cargo build --target thumbv7em-none-eabi`, and
reset the MCU after `load`.

```json
{
    // Use IntelliSense to learn about possible attributes.
    // Hover to view descriptions of existing attributes.
    // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387
    "version": "0.2.0",
    "configurations": [
        {
            "type": "gdb",
            "request": "launch",
            "name": "Debug Microcontroller",
            "preLaunchTask": "",
            "target": "${workspaceFolder}\\target\\thumbv7em-none-eabi\\debug\\stm32f446-blinky",
            "cwd": "${workspaceRoot}",
            "gdbpath": "D:\\GNU Tools ARM Embedded\\8 2019-q3-update\\bin\\arm-none-eabi-gdb.exe",
            "autorun": [
                "target remote :3333",
                "load"
            ]
        }
    ]
}
```
