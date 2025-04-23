// Compile without standard library
#![no_std]
#![no_main]

// Import HAL crates
use rp_pico::entry;
use rp_pico::hal;
use rp_pico::hal::pac;
use rp_pico::hal::prelude::*;

// HAL traits
use embedded_hal::digital::OutputPin;

// i2c elements
use rp_pico::hal::fugit::RateExtU32;
use rp_pico::hal::gpio::{FunctionI2C, Pin};

// dht20 driver
use cortex_m::delay::Delay;
use dht20::Dht20;

use panic_halt as _;

// Main entry point
#[entry]
fn main() -> ! {
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

    // The delay object lets us wait for specified amounts of time (in
    // milliseconds)
    let delay: Delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(peripherals.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        peripherals.IO_BANK0,
        peripherals.PADS_BANK0,
        sio.gpio_bank0,
        &mut peripherals.RESETS,
    );

    //**************** The below code can be deleted if not needed. ************
    // Set the LED to be an output
    let mut led_pin_led = pins.led.into_push_pull_output();
    // Set the GPIO 14, 15, 16 (Pico pins 19, 20, 21) to be an output
    let mut led_pin_yellow = pins.gpio14.into_push_pull_output();
    let mut led_pin_red = pins.gpio15.into_push_pull_output();
    let mut led_pin_green = pins.gpio16.into_push_pull_output();
    let mut led_pin_yellow2 = pins.gpio13.into_push_pull_output();
    let mut led_pin_red2 = pins.gpio12.into_push_pull_output();
    //**************** The above code can be deleted if not needed. ************

    // Configure two pins as being I²C, not GPIO
    let sda_pin: Pin<_, FunctionI2C, _> = pins.gpio18.reconfigure();
    let scl_pin: Pin<_, FunctionI2C, _> = pins.gpio19.reconfigure();

    // Initiate dht20 sensor
    let mut sensor = Dht20::new(
        hal::I2C::i2c1(
            peripherals.I2C1,
            sda_pin,
            scl_pin, // Try `not_an_scl_pin` here
            400.kHz(),
            &mut peripherals.RESETS,
            &clocks.system_clock,
        ),
        0x38,
        delay,
    );

    // sensor.read will produce two f32 values: reading.hum and reading.temp
    match sensor.read() {
        Ok(reading) => {
            // Do something with the reading.hum value
            if (reading.hum) > 0.0 {
                led_pin_red.set_high().unwrap();
            }
            if (reading.hum) > 20.0 {
                led_pin_yellow.set_high().unwrap();
            }
            if (reading.hum) > 40.0 {
                led_pin_green.set_high().unwrap();
            }
            if (reading.hum) > 60.0 {
                led_pin_yellow2.set_high().unwrap();
            }
            if (reading.hum) > 80.0 {
                led_pin_red2.set_high().unwrap();
            }
        }
        Err(_e) => {
            led_pin_led.set_high().unwrap();
            // error!("Error reading sensor: {e:?}");
        }

        
    }

    // To prevent a return from main()
    loop {}
}
