// Compile without standard library
#![no_std]
#![no_main]

// Import HAL crates
use rp_pico::entry;

// HAL traits
// use embedded_hal::digital::OutputPin; // depreciated due to updated version to utilie embedded-hal = "0.2.7" for mod solution of Delay
use core::fmt;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::i2c::{Read, Write, WriteRead};
use embedded_hal::digital::v2::OutputPin;

mod board;
// mod dht; // note: the original crate for Dht20 has been replicated (forked) locally to modify for parallel calls to Delay. This import (src/dht.rs) is our custom variation
mod leds;
mod utils;
mod delay;

use rp_pico::hal;
use rp_pico::hal::pac;
use rp_pico::hal::prelude::*;

// i2c elements
use rp_pico::hal::fugit::RateExtU32;

// ryu formats a float as a string, as required by the lcd
use crate::utils::round_to_decimal;
use ryu; // modularize some no-std math out of main

// custom adapted dht20 driver import
// use crate::dht::Dht20;
use dht20::Dht20;

use cortex_m::delay::Delay;

use liquidcrystal_i2c_rs::{Backlight, Display, Lcd};
static LCD_ADDRESS: u8 = 0x27;

use panic_halt as _;

fn read_sensor<'a, I2C, DELAY, E>(
    sensor: &mut Dht20<I2C, DELAY>,
    led_pin_led: &mut impl OutputPin,
) -> f32
where
    I2C: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
    DELAY: DelayMs<u16>,
    E: fmt::Debug,
{
    match sensor.read() {
        Ok(reading) => {
            let hum = reading.hum;
            hum
        }
        Err(_e) => {
            let _ = led_pin_led.set_high();
            101.0 // Return the default value on error
        }
    }
}

// note: generic parameter I implements the i2c::Write trait, and D implements the DelayMs<u8> trait
fn print_humidity_to_lcd<I, D>(
    the_lcd: &mut Lcd<I, D>,
    the_hum: f32,
    buffer: &mut ryu::Buffer,
    rounding: u32,
) -> Result<(), <I as embedded_hal::blocking::i2c::Write>::Error>
where
    I: embedded_hal::blocking::i2c::Write,
    D: embedded_hal::blocking::delay::DelayMs<u8>,
{
    the_lcd.set_display(Display::On)?;
    the_lcd.set_backlight(Backlight::On)?;

    the_lcd.clear()?;

    the_lcd.print("Current Humidity")?;

    the_lcd.set_cursor_position(5, 1)?;
    the_lcd.print(buffer.format(round_to_decimal(the_hum, rounding)))?;
    the_lcd.print(" %")?;

    Ok(())
}

// Main entry point
#[entry]
fn main() -> ! {
    let mut components = board::BoardComponents::setup_board();
                                                        // Buffer is required by ryu to transform a float into a string.
    let mut buffer = ryu::Buffer::new();
    // Allows customized rounding. Humidity sensor precision is 6 digits.
    let rounding: u32 = 1;

    // To prevent a return from main()
    loop {
        let the_hum = read_sensor(&mut components.sensor, &mut components.led_pin_led);

        // Set the LED array to indicate the humidity level
        components.led_array.update(&the_hum);

        // Print the humidity to the LCD
        if let Err(_) = print_humidity_to_lcd(&mut components.lcd, the_hum, &mut buffer, rounding) {
            // If there is an error printing to the LCD, turn on the onboard LED
            let _ = components.led_pin_led.set_high();
        }

        components.delay.delay_ms(10000 as u32); // sleep 10 seconds between readings

        components.led_array.clear();
    }
}
// end of file
