#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _; // panic handler

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::delay::Delay;
use crate::hal::pac;
use crate::hal::prelude::*;

const ONBOARD_LED_DELAY_MS: u32 = 1000; // Delay duration in milliseconds

#[entry]
fn main() -> ! {
    if let Some(dp) = pac::Peripherals::take() {
        // Constrain clocking registers
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        if let Some(cp) = cortex_m::peripheral::Peripherals::take() {
            let mut delay = Delay::new(cp.SYST, &clocks);

            // GPIOC for onboard led (PC13)
            let gpioa = dp.GPIOA.split();
            let mut onboard_led = gpioa.pa5.into_push_pull_output();

            loop {
                onboard_led.set_high(); // Turn onboard LED on
                delay.delay_ms(ONBOARD_LED_DELAY_MS);

                onboard_led.set_low(); // Turn onboard LED off
                delay.delay_ms(ONBOARD_LED_DELAY_MS);
            }
        }
    }

    loop {
        cortex_m::asm::nop(); // Loop indefinitely
    }
}