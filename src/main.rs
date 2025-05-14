// Compile without standard library
#![no_std]
#![no_main]

// Import HAL crates
use rp_pico::entry;

// HAL traits
// use embedded_hal::digital::OutputPin; // depreciated due to updated version to utilie embedded-hal = "0.2.7" for mod solution of Delay
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
use ryu;
use crate::utils::round_to_decimal;   // modularize some no-std math out of main

// custom adapted dht20 driver import
use crate::dht::Dht20;

use cortex_m::delay::Delay;

use liquidcrystal_i2c_rs::{Backlight, Display, Lcd};
static  LCD_ADDRESS: u8 = 0x27;

use panic_halt as _;

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

    // let mut lcd = Lcd::new(&mut i2clcd, LCD_ADDRESS, &mut sensor.delay()).unwrap();
    // let mut lcd = Lcd::new(&mut i2clcd, LCD_ADDRESS, &mut sensor.delay()).unwrap();
    // let mut lcd = Lcd::new(&mut i2clcd, LCD_ADDRESS, sensor.delay()).unwrap();

    // lcd.set_display(Display::On).unwrap();
    // lcd.set_backlight(Backlight::On).unwrap();

    // lcd.clear().unwrap();
    // lcd.print("Hello World!").unwrap();

    // Set up the board and get all components via our struct
    // let mut components = board::BoardComponents::setup_board();

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

    // init floating point pass-through variable to copy reading.hum before translating to string to print to LCD
    let mut the_hum: f32 = 101.0;

    // Buffer is required by ryu to transform a float into a string.
    let mut buffer = ryu::Buffer::new();
    // Allows customized rounding. Humidity sensor precision is 6 digits.
    let rounding: u32 = 1;

    // To prevent a return from main()
    loop {
        // match components.sensor.read() {
        //     Ok(reading) => {
        //         components.led_array.update(reading);
        //     }
        //     Err(_e) => {
        //         components.led_pin_led.set_high().unwrap();
        //         // error!("Error reading sensor: {e:?}");
        //     }
        // }
        // Dht20 sensor crate class now has a delay function appended to it
        // components.sensor.delay_ms(10000); // sleep 10 seconds between readings

        match sensor.read() {
            Ok(reading) => {
                the_hum = reading.hum;
                led_array.update(reading);
            }
            Err(_e) => {
                led_pin_led.set_high().unwrap();
                // error!("Error reading sensor: {e:?}");
            }
        }
        // lcd.print("Hello World!").unwrap();

        // sensor.delay_ms(10000); // sleep 10 seconds between readings

        // // reset LEDs to off
        // // components.led_array.clear();
        // led_array.clear();

        // todo: use our components here via the `components` struct
        let mut lcd = Lcd::new(&mut i2clcd, LCD_ADDRESS, sensor.delay()).unwrap();

        lcd.set_display(Display::On).unwrap();
        lcd.set_backlight(Backlight::On).unwrap();

        lcd.clear().unwrap();
        // let hum_string = the_hum.to_string();
        // let hum_string = format!("{}", the_hum);
        // lcd.print("{}",the_hum).unwrap();
        // lcd.print(hum_string).unwrap();
        lcd.print("Current Humidity").unwrap();

        //  Humidity reading placement (col, row): on lower row, centered (for 1
        //    decimal place precision)
        lcd.set_cursor_position(5, 1).unwrap();
        lcd.print(buffer.format(round_to_decimal(the_hum, rounding))).unwrap();
        lcd.print(" %").unwrap();

        sensor.delay_ms(10000); // sleep 10 seconds between readings

        // reset LEDs to off
        // components.led_array.clear();
        led_array.clear();
    }
}
// end of file
