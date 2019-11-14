// IMPORTANT the standard `main` interface is not used because it requires nightly
#![no_main]
#![no_std]
#![feature(lang_items)]
extern crate cortex_m_rt;

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// extern crate crazy_flie as board;
extern crate embedded_hal as hal;
use crazy_flie as board;

use cortex_m_rt::entry;
use cortex_m::peripheral::Peripherals;

use crate::board::{
    hal::stm32,
    hal::{delay::Delay, prelude::*},
    led::{LedName, Leds},
};

// use `main` as the entry point of this application
// `main` is not allowed to return
#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        let gpioc = p.GPIOC.split();

        // Configure LED outputs
        let mut all_leds = Leds::new(gpioc);

        // Constrain clock registers
        let rcc = p.RCC.constrain();

        // Configure clock to 168 MHz (i.e. the maximum) and freeze it
        let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();

        // Get delay provider
        let mut delay = Delay::new(cp.SYST, clocks);

        loop {
            // Turn LED on
            all_leds[LedName::left_red].on();

            // Delay twice for half a second due to limited timer resolution
            delay.delay_ms(500_u16);
            delay.delay_ms(500_u16);

            // Turn LED off
            all_leds[LedName::left_red].off();

            // Delay twice for half a second due to limited timer resolution
            delay.delay_ms(500_u16);
            delay.delay_ms(500_u16);
        }
    }

    loop {}
}
