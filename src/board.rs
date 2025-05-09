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

// use rppal::{gpio::Gpio, i2c::I2c};
// new library crate that works for the LCD; 
use liquidcrystal_i2c_rs::{Backlight, Display, Lcd};
static  LCD_ADDRESS: u8 = 0x27;

// ~~~ +++ ~~~ +++ // errata from trying

    // use crate::delay_wrapper::DelayWrapper;

    // use embedded_hal::blocking::delay::DelayNs;
// use eh1::delay::DelayNs;
    // use cortex_m::delay::DelayNs;
    // use embedded_hal::delay::DelayNs;

    // impl DelayNs<u32> for Delay {
    // impl eh1::delay::DelayNs for cortex_m::delay::Delay {
    //     fn delay_ns(&mut self, ns: u32) {
    //         // Convert nanoseconds to microseconds (1 microsecond = 1000 nanoseconds)
    //         let us = ns / 1000;
    //         self.delay_us(us);
    //     }
    // }
    // impl DelayNs<u32> for Delay {
    //     fn delay_ns(&mut self, ns: u32) {
    //         // Convert nanoseconds to microseconds (1 microsecond = 1000 nanoseconds)
    //         let us = ns / 1000;
    //         self.delay_us(us);
    //     }
    // }

// use rp_pico::hal::Timer;

// use lcd1602_driver::{
//     command::{DataWidth, MoveDirection, State},
//     lcd::{self, Anim, Basic, CGRAMGraph, Ext, ExtRead, FlipStyle, Lcd, MoveStyle},
//     sender::I2cSender,
//     utils::BitOps,
// };

    // use lcd1602_driver::{
    //     builder::{Builder, BuilderAPI},
    //     enums::{
    //         animation::{FlipStyle, MoveStyle},
    //         basic_command::{Font, LineMode, MoveDirection, ShiftType, State},
    //     },
    //     pins::{FourPinsAPI, Pins},
    //     utils::BitOps,
    //     LCDAnimation, LCDBasic, LCDExt,
    // };

    // use lcd1602_rs::LCD1602;

    // const HEART: [u8; 8] = [
    //     0b00000, 0b00000, 0b01010, 0b11111, 0b01110, 0b00100, 0b00000, 0b00000,
    // ];

// const HEART: CGRAMGraph = CGRAMGraph {
//     upper: [
//         0b00000, 0b00000, 0b01010, 0b11111, 0b01110, 0b00100, 0b00000, 0b00000,
//     ],
//     lower: Some([0b00100, 0b01110, 0b00100]),
// };

// ~~~ +++ ~~~ +++ 

// Abstract the components we'll be using on the board into their own struct
// This is useful for passing around the components in a single "object"
// This struct can be expanded to include other components as needed (i.e. our LCD)
    // updated to pass mutable borrowable delay, need to pass from main to possibly fix..
pub struct BoardComponents<'a> {
    // DHT-20 humidity sensor
    pub sensor: Dht20<'a,
        hal::I2C<
            pac::I2C1,
            (
                Pin<hal::gpio::bank0::Gpio18, FunctionI2C, hal::gpio::PullUp>,
                Pin<hal::gpio::bank0::Gpio19, FunctionI2C, hal::gpio::PullUp>,
            ),
        >,
        Delay, 
        // DelayWrapper<'a>,
        // &'a mut Delay, // Borrow DELAY instead of owning it
        hal::i2c::Error, // compiler requests explicit definition of third argument for I2C error
    >,

    // Delay object // errata
    // pub delay: Delay, // Added as a public field

    // pub delay: DelayWrapper<'a>, // Add DelayWrapper as a public field

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
// updated to pass mutable borrowable delay, would need to pass delay, pins, and both i2c objects into setup method to possibly fix... 
impl<'a> BoardComponents<'a> {
    // Set up all of our board components and return them in a single struct
    pub fn setup_board() -> BoardComponents<'a> {
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
        // update as mutable for borrow
        let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz()); // updated to compile the Dht mod solution by suhrmosu
        
        // let mut delay =  <dyn eh1::delay::DelayNs as DelayNs>::cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz()); 
        // let mut delay = core.SYST.delay(&clocks);
        // let mut delay = <dyn embedded_hal::delay::DelayNs>::new(core.SYST, clocks.system_clock.freq().to_Hz());
        // let mut delay = core.SYST.delay(&clocks);

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
        // let sensor = Dht20::new(i2c, 0x38, delay);
        let sensor = Dht20::new(i2c, 0x38, &mut delay); // borrow the delay as mutable 

        // todo: set up LCD if present on board (and after adding it to the struct)

        // Configure two pins as being I²C, not GPIO
        let sda_lcd_pin = pins.gpio0.reconfigure(); 
        let scl_lcd_pin = pins.gpio1.reconfigure(); 

        // init for LCD embedded hal I2C
        let mut i2clcd = hal::I2C::i2c0(
            peripherals.I2C0,
            sda_lcd_pin,
            scl_lcd_pin,
            100.kHz(),
            &mut peripherals.RESETS,
            &clocks.system_clock,
        );

        // errata from crate that does not mesh well with embedded hal versions 
        // let mut sender = I2cSender::new(&mut i2clcd, 0x27u8);

        // let lcd_config = lcd::Config::default().set_data_width(DataWidth::Bit4);

        // let mut delayer = sensor.delay_ms;

        // init LCD1602
        // let mut lcd = Lcd::new(&mut sender,  &mut delay, lcd_config, None);

        // draw a little heart in CGRAM
        // lcd.write_graph_to_cgram(0, &HEART);

        // let mut lcd = Lcd::new(&mut i2clcd, LCD_ADDRESS, &mut sensor.delay()).unwrap();
        // old attempt implement new library; 
        // let mut lcd = Lcd::new(&mut i2clcd, LCD_ADDRESS, &mut delay).unwrap();
        let mut lcd = Lcd::new(&mut i2clcd, LCD_ADDRESS, sensor.delay()).unwrap();
        // test LCD in place // to remove if Board components is revived to work again
        lcd.set_display(Display::On).unwrap();
        lcd.set_backlight(Backlight::On).unwrap();

        lcd.clear().unwrap();
        lcd.print("Hello World!").unwrap();

        // Return all components in the form of the struct (LCD will need to be added here as well)
        BoardComponents {
            sensor,
            // lcd, // need to implement in component type struct
            led_pin_led,
            led_array,
        }
    }
}
