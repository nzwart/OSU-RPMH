// Compile without standard library
#![no_std]
#![no_main]

// Import HAL crates
use rp_pico::entry;

// HAL traits
// use embedded_hal::digital::OutputPin; // depreciated due to updated version to utilie embedded-hal = "0.2.7" for mod solution of Delay
use embedded_hal::digital::v2::OutputPin;

mod board; // partial redemption for LCD proof-of-concept to enable repeat borrow of delay in main loop
mod dht; // note: the original crate for Dht20 has been replicated (forked) locally to modify for parallel calls to Delay. This import (src/dht.rs) is our custom variation
mod leds;
mod pico;
mod utils;

use liquidcrystal_i2c_rs::{Backlight, Display, Lcd};
static  LCD_ADDRESS: u8 = 0x27;

// ryu formats a float as a string, as required by the lcd
use ryu;

use panic_halt as _;
use utils::round_to_decimal;

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
    // Allows customized rounding.  Humidity sensor reading precision is 6 digits
    let rounding: u32 = 1;

    // To prevent a return from main()
    loop {
        // sensor.read will produce two f32 values: reading.hum and reading.temp
        // parse the sensor reading
        match components.sensor.read() {
            Ok(reading) => {
                the_hum = reading.hum;
                components.led_array.update(reading);
            }
            Err(_e) => {
                components.led_pin_led.set_high().unwrap();
                // error!("Error reading sensor: {e:?}");
            }
        }

        // use our components here via the `components` struct
        let mut lcd = Lcd::new(&mut rpp_core.i2clcd, LCD_ADDRESS, components.sensor.delay()).unwrap();

        lcd.set_display(Display::On).unwrap();
        lcd.set_backlight(Backlight::On).unwrap();

        lcd.clear().unwrap();
    
        lcd.print("Current Humidity").unwrap();

        // Humidity reading placement (col, row): on lower row, centered (for 1
        //   decimal place precision)
        lcd.set_cursor_position(5, 1).unwrap();
        lcd.print(buffer.format(round_to_decimal(the_hum, rounding))).unwrap();
        lcd.print(" %").unwrap();

        // Dht20 sensor crate class now has a delay function appended to it
        components.sensor.delay_ms(10000); // sleep 10 seconds between readings

        // reset LEDs to off
        components.led_array.clear();
    }
}
// end of file
