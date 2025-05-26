// Compile without standard library
#![no_std]
#![no_main]

use rp_pico::entry;
use embedded_hal::digital::v2::OutputPin;
use OSU_RPMH::{board::BoardComponents, pico, shared_delay::{self, DelayTimer}};
use panic_halt as _;
use OSU_RPMH::board;

use embedded_hal::blocking::delay::DelayMs;

// Helper function: blinks the on-board LED n times
fn blink(components: &mut BoardComponents, n: u32, delay: &mut DelayTimer) {
    for _ in 0..n {
        components.led_pin_led.set_high().unwrap();
        delay.delay_ms(500);

        components.led_pin_led.set_low().unwrap();
        delay.delay_ms(500);
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

    loop {
        components.led_pin_led.set_low().unwrap();

        match components.sensor.read() {
            // "Good" reading: blink once
            Ok(reading) if reading.hum > 10.0 && reading.hum < 90.0 => blink(&mut components, 1, &mut delays.generic_delay),
            // Unusually low reading: blink twice
            Ok(reading) if reading.hum < 10.0 => blink(&mut components, 2, &mut delays.generic_delay),
            // Unusually high reading: blink 3 times
            Ok(reading) if reading.hum > 90.0 => blink(&mut components, 3, &mut delays.generic_delay),
            // Reading that is outside normal range but did not generate an error: blink 4 times
            Ok(_) => blink(&mut components, 4, &mut delays.generic_delay),
            // Error reading: solid LED indicator
            Err(_) => components.led_pin_led.set_high().unwrap(),
        }

        // Wait 15s before next reading
        delays.generic_delay.delay_ms(15000);
    }
}
