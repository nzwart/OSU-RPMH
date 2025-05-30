// Compile without standard library
#![no_std]
#![no_main]
#![allow(unused)]

use core::fmt;

// HAL traits
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::i2c::{Read, Write, WriteRead};
use embedded_hal::digital::v2::OutputPin;

use rp_pico::entry;
use rp_pico::hal;
use rp_pico::hal::pac;
use rp_pico::hal::prelude::*;


use cortex_m::delay;

// i2c elements
use rp_pico::hal::fugit::RateExtU32;

// ryu formats a float as a string, as required by the lcd
use OSU_RPMH::utils::round_to_decimal;
use ryu; // modularize some no-std math out of main

// custom adapted dht20 driver import
use dht20::Dht20;

// LCD imports
use liquidcrystal_i2c_rs::{Backlight, Display, Lcd};
static LCD_ADDRESS: u8 = 0x27;

use panic_halt as _;

use OSU_RPMH::shared_delay::{self, DelayTimer};
use OSU_RPMH::pico;
use OSU_RPMH::board;

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

// Helper function for displaying miscellaneous data to the LCD
// (useful for debugging purposes)
fn print_message_to_lcd<I, D>(
    the_lcd: &mut Lcd<I, D>,
    message: &str,
) -> Result<(), <I as embedded_hal::blocking::i2c::Write>::Error>
where 
    I: embedded_hal::blocking::i2c::Write,
    D: embedded_hal::blocking::delay::DelayMs<u8>,
{
    the_lcd.set_display(Display::On)?;
    the_lcd.set_backlight(Backlight::On)?;

    the_lcd.clear()?;

    the_lcd.print(message)?;

    Ok(())
}

// Main entry point
#[entry]
fn main() -> ! {
    // This is the Pico-specific setup, now abstracted, away from component init for mutable borrow
    let mut rpp_core = pico::CoreComponents::setup_board();

    let mut delays = shared_delay::Delays::new(&rpp_core.shared_timer);
    
    // Set up the board and get all components via our struct; partial fix to redeem board components mod struct
    let mut components = board::BoardComponents::setup_board(
        &rpp_core.shared_timer, 
        rpp_core.sensor_i2c, 
        &mut rpp_core.i2clcd, 
        &mut delays.lcd_delay,
        rpp_core.led_pin_led, 
        rpp_core.led_array,
    );
 
    // init floating point pass-through variable to copy reading.hum before translating to string to print to LCD
    let mut the_hum: f32 = 101.0;

    // Buffer is required by ryu to transform a float into a string
    let mut buffer = ryu::Buffer::new();
    // Allows customized rounding. Humidity sensor precision is 6 digits.
    let rounding: u32 = 1;

    // To prevent a return from main()
    loop {
        delays.generic_delay.delay_ms(500);

        // sensor.read will produce two f32 values: reading.hum and reading.temp
        // parse the sensor reading
        the_hum = read_sensor(&mut components.sensor, &mut components.led_pin_led);
        delays.generic_delay.delay_ms(500);

        // Set the LED array to indicate the humidity level
        components.led_array.update(&the_hum);
        delays.generic_delay.delay_ms(500);

        // Print the humidity to the LCD
        if let Err(_) = print_humidity_to_lcd(&mut components.lcd, the_hum, &mut buffer, rounding) {
            delays.generic_delay.delay_ms(500);
            
            // If there is an error printing to the LCD, turn on the onboard LED
            let _ = components.led_pin_led.set_high();
            delays.generic_delay.delay_ms(500);
        }

        // Dht20 sensor crate class now has a delay function appended to it
        delays.generic_delay.delay_ms(10000); // sleep 10 seconds between readings

        // reset LEDs to off
        components.led_array.clear();
        delays.generic_delay.delay_ms(500);
    }
}
// end of file
