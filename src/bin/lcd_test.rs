// Compile without standard library
#![no_std]
#![no_main]

use embedded_hal::digital::v2::OutputPin;
use rp_pico::entry;
use OSU_RPMH::{board, pico, utils::round_to_decimal};

use liquidcrystal_i2c_rs::{Backlight, Display, Lcd};
static LCD_ADDRESS: u8 = 0x27;

#[entry]
fn main() -> ! {
    let mut rpp_core = pico::CoreComponents::setup_board();

    let mut components = board::BoardComponents::setup_board(&mut rpp_core.delay, rpp_core.i2c, rpp_core.led_pin_led, rpp_core.led_array);

    let mut buffer = ryu::Buffer::new();
    let rounding: u32 = 1;

    loop {
        components.led_pin_led.set_high().unwrap();

        let mut lcd = Lcd::new(&mut rpp_core.i2clcd, LCD_ADDRESS, components.sensor.delay()).unwrap();

        lcd.set_display(Display::On).unwrap();
        lcd.set_backlight(Backlight::On).unwrap();
        lcd.clear().unwrap();
        lcd.print("Testing").unwrap();

        lcd.set_cursor_position(5, 1).unwrap();
        lcd.print(buffer.format(round_to_decimal(55.32, rounding))).unwrap();
        lcd.print(" %").unwrap();

        components.sensor.delay_ms(10000);

        components.led_pin_led.set_low().unwrap();

        components.sensor.delay_ms(1000);
    }
}
