// Compile without standard library
#![no_std]
#![no_main]

use embedded_hal::digital::v2::OutputPin;
use rp_pico::entry;
use OSU_RPMH::{board, pico, shared_delay};

use liquidcrystal_i2c_rs::{Backlight, Display, Lcd};

use embedded_hal::blocking::delay::DelayMs;

fn print_test_message_to_lcd<I, D>(
    the_lcd: &mut Lcd<I, D>,
    component: &str,
    message: &str,
) -> Result<(), <I as embedded_hal::blocking::i2c::Write>::Error>
where 
    I: embedded_hal::blocking::i2c::Write,
    D: embedded_hal::blocking::delay::DelayMs<u8>,
{
    the_lcd.set_display(Display::On)?;
    the_lcd.set_backlight(Backlight::On)?;
    the_lcd.clear()?;
    the_lcd.print("Test: ")?;
    the_lcd.print(component)?;

    the_lcd.set_cursor_position(0, 1)?;
    the_lcd.print(message)?;

    Ok(())
}

/*
    Tests the functionality of 1602 LCD display. If operating normally, the LCD will
    display the message "Testing" on the first line and "55.3 %" on the second line. 
    This output demonstrates that the LCD can display arbitrary data and that
    it is rounding data correctly.
    
    To run the test program, use the command $cargo run --bin lcd_test
*/
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

    loop {
        delays.generic_delay.delay_ms(500);
        components.led_pin_led.set_high().unwrap();

        // Test LCD
        print_test_message_to_lcd(&mut components.lcd, "LCD", "Working").unwrap();
        delays.generic_delay.delay_ms(5000);

        // Test DHT20 humidity sensor
        match components.sensor.read() {
            Ok(_) => print_test_message_to_lcd(&mut components.lcd, "DHT20", "Working"),
            _ => print_test_message_to_lcd(&mut components.lcd, "DHT20", "Error"),
        }.unwrap();
        delays.generic_delay.delay_ms(5000);

        // Test LED array
        components.led_array.update(&100.0);
        print_test_message_to_lcd(&mut components.lcd, "LED Array", "Wrk if 5 leds on").unwrap();
        delays.generic_delay.delay_ms(5000);
        components.led_array.clear();

        components.led_pin_led.set_low().unwrap();

        delays.generic_delay.delay_ms(1000);
    }
}
