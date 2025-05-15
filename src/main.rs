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
mod dht; // note: the original crate for Dht20 has been replicated (forked) locally to modify for parallel calls to Delay. This import (src/dht.rs) is our custom variation
mod leds;
mod utils;

use rp_pico::hal;
use rp_pico::hal::pac;
use rp_pico::hal::prelude::*;

// i2c elements
use rp_pico::hal::fugit::RateExtU32;

// ryu formats a float as a string, as required by the lcd
use crate::utils::round_to_decimal;
use ryu; // modularize some no-std math out of main

// custom adapted dht20 driver import
use crate::dht::Dht20;

use cortex_m::delay::Delay;

use liquidcrystal_i2c_rs::{Backlight, Display, Lcd};
static LCD_ADDRESS: u8 = 0x27;

use panic_halt as _;

// Main entry point
#[entry]
fn main() -> ! {
    let components = BoardComponents::setup_board();

    // init floating point pass-through variable to copy reading.hum before translating to string to print to LCD
    let mut the_hum: f32 = 101.0;

    // Buffer is required by ryu to transform a float into a string.
    let mut buffer = ryu::Buffer::new();
    // Allows customized rounding. Humidity sensor precision is 6 digits.
    let rounding: u32 = 1;

    // To prevent a return from main()
    loop {
        the_hum = match components.sensor.read(&mut delay) {
            Ok(reading) => {
                let hum = reading.hum;
                components.led_array.update(&reading);
                hum
            }
            Err(_e) => {
                let _ = components.led_pin_led.set_high();
                101.0 // Return the default value on error
            }
        };

        // todo: use our components here via the `components` struct
        let mut lcd = Lcd::new(&mut i2clcd, LCD_ADDRESS, &mut delay).unwrap();

        components.lcd.set_display(Display::On).unwrap();
        components.lcd.set_backlight(Backlight::On).unwrap();

        components.lcd.clear().unwrap();

        components.lcd.print("Current Humidity").unwrap();

        components.lcd.set_cursor_position(5, 1).unwrap();
        components.lcd.print(buffer.format(round_to_decimal(the_hum, rounding)))
            .unwrap();
        components.lcd.print(" %").unwrap();

        delay.delay_ms(10000); // sleep 10 seconds between readings

        components.led_array.clear();
    }
}
// end of file
