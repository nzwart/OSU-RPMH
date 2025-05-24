// Compile without standard library
#![no_std]
#![no_main]
#![ allow(unused)]
// Import HAL crates
use rp_pico::entry;

// HAL traits
// use embedded_hal::digital::OutputPin; // depreciated due to updated version to utilie embedded-hal = "0.2.7" for mod solution of Delay
use core::fmt;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::i2c::{Read, Write, WriteRead};
use embedded_hal::digital::v2::OutputPin;
use core::fmt;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::i2c::{Read, Write, WriteRead};

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
mod board; // partial redemption for LCD proof-of-concept to enable repeat borrow of delay in main loop
mod dht; // note: the original crate for Dht20 has been replicated (forked) locally to modify for parallel calls to Delay. This import (src/dht.rs) is our custom variation
mod leds;
mod pico;
mod utils;
use crate::dht::Dht20;

use liquidcrystal_i2c_rs::{Backlight, Display, Lcd};
static LCD_ADDRESS: u8 = 0x27;

// ryu formats a float as a string, as required by the lcd
use ryu;

use panic_halt as _;
use utils::round_to_decimal;

// Great work Nic ~~
fn read_sensor<'a, I2C, DELAY, E>(
    sensor: &mut Dht20<'a, I2C, DELAY, E>,
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
// ~~ Thanks Nic!

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
    // This is the Pico-specific setup, now abstracted, away from component init for mutable borrow
    let mut rpp_core = pico::CoreComponents::setup_board();
    
    // Set up the board and get all components via our struct; partial fix to redeem board components mod struct
    let mut components = board::BoardComponents::setup_board(&mut rpp_core.delay, rpp_core.i2c, rpp_core.led_pin_led, rpp_core.led_array);

    // init floating point pass-through variable to copy reading.hum before translating to string to print to LCD
    let mut the_hum: f32 = 101.0;

    // Buffer is required by ryu to transform a float into a string
    let mut buffer = ryu::Buffer::new();
    // Allows customized rounding. Humidity sensor precision is 6 digits.
    let rounding: u32 = 1;

    // To prevent a return from main()
    loop {
        // sensor.read will produce two f32 values: reading.hum and reading.temp
        // parse the sensor reading
        the_hum = read_sensor(&mut components.sensor, &mut components.led_pin_led);
        // match components.sensor.read() {
        //     Ok(reading) => {
        //         the_hum = reading.hum;
        //         components.led_array.update(reading);
        //     }
        //     Err(_e) => {
        //         components.led_pin_led.set_high().unwrap();
        //         // error!("Error reading sensor: {e:?}");
        //     }
        // }
        // Set the LED array to indicate the humidity level
        components.led_array.update(&the_hum);

        // use our components here via the `components` struct
        let mut lcd = Lcd::new(&mut rpp_core.i2clcd, LCD_ADDRESS, components.sensor.delay()).unwrap();

        // Print the humidity to the LCD
        if let Err(_) = print_humidity_to_lcd(&mut lcd, the_hum, &mut buffer, rounding) {
            // If there is an error printing to the LCD, turn on the onboard LED
            let _ = components.led_pin_led.set_high();
        }
        // lcd.set_display(Display::On).unwrap();
        // lcd.set_backlight(Backlight::On).unwrap();

        // lcd.clear().unwrap();
    
        // lcd.print("Current Humidity").unwrap();

        // // Humidity reading placement (col, row): on lower row, centered (for 1
        // //   decimal place precision)
        // lcd.set_cursor_position(5, 1).unwrap();
        // lcd.print(buffer.format(round_to_decimal(the_hum, rounding))).unwrap();
        // lcd.print(" %").unwrap();

        // Dht20 sensor crate class now has a delay function appended to it
        components.sensor.delay_ms(10000); // sleep 10 seconds between readings

        // reset LEDs to off
        components.led_array.clear();
    }
}
// end of file
