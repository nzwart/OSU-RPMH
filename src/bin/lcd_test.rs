// Compile without standard library
#![no_std]
#![no_main]

use embedded_hal::digital::v2::OutputPin;
use rp_pico::entry;
use OSU_RPMH::{board, pico, shared_delay, utils::round_to_decimal};

use liquidcrystal_i2c_rs::{Backlight, Display};

use embedded_hal::blocking::delay::DelayMs;

#[entry]
fn main() -> ! {
    let mut rpp_core = pico::CoreComponents::setup_board();

    let mut delays = shared_delay::Delays::new(&rpp_core.shared_timer);

    let mut components = board::BoardComponents::setup_board(
        &rpp_core.shared_timer, 
        rpp_core.sensor_i2c, 
        &mut rpp_core.i2clcd,
        &mut delays.lcd_delay,
        rpp_core.led_pin_led, 
        rpp_core.led_array
    );

    let mut buffer = ryu::Buffer::new();
    let rounding: u32 = 1;

    loop {
        delays.generic_delay.delay_ms(500);
        
        components.led_pin_led.set_high().unwrap();

        components.lcd.set_display(Display::On).unwrap();
        components.lcd.set_backlight(Backlight::On).unwrap();
        components.lcd.clear().unwrap();
        components.lcd.print("Testing").unwrap();

        components.lcd.set_cursor_position(5, 1).unwrap();
        components.lcd.print(buffer.format(round_to_decimal(55.32, rounding))).unwrap();
        components.lcd.print(" %").unwrap();

        delays.generic_delay.delay_ms(10000);

        components.led_pin_led.set_low().unwrap();

        delays.generic_delay.delay_ms(1000);
    }
}
