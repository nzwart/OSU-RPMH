// Import HAL crates (only used on Pico)
// use rp_pico::hal::{self, pac};
// todo: importing any additional needed hal crates for the RPP and getting a blinker going on there)

// Main entry point
#[cfg(feature = "pico")]
fn main() {
    // Embedded Pico version of main -- only runs if we build for the Pico
    use rp_pico::hal::{pac, prelude::*}; // todo: move this up later

    // This is the Pico-specific setup
    let _peripherals = pac::Peripherals::take().unwrap();
    let _core = pac::CorePeripherals::take().unwrap();

    // TODO: our actual Pico code will go here
    // (LED blinking, sensor reading, etc.)
    loop {
        // Blinking LEDs and sensor feedback and whatnot
    }
}

// Desktop code (default)
#[cfg(feature = "desktop")]
fn main() {
    println!("Hello, Rust Humidity Sensor Team! This is the desktop test. If you see this message, that is a sign that your Rust installation is working.");
}
