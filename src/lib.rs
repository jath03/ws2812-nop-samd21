#![no_std]

use cortex_m::asm::delay;
use embedded_hal::digital::v2::OutputPin;
use core::marker::PhantomData;
use smart_leds_trait::{RGB8, RGBW, SmartLedsWrite};

pub mod devices {
    pub struct Ws2812;
    pub struct Sk6812w;
}

pub struct Ws2812<P: OutputPin, DEVICE = devices::Ws2812> {
    pin: P,
    device: PhantomData<DEVICE>,
}

impl<P: OutputPin> Ws2812<P> {
    pub fn new(pin: P) -> Self {
        Self {
            pin: pin,
            device: PhantomData {},
        }
    }
}

impl<P: OutputPin> Ws2812<P, devices::Sk6812w> {
    pub fn new_sk6812w(pin: P) -> Self {
        Self {
            pin: pin,
            device: PhantomData {},
        }
    }
}

impl<P: OutputPin, D> Ws2812<P, D> {
    fn write_byte(&mut self, data: u8) {
        let mut bitmask: u8 = 0x80;
        while bitmask != 0 {
            self.pin.set_high().unwrap_or(());
            delay(5);
            if data & bitmask != 0 {
                delay(7);
                self.pin.set_low().unwrap_or(());
            } else {
                self.pin.set_low().unwrap_or(());
                delay(2);
            }
            bitmask >>= 1;
        }
        delay(2);
    }
}

impl<P> SmartLedsWrite for Ws2812<P>
where
    P: OutputPin,
{
    type Color = RGB8;
    type Error = ();
    fn write<T, I>(&mut self, iterator: T) -> Result<(), ()>
    where
        T: Iterator<Item = I>,
        I: Into<Self::Color>
    {
        for item in iterator {
            let color: Self::Color = item.into();
            self.write_byte(color.g);
            self.write_byte(color.r);
            self.write_byte(color.b);
        }
        Ok(())
    }
}

impl<P> SmartLedsWrite for Ws2812<P, devices::Sk6812w>
where
    P: OutputPin,
{
    type Color = RGBW<u8, u8>;
    type Error = ();
    fn write<T, I>(&mut self, iterator: T) -> Result<(), ()>
    where
        T: Iterator<Item = I>,
        I: Into<Self::Color>
    {
        for item in iterator {
            let color: Self::Color = item.into();
            self.write_byte(color.g);
            self.write_byte(color.r);
            self.write_byte(color.b);
            self.write_byte(color.a.0);
        }
        Ok(())
    }
}
