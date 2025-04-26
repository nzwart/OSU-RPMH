use rp_pico::hal::gpio::{self, bank0::{Gpio12, Gpio13, Gpio14, Gpio15, Gpio16}, DefaultTypeState, Pin};

// A struct for interacting with the LedArray 
pub struct LedArray {
    led_pin_yellow:     Pin<Gpio14, gpio::FunctionSioOutput, gpio::PullDown>,
    led_pin_red:        Pin<Gpio15, gpio::FunctionSioOutput, gpio::PullDown>,
    led_pin_green:      Pin<Gpio16, gpio::FunctionSioOutput, gpio::PullDown>,
    led_pin_yellow2:    Pin<Gpio13, gpio::FunctionSioOutput, gpio::PullDown>,
    led_pin_red2:       Pin<Gpio12, gpio::FunctionSioOutput, gpio::PullDown>,
}

impl LedArray {
    // Creates a new LedArray using the five pins passed as arguments
    pub fn new(
        gpio12: Pin<Gpio12, <Gpio12 as DefaultTypeState>::Function, <Gpio12 as DefaultTypeState>::PullType>,
        gpio13: Pin<Gpio13, <Gpio13 as DefaultTypeState>::Function, <Gpio13 as DefaultTypeState>::PullType >,
        gpio14: Pin<Gpio14, <Gpio14 as DefaultTypeState>::Function, <Gpio14 as DefaultTypeState>::PullType>,
        gpio15: Pin<Gpio15, <Gpio15 as DefaultTypeState>::Function, <Gpio15 as DefaultTypeState>::PullType>,
        gpio16: Pin<Gpio16, <Gpio16 as DefaultTypeState>::Function, <Gpio16 as DefaultTypeState>::PullType>,
    ) -> Self {
        LedArray {
            led_pin_yellow: gpio14.into_push_pull_output(),
            led_pin_red: gpio15.into_push_pull_output(),
            led_pin_green: gpio16.into_push_pull_output(),
            led_pin_yellow2: gpio13.into_push_pull_output(),
            led_pin_red2: gpio12.into_push_pull_output(),
        }
    }
}
