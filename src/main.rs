// Compile without standard library
#![no_std]
#![no_main]

// Import HAL crates
use rp_pico::entry;

// HAL traits
// use embedded_hal::digital::OutputPin; // depreciated due to updated version to utilie embedded-hal = "0.2.7" for mod solution of Delay
use embedded_hal::digital::v2::OutputPin;

// i2c elements
use rp_pico::hal::fugit::RateExtU32;
use rp_pico::hal::gpio::{FunctionI2C, Pin};

// dht20 driver
use cortex_m::delay::Delay;
// use dht20::Dht20;
mod board;
mod dht; // note: the original crate for Dht20 has been replicated (forked) locally to modify for parallel calls to Delay. This import (src/dht.rs) is our custom variation

use panic_halt as _;

// Main entry point
#[entry]
fn main() -> ! {
    // Set up the board and get all components via our struct
    let mut components = board::BoardComponents::setup_board();

    // main event loop v0 beta for solution demonstration
    // To prevent a return from main()
    loop {
        // use our components here via the `components` struct
        // sensor.read will produce two f32 values: reading.hum and reading.temp
        match components.sensor.read() {
            Ok(reading) => {
                if reading.hum > 0.0 {
                    components.led_pin_red.set_high().unwrap();
                }
                if reading.hum > 20.0 {
                    components.led_pin_yellow.set_high().unwrap();
                }
                if reading.hum > 40.0 {
                    components.led_pin_green.set_high().unwrap();
                }
                if reading.hum > 60.0 {
                    components.led_pin_yellow2.set_high().unwrap();
                }
                if reading.hum > 80.0 {
                    components.led_pin_red2.set_high().unwrap();
                }
            }
            Err(_e) => {
                components.led_pin_led.set_high().unwrap();
                // error!("Error reading sensor: {e:?}");
            }
        }
        // Dht20 sensor crate class now has a delay function appended to it
        components.sensor.delay_ms(10000); // sleep 10 seconds between readings
                                           // reset LEDs to off
        components.led_pin_red.set_low().unwrap();
        components.led_pin_yellow.set_low().unwrap();
        components.led_pin_green.set_low().unwrap();
        components.led_pin_yellow2.set_low().unwrap();
        components.led_pin_red2.set_low().unwrap();
    }
}
// end of file
