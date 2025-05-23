// Compile without standard library
#![no_std]
#![no_main]

use cortex_m::delay::Delay;

// Import HAL crates
use rp_pico::entry;
use rp_pico::hal;
use rp_pico::hal::gpio::Pin;
use rp_pico::hal::pac;

// HAL traits
use embedded_hal::digital::v2::OutputPin;

use OSU_RPMH::dht::Reading;

use panic_halt as _;

use rp_pico::hal::Clock;
use OSU_RPMH::leds;

// A pared down version of BoardComponents for this test executable
struct BoardComponents {
    // A struct containing all indicator LEDs and methods to control their behavior
    led_array: leds::LedArray,

    led_pin_led: Pin<hal::gpio::bank0::Gpio25, hal::gpio::FunctionSioOutput, hal::gpio::PullDown>,
    delay: Delay,
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

    let delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(peripherals.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        peripherals.IO_BANK0,
        peripherals.PADS_BANK0,
        sio.gpio_bank0,
        &mut peripherals.RESETS,
    );

    let led_pin_led = pins.led.into_push_pull_output();

    // Initialize an led array with five led pins
    let led_array = leds::LedArray::new(
        pins.gpio12,
        pins.gpio13,
        pins.gpio14,
        pins.gpio15,
        pins.gpio16,
    );

    // Return all components in the form of the struct (LCD will need to be added here as well)
    BoardComponents {
        led_pin_led,
        led_array,
        delay,
    }
}

/*
    To run the test program, use the command $cargo run --bin led_test

    TODO: Move the above documentation into a more obvious place (possibly the README)
*/
#[entry]
fn main() -> ! {
    // Fake readings to test the LED array
    let readings = [
        Reading {
            temp: 0.0,
            hum: 10.0,
        },
        Reading {
            temp: 0.0,
            hum: 30.0,
        },
        Reading {
            temp: 0.0,
            hum: 50.0,
        },
        Reading {
            temp: 0.0,
            hum: 70.0,
        },
        Reading {
            temp: 0.0,
            hum: 90.0,
        },
    ];

    // Set up the board and get all components via our struct
    let mut components = setup_board();

    let mut i = 0;

    loop {
        components.led_pin_led.set_high().unwrap();
        components.delay.delay_ms(500);
        components.led_pin_led.set_low().unwrap();

        components.led_array.clear();
        components.delay.delay_ms(500);

        if i < readings.len() {
            components.led_array.update(&readings[i].hum);
            i += 1;
        } else {
            i = 0;
        }

        components.delay.delay_ms(500);
    }
}
