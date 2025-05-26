use crate::shared_delay::{DelayTimer, SharedTimer};
use crate::leds;

use rp_pico::hal;
use rp_pico::hal::pac;

// i2c elements
use rp_pico::hal::gpio::{FunctionI2C, Pin};

// custom adapted dht20 driver import
use dht20::Dht20;

use liquidcrystal_i2c_rs::{Lcd};
static LCD_ADDRESS: u8 = 0x27;

// Abstract the components we'll be using on the board into their own struct
// This is useful for passing around the components in a single "object"
// This struct can be expanded to include other components as needed (i.e. our LCD)
pub struct BoardComponents<'a> {
    // DHT-20 humidity sensor
    pub sensor: Dht20<
        hal::I2C<
            pac::I2C1,
            (
                Pin<hal::gpio::bank0::Gpio18, FunctionI2C, hal::gpio::PullUp>,
                Pin<hal::gpio::bank0::Gpio19, FunctionI2C, hal::gpio::PullUp>,
            ),
        >,
        DelayTimer<'a>, 
    >,

    // LED Outputs
    // note: we're using PullDown to match what into_push_pull_output()
    //   returns, as we need to explicitly specify all generic type
    //   parameters in Rust struct definitions. The into_push_pull_output()
    //   function configures pins with PullDown by default, so by using
    //   the same type here, we ensure compatibility between our struct
    //   definition and the initialization code in setup_board()
    // On board LED
    pub led_pin_led:
        Pin<hal::gpio::bank0::Gpio25, hal::gpio::FunctionSioOutput, hal::gpio::PullDown>,

    // A struct containing all indicator LEDs and methods to control their behavior
    pub led_array: leds::LedArray,
    
    // 1602 LCD visual display
    pub lcd: Lcd<'a, 
        hal::I2C<
            pac::I2C0,
            (
                Pin<hal::gpio::bank0::Gpio0, FunctionI2C, hal::gpio::PullUp>,
                Pin<hal::gpio::bank0::Gpio1, FunctionI2C, hal::gpio::PullUp>,
            ),
        >,
        DelayTimer<'a>,
    >,
}

impl<'a> BoardComponents<'a> {
    // Set up all of our board components and return them in a single struct
    pub fn setup_board(shared_timer: &'a SharedTimer, 
        sensor_i2c: hal::I2C<
            pac::I2C1,
            (
                Pin<hal::gpio::bank0::Gpio18, FunctionI2C, hal::gpio::PullUp>,
                Pin<hal::gpio::bank0::Gpio19, FunctionI2C, hal::gpio::PullUp>,
            ),
        >,
        lcd_i2c: &'a mut hal::I2C<
            pac::I2C0, 
            (
                Pin<hal::gpio::bank0::Gpio0, FunctionI2C, hal::gpio::PullUp>, 
                Pin<hal::gpio::bank0::Gpio1, FunctionI2C, hal::gpio::PullUp>
            ),
        >,
        lcd_delay: &'a mut DelayTimer<'a>,
        led_pin_led: Pin<hal::gpio::bank0::Gpio25, hal::gpio::FunctionSioOutput, hal::gpio::PullDown>,
        led_array: leds::LedArray,
    ) -> BoardComponents<'a> {
        // Create sensor delay
        let sensor_delay = DelayTimer::new(&shared_timer);

        // Set up DHT20 sensor
        let sensor = Dht20::new(sensor_i2c, 0x38, sensor_delay);

        // Set up LCD 
        let lcd = Lcd::new(lcd_i2c, LCD_ADDRESS, lcd_delay).unwrap();

        // Return all components in the form of the struct (LCD will need to be added here as well)
        BoardComponents {
            sensor,
            lcd,
            led_pin_led,
            led_array,
        }
    }
}
