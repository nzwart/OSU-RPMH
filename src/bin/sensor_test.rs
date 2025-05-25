// Compile without standard library
#![no_std]
#![no_main]

use rp_pico::entry;
use embedded_hal::digital::v2::OutputPin;
use OSU_RPMH::{board::BoardComponents, pico};
use panic_halt as _;
use OSU_RPMH::board;

// Helper function: blinks the on-board LED n times
fn blink(components: &mut BoardComponents, n: u32) {
    for _ in 0..n {
        components.led_pin_led.set_high().unwrap();
        components.sensor.delay_ms(500);
        components.led_pin_led.set_low().unwrap();
        components.sensor.delay_ms(500);
    }
}

/*
    Tests the functionality of the DHT20 humidity sensor. If operating normally, the LED indicator on
    the Raspberry Pi Pico chip will blink once and then rest for 15s. Repeated blinking or a solid light
    for 15s indicates incorrect component state.
    
    To run the test program, use the command $cargo run --sensor_test
*/
#[entry]
fn main() -> ! {

    let mut rpp_core = pico::CoreComponents::setup_board();

    // Set up the board and get all components via our struct
    let mut components = board::BoardComponents::setup_board(&mut rpp_core.delay, rpp_core.i2c, rpp_core.led_pin_led, rpp_core.led_array);

    loop {
        components.led_pin_led.set_low().unwrap();

        match components.sensor.read() {
            // "Good" reading: blink once
            Ok(reading) if reading.hum > 10.0 && reading.hum < 90.0 => blink(&mut components, 1),
            // Unusually low reading: blink twice
            Ok(reading) if reading.hum < 10.0 => blink(&mut components, 2),
            // Unusually high reading: blink 3 times
            Ok(reading) if reading.hum > 90.0 => blink(&mut components, 3),
            // Reading that is outside normal range but did not generate an error: blink 4 times
            Ok(_) => blink(&mut components, 4),
            // Error reading: solid LED indicator
            Err(_) => components.led_pin_led.set_high().unwrap(),
        }

        // Wait 15s before next reading
        components.sensor.delay_ms(15000);
    }
}
