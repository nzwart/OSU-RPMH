// Compile without standard library
#![no_std]
#![no_main]
#![allow(non_snake_case)] // Allow our crate to have a non-snake-case name

pub mod board; // partial redemption after LCD proof-of-concept to enable repeat borrow of delay in main loop
pub mod dht;
pub mod leds;
pub mod pico;