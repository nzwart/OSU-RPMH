use crate::board::hal::Clock;
use crate::leds;
use rp_pico::hal;
use rp_pico::hal::pac;

// i2c elements
use rp_pico::hal::fugit::RateExtU32;
use rp_pico::hal::gpio::{FunctionI2C, Pin};

// custom adapted dht20 driver import
use crate::dht::Dht20;

use cortex_m::delay::Delay;

// Abstract the components we'll be using on the board into their own struct
// This is useful for passing around the components in a single "object"
// This struct can be expanded to include other components as needed (i.e. our LCD)
pub struct BoardComponents {
    // DHT-20 humidity sensor
    pub sensor: Dht20<
        hal::I2C<
            pac::I2C1,
            (
                Pin<hal::gpio::bank0::Gpio18, FunctionI2C, hal::gpio::PullUp>,
                Pin<hal::gpio::bank0::Gpio19, FunctionI2C, hal::gpio::PullUp>,
            ),
        >,
        Delay,
        hal::i2c::Error, // compiler requests explicit definition of third argument for I2C error
    >,

    // LED Outputs
    // note: we're using PullDown to match what into_push_pull_output()
    //   returns, as we need to explicitly specify all generic type
    //   parameters in Rust struct definitions. The into_push_pull_output()
    //   function configures pins with PullDown by default, so by using
    //   the same type here, we ensure compatibility between our struct
    //   definition and the initialization code in setup_board()
    // note: the below lines were added manually from mjanderson's code during merge. todo: remove this comment line once merge is complete
    // On board LED
    pub led_pin_led:
        Pin<hal::gpio::bank0::Gpio25, hal::gpio::FunctionSioOutput, hal::gpio::PullDown>,

    // A struct containing all indicator LEDs and methods to control their behavior
    pub led_array: leds::LedArray,
    // note: END lines added manually from mjanderson's code during merge. todo: remove this comment line once merge is complete
    // todo: Other peripherals can be added below, such as an LCD
}

impl BoardComponents {
    // Set up all of our board components and return them in a single struct
    pub fn setup_board() -> BoardComponents {
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
        let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz()); // updated to compile the Dht mod solution // note: added manually during merge by zwartn. todo: remove NOTE, not line, once merge is complete

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
        // Initialize an led array with five led pins
        let led_array = leds::LedArray::new(
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
        let sensor = Dht20::new(i2c, 0x38, delay);

        // todo: set up LCD if present on board (and after adding it to the struct)

        // Return all components in the form of the struct (LCD will need to be added here as well)
        BoardComponents {
            sensor,
            led_pin_led,
            led_array,
        }
    }
}
