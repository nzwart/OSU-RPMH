// Compile without standard library
#![no_std]
#![no_main]

// Import HAL crates
use rp_pico::entry;
use rp_pico::hal::prelude::*;
use rp_pico::hal::pac;
use rp_pico::hal;

// HAL traits
use embedded_hal::digital::OutputPin;

// ************* i2c code BEGIN ************************************************
use rp_pico::hal::fugit::RateExtU32;

// use embedded_hal::i2c::Operation::Write;
use rp_pico::hal::{
    gpio::{FunctionI2C, Pin},
};

use cortex_m::prelude::_embedded_hal_blocking_i2c_Write;
// ************* i2c code END **************************************************
// ************* dht20 code BEGIN **********************************************
use dht20::Dht20;
// use esp_hal::{delay::Delay, main};
// use cortex_m::delay::Delay;
// ************* dht20 code END ************************************************

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

    // The delay object lets us wait for specified amounts of time (in milliseconds)
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(peripherals.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        peripherals.IO_BANK0,
        peripherals.PADS_BANK0,
        sio.gpio_bank0,
        &mut peripherals.RESETS,
    );

    // Set the LED to be an output
    let mut led_pin = pins.led.into_push_pull_output();
    // Set the GPIO 14, 15, 16 (Pico pins 19, 20, 21) to be an output
    let mut led_pin_yellow = pins.gpio14.into_push_pull_output();
    let mut led_pin_red = pins.gpio15.into_push_pull_output();
    let mut led_pin_green = pins.gpio16.into_push_pull_output();
    let mut led_pin_err = pins.gpio22.into_push_pull_output();

    // ************* i2c code BEGIN ********************************************

    // Configure two pins as being I²C, not GPIO
    let sda_pin: Pin<_, FunctionI2C, _> = pins.gpio18.reconfigure();
    let scl_pin: Pin<_, FunctionI2C, _> = pins.gpio19.reconfigure();

    // Create the I²C drive, using the two pre-configured pins. This will fail
    // at compile time if the pins are in the wrong mode, or if this I²C
    // peripheral isn't available on these pins!
    let mut i2c = hal::I2C::i2c1(
        peripherals.I2C1,
        sda_pin,
        scl_pin, // Try `not_an_scl_pin` here
        400.kHz(),
        &mut peripherals.RESETS,
        &clocks.system_clock,
    );

    // Write three bytes to the I²C device with 7-bit address 0x2C
    i2c.write(0x2Cu8, &[1, 2, 3]).unwrap();

    // ************* i2c code END ********************************************
    // ************* dht20 code BEGIN ******************************************

    // Set up the DHT20 sensor
    // let mut delayy = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    // let mut dht20 = Dht20::new(i2c);
    // let mut delayy = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let core2 = pac::CorePeripherals::take().unwrap();
    let mut sensor = Dht20::new(
        i2c/*platform specific i2c driver*/,
        0x38,
        cortex_m::delay::Delay::new(core2.SYST, clocks.system_clock.freq().to_Hz())/*platform specific delay*/,
    );
    match sensor.read() {
        Ok(reading) => led_pin_green.set_high().unwrap(),
        // println!("Temp: {} °C, Hum: {} %",reading.temp, reading.hum),
        Err(e) => {
            led_pin_err.set_high().unwrap();
            // error!("Error reading sensor: {e:?}");
        }
    }
    // intitialize the sensor
    // if let Err(e) = dht20.init(&mut delay) {
    //   led_pin_err.set_high().unwrap();
    //   loop{}
    // }

    // ************* dht20 code END ********************************************

    // TODO: our actual Pico code will go here
    // (LED blinking, sensor reading, etc.)
    loop {
        // Blink the LEDs at 1 Hz
        led_pin.set_high().unwrap();
        delay.delay_ms(500);
        led_pin_green.set_high().unwrap();
        delay.delay_ms(500);
        led_pin_red.set_high().unwrap();
        delay.delay_ms(500);
        led_pin_yellow.set_high().unwrap();
        delay.delay_ms(500);

        // set all low
        led_pin.set_low().unwrap();
        led_pin_green.set_low().unwrap();
        led_pin_red.set_low().unwrap();
        led_pin_yellow.set_low().unwrap();
        delay.delay_ms(500);
        // delay.delay_ns(500);
    }
}

