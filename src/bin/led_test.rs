// Compile without standard library
#![no_std]
#![no_main]

use rp_pico::entry;
use embedded_hal::digital::v2::OutputPin;
use OSU_RPMH::{dht::Reading, pico, shared_delay};
use panic_halt as _;
use OSU_RPMH::board;

use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;

/*
    Tests the functionality of 5 light LED array. If operating normally, the LEDs
    will blink in sequence, starting with just one LED, then two, three, four, and 
    finally all five LEDs. This is meant to simulate the different levels of 
    humidity that can be read by the sensor.
    
    To run the test program, use the command $cargo run --bin led_test
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

    let mut rpp_core = pico::CoreComponents::setup_board();

    let mut delays = shared_delay::Delays::new(&rpp_core.shared_timer);

    // Set up the board and get all components via our struct
    let mut components = board::BoardComponents::setup_board(
        &rpp_core.shared_timer,
        rpp_core.sensor_i2c, 
        &mut rpp_core.i2clcd,
        &mut delays.lcd_delay,
        rpp_core.led_pin_led, 
        rpp_core.led_array
    );

    let mut i = 0;

    loop {
        components.led_pin_led.set_high().unwrap();
        delays.generic_delay.delay_ms(500);
        components.led_pin_led.set_low().unwrap();

        components.led_array.clear();
        delays.generic_delay.delay_ms(500);

        if i < readings.len() {
            components.led_array.update(&readings[i].hum);
            i += 1;
        } else {
            i = 0;
        }

        delays.generic_delay.delay_ms(500);
    }
}
