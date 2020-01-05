//! Uses the StatefulOutputPin embedded_hal trait to toggle the pin
//! On the stm32 discovery board this is the "south" led
//! Target board: STM32F3DISCOVERY

#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_halt;

mod leds;

use cortex_m_rt::entry;
use stm32f3xx_hal::prelude::*;
use stm32f3xx_hal::stm32;
use crate::leds::Leds;
use stm32f3xx_hal::delay::Delay;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    //dp.RCC.ahbenr.modify(|r, w| w.paen().set_bit().pben().variant(BLAH::Variant1));
    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut leds = Leds::new(gpioe);
    let mut delay = Delay::new(cp.SYST, clocks);

    loop {

        for i in (0..4)
            {
                leds[i].on();
                leds[i+4].on();
                delay.delay_ms(500u32);
                leds[i].off();
                leds[i+4].off();
                delay.delay_ms(500u32);
            }
    }
}