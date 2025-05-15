/*
cite: source dht20 v0.1.0 crate for rust embedded DHT20 sensor
This module forks the dht20 crate to add support for multiple uses of RPP core functions, like Delay
This file modifies the fork of the published crate, less the extra feature code for non-pertinent embedded-hal version
!!!    additions by Malcolm are only lines 100 t0 103. and updating the struct and impl to borrow a mutable delay 
URL: https://github.com/MnlPhlp/dht20
URL: https://crates.io/crates/dht20
*/

use core::fmt;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::i2c::{Read, Write, WriteRead};

// use embedded_hal::delay::DelayNs;
// use embedded_hal::i2c::{I2c, SevenBitAddress};

use log::info;

use panic_halt as _; // addition from OSU-PRMH repo solution

#[allow(dead_code)] // note: remove this line if we ever use the temp variable.
#[derive(Debug, Clone)]
pub struct Reading {
    pub temp: f32,
    pub hum: f32,
}

#[allow(dead_code)] // note: remove this line if we ever use the Error enum
#[derive(Debug)]
pub enum Error<E: fmt::Debug> {
    I2cError(E),
    ReadTooFast,
}
// updated Dht20 with mutable borrow of DELAY
pub struct Dht20<I2C, E>
where
    I2C: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
    E: fmt::Debug,
{
    i2c: I2C,
    address: u8,
    // delay: &'a mut DELAY,
}
// updated Dht20 with mutable borrow of DELAY
impl<I2C, E> Dht20<I2C, E>
where
    I2C: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
    E: fmt::Debug,
{
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self {
            i2c,
            address,
        }
    }

    pub fn read<DELAY>(&mut self, delay: &mut DELAY) -> Result<Reading, E>
    where DELAY: DelayMs<u16> {
        self.reset()?;
        // request reading
        self.write_data(&[0xAC, 0x33, 0])?;
        delay.delay_ms(80);
        // read data
        let data = self.read_data()?;
        // convert values
        let mut raw = (data[1] as u32) << 8;
        raw += data[2] as u32;
        raw <<= 4;
        raw += (data[3] >> 4) as u32;
        let hum = raw as f32 * 9.5367431640625e-5; // ==> / 1048576.0 * 100%;

        let mut raw = (data[3] & 0x0F) as u32;
        raw <<= 8;
        raw += data[4] as u32;
        raw <<= 8;
        raw += data[5] as u32;
        let temp = raw as f32 * 1.9073486328125e-4 - 50.0; //  ==> / 1048576.0 * 200 - 50;
        Ok(Reading { temp, hum })
    }

    fn reset(&mut self) -> Result<(), E> {
        let status = self.read_status()?;
        if status & 0x18 != 0x18 {
            info!("resetting");
            self.write_data(&[0x1B, 0, 0])?;
            self.write_data(&[0x1C, 0, 0])?;
            self.write_data(&[0x1E, 0, 0])?;
        }
        Ok(())
    }

    fn read_data(&mut self) -> Result<[u8; 8], E> {
        let mut buffer = [0; 8];
        self.i2c.read(self.address, &mut buffer)?;
        Ok(buffer)
    }

    fn read_status(&mut self) -> Result<u8, E> {
        let mut buffer = [0; 1];
        self.i2c.read(self.address, &mut buffer)?;
        Ok(buffer[0])
    }

    fn write_data(&mut self, data: &[u8]) -> Result<(), E> {
        self.i2c.write(self.address, data)
    }
}
