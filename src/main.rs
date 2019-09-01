#![deny(unsafe_code)]
// We don't have the time or space for the standard library.
#![no_main]
#![no_std]

// Halt on panic
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt; // panic handler

use cortex_m;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{prelude::*, stm32};

// main() is not your usual main() - this function should never return,
// there are no arguments, and it runs straight after the RAM is initialized.
#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LED. On the Nucleo-446RE it's connected to pin PA5.
        let gpioa = dp.GPIOA.split();
        let mut led = gpioa.pa5.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

        loop {
            // On for 1s, off for 1s.
            led.set_high();
            delay.delay_ms(1000_u32);
            led.set_low();
            delay.delay_ms(1000_u32);
        }
    }

    loop {}
}
