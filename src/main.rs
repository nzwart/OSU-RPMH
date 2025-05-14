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

// mod board; // depreciated for LCD proof-of-concept to enable repeat borrow of delay in main loop
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

fn read_sensor<'a, I2C, DELAY, E>(
    sensor: &mut Dht20<'a, I2C, DELAY, E>,
    led_array: &mut leds::LedArray,
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
            led_array.update(&reading);
            hum
        }
        Err(_e) => {
            let _ = led_pin_led.set_high();
            101.0 // Return the default value on error
        }
    }
}

// Main entry point
#[entry]
fn main() -> ! {
    // local init from Board mod
    // This is the Pico-specific setup
    let mut peripherals = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(peripherals.WATCHDOG);

    // Configure the clocks
    // (The default is to generate a 125 MHz system clock)
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        peripherals.XOSC,
        peripherals.CLOCKS,
        peripherals.PLL_SYS,
        peripherals.PLL_USB,
        &mut peripherals.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The delay object lets us wait for specified amounts of time (in milliseconds)
    let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz()); // updated to compile the Dht mod solution by suhrmosu

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(peripherals.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        peripherals.IO_BANK0,
        peripherals.PADS_BANK0,
        sio.gpio_bank0,
        &mut peripherals.RESETS,
    );

    // Set the onboard RPP LED to be an output
    let mut led_pin_led = pins.led.into_push_pull_output();
    // Initialize an led array with five led pins
    let mut led_array = leds::LedArray::new(
        pins.gpio12,
        pins.gpio13,
        pins.gpio14,
        pins.gpio15,
        pins.gpio16,
    );

    // Configure two pins as being I²C, not GPIO
    let sda_pin = pins.gpio18.reconfigure();
    let scl_pin = pins.gpio19.reconfigure();

    // init for embedded hal I2C
    let i2c = hal::I2C::i2c1(
        peripherals.I2C1,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut peripherals.RESETS,
        &clocks.system_clock,
    );

    // Set up DHT20 sensor
    // let sensor = Dht20::new(i2c, 0x38, delay);
    let mut sensor = Dht20::new(i2c, 0x38, &mut delay); // mutable borrow of delay

    // Configure two pins as being I²C for LCD SDA/SCL
    let sda_lcd_pin = pins.gpio0.reconfigure();
    let scl_lcd_pin = pins.gpio1.reconfigure();

    let mut i2clcd = hal::I2C::i2c0(
        peripherals.I2C0,
        sda_lcd_pin,
        scl_lcd_pin,
        100.kHz(),
        &mut peripherals.RESETS,
        &clocks.system_clock,
    );

    // init floating point pass-through variable to copy reading.hum before translating to string to print to LCD
    let mut the_hum: f32 = 101.0;

    // Buffer is required by ryu to transform a float into a string.
    let mut buffer = ryu::Buffer::new();
    // Allows customized rounding. Humidity sensor precision is 6 digits.
    let rounding: u32 = 1;

    // To prevent a return from main()
    loop {
        the_hum = read_sensor(&mut sensor, &mut led_array, &mut led_pin_led);

        // todo: use our components here via the `components` struct
        let mut lcd = Lcd::new(&mut i2clcd, LCD_ADDRESS, sensor.delay()).unwrap();

        lcd.set_display(Display::On).unwrap();
        lcd.set_backlight(Backlight::On).unwrap();

        lcd.clear().unwrap();

        lcd.print("Current Humidity").unwrap();

        lcd.set_cursor_position(5, 1).unwrap();
        lcd.print(buffer.format(round_to_decimal(the_hum, rounding)))
            .unwrap();
        lcd.print(" %").unwrap();

        sensor.delay_ms(10000); // sleep 10 seconds between readings

        led_array.clear();
    }
}
// end of file
