#![no_std]
#![no_main]

use cortex_m_rt::entry;
extern crate cortex_m;

extern crate panic_halt;

extern crate trinket_m0 as hal;
extern crate ws2812_nop_samd21 as ws2812;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::pac::{CorePeripherals, Peripherals};

use smart_leds::{brightness, SmartLedsWrite};
use smart_leds::hsv::{RGBW, White};

use ws2812::Ws2812;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let neopixel_pin = pins.d4.into_push_pull_output(&mut pins.port);
    let mut neopixel = Ws2812::new_sk6812w(neopixel_pin);

    const NUM_LEDS: usize = 150;
    let mut data = [RGBW::default(); NUM_LEDS];

    loop {
        for j in 0..(256 * 5) {
            for i in 0..NUM_LEDS {
                data[i] = wheel((((i * 256) as u16 / NUM_LEDS as u16 + j as u16) & 255) as u8);
            }
            cortex_m::interrupt::free(|_| {
                neopixel
                    .write(data.iter().cloned())
                    .unwrap();
            });
            delay.delay_ms(1u8);
        }
    }
}

/// Input a value 0 to 255 to get a color value
/// The colours are a transition r - g - b - back to r.
fn wheel(mut wheel_pos: u8) -> RGBW<u8> {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        return (255 - wheel_pos * 3, 0, wheel_pos * 3, White(0)).into();
    }
    if wheel_pos < 170 {
        wheel_pos -= 85;
        return (0, wheel_pos * 3, 255 - wheel_pos * 3, White(0)).into();
    }
    wheel_pos -= 170;
    (wheel_pos * 3, 255 - wheel_pos * 3, 0, White(0)).into()
}
