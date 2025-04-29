// Compile without standard library
#![no_std]
#![no_main]

// Import HAL crates
use rp_pico::entry;

// HAL traits
use embedded_hal::digital::OutputPin;

use panic_halt as _;

mod board;


// Main entry point
#[entry]
fn main() -> ! {
    // Set up the board and get all components via our struct
    let mut components = board::BoardComponents::setup_board();

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

    // To prevent a return from main()
    loop {
        // todo: use our components here via the `components` struct
    }
}
