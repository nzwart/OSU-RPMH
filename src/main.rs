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

mod leds;

// Abstract the components we'll be using on the board into their own struct
// This is useful for passing around the components in a single "object"
// This struct can be expanded to include other components as needed (i.e. our LCD)
struct BoardComponents {
    // DHT-20 humidity sensor
    sensor: Dht20<
        hal::I2C<
            pac::I2C1,
            (
                Pin<hal::gpio::bank0::Gpio18, FunctionI2C, hal::gpio::PullUp>,
                Pin<hal::gpio::bank0::Gpio19, FunctionI2C, hal::gpio::PullUp>,
            ),
        >,
        Delay,
    >,

    // LED Outputs
    // note: we're using PullDown to match what into_push_pull_output() returns, as we need to explicitly specify all generic type parameters in Rust struct definitions. The into_push_pull_output() function configures pins with PullDown by default, so by using the same type here, we ensure compatibility between our struct definition and the initialization code in setup_board()
    led_pin_led: Pin<hal::gpio::bank0::Gpio25, hal::gpio::FunctionSioOutput, hal::gpio::PullDown>, // note: this is the onboard LED, whereas the others in the following initializations are LEDs physically connected to GPIO pins

    // A struct containing all indicator LEDs and methods to control their behavior
    led_array: leds::LedArray,

    // todo: Other peripherals can be added below, such as an LCD
}

// Set up all of our board components and return them in a single struct
fn setup_board() -> BoardComponents {
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

    // Set the onboard RPP LED to be an output
    let led_pin_led = pins.led.into_push_pull_output();

    let led_array = leds::LedArray::new(pins.gpio12, pins.gpio13, pins.gpio14, pins.gpio15, pins.gpio16);

    // Configure two pins as being I²C, not GPIO
    let sda_pin = pins.gpio18.reconfigure();
    let scl_pin = pins.gpio19.reconfigure();

    // Set up DHT20 sensor
    let sensor = Dht20::new(
        hal::I2C::i2c1(
            peripherals.I2C1,
            sda_pin,
            scl_pin,
            400.kHz(),
            &mut peripherals.RESETS,
            &clocks.system_clock,
        ),
        0x38,
        delay,
    );

    // todo: set up LCD if present on board (and after adding it to the struct)

    // Return all components in the form of the struct (LCD will need to be added here as well)
    BoardComponents {
        sensor,
        led_pin_led,
        led_array,
    }
}

// Main entry point
#[entry]
fn main() -> ! {
    // Set up the board and get all components via our struct
    let mut components = setup_board();

    // sensor.read will produce two f32 values: reading.hum and reading.temp
    match components.sensor.read() {
        Ok(reading) => {
//             if reading.hum > 0.0 {
                // components.led_pin_red.set_high().unwrap();
            // }
            // if reading.hum > 20.0 {
                // components.led_pin_yellow.set_high().unwrap();
            // }
            // if reading.hum > 40.0 {
                // components.led_pin_green.set_high().unwrap();
            // }
            // if reading.hum > 60.0 {
                // components.led_pin_yellow2.set_high().unwrap();
            // }
            // if reading.hum > 80.0 {
                // components.led_pin_red2.set_high().unwrap();
            // }
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
