// Compile without standard library
#![no_std]
#![no_main]

// Import HAL crates
use rp_pico::entry;

// HAL traits
// use embedded_hal::digital::OutputPin; // depreciated due to updated version to utilie embedded-hal = "0.2.7" for mod solution of Delay
use embedded_hal::digital::v2::OutputPin;

mod board;
mod dht; // note: the original crate for Dht20 has been replicated (forked) locally to modify for parallel calls to Delay. This import (src/dht.rs) is our custom variation
mod leds;

use panic_halt as _;

// Main entry point
#[entry]
fn main() -> ! {
    // Set up the board and get all components via our struct
    let mut components = board::BoardComponents::setup_board();

    // sensor.read will produce two f32 values: reading.hum and reading.temp
    // match components.sensor.read() {
    //     Ok(reading) => {
    //         if reading.hum > 0.0 {
    //             components.led_pin_red.set_high().unwrap();
    //         }
    //         if reading.hum > 20.0 {
    //             components.led_pin_yellow.set_high().unwrap();
    //         }
    //         if reading.hum > 40.0 {
    //             components.led_pin_green.set_high().unwrap();
    //         }
    //         if reading.hum > 60.0 {
    //             components.led_pin_yellow2.set_high().unwrap();
    //         }
    //         if reading.hum > 80.0 {
    //             components.led_pin_red2.set_high().unwrap();
    //         }
    //     }
    //     Err(_e) => {
    //         components.led_pin_led.set_high().unwrap();
    //         // error!("Error reading sensor: {e:?}");
    //     }
    // }

    // To prevent a return from main()
    loop {
        match components.sensor.read() {
            Ok(reading) => {
                components.led_array.update(reading);
            }
            Err(_e) => {
                components.led_pin_led.set_high().unwrap();
                // error!("Error reading sensor: {e:?}");
            }
        }
        // Dht20 sensor crate class now has a delay function appended to it
        components.sensor.delay_ms(10000); // sleep 10 seconds between readings

        // reset LEDs to off
        components.led_array.clear();

        // todo: use our components here via the `components` struct
    }
}
// end of file
