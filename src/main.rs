#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt; // panic handler

use stm32f4xx_hal as hal;
use cortex_m;
use cortex_m_rt::entry;

use crate::hal::{prelude::*, stm32};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (stm32::Peripherals::take(), cortex_m::peripheral::Peripherals::take()) {
        // Set up the LED
        let gpioa = dp.GPIOA.split();
        let mut led = gpioa.pa5.into_push_pull_output();

        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

        loop {
            led.set_high();
            delay.delay_ms(1000_u32);
            led.set_low();
            delay.delay_ms(1000_u32);
        }
    }

    loop {}
}
