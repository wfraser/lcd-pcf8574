#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use i2cdev::core::I2CDevice;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};
use lcd::{Delay, Hardware};
use std::thread;
use std::time::Duration;

/// Represents an LCD display attached via PCF8574 I2C expander. Use the traits in the [`lcd`]
/// crate to interact with it.
pub struct Pcf8574 {
    dev: LinuxI2CDevice,
    data: u8,
    on_err: ErrorHandling,
}

/// What to do on I/O errors.
pub enum ErrorHandling {
    /// Ignore write errors.
    None,

    /// Panic on write errors.
    Panic,

    /// Run a custom handler function.
    Custom(Box<dyn FnMut(LinuxI2CError) + 'static>),
}

impl Pcf8574 {
    /// Create a new instance, using the Linux I2C interface for communication. `bus` is the number
    /// of `/dev/i2c-<bus>` to use, and `address` is the I2C address of the device.
    ///
    /// After opening the device, defaults to ignoring all I/O errors; see [`Self::on_error`] and
    /// [`ErrorHandling`] for how to change this behavior.
    pub fn new(bus: u8, address: u16) -> Result<Self, LinuxI2CError> {
        Ok(Self {
            dev: LinuxI2CDevice::new(format!("/dev/i2c-{}", bus), address)?,
            data: 0b0000_1000, // backlight on by default
            on_err: ErrorHandling::None,
        })
    }

    /// Change the I/O error handling strategy.
    ///
    /// [`lcd::Hardware`] doesn't have any way to return errors to the caller, so error handling
    /// has to be done here, internally.
    pub fn on_error(&mut self, on_err: ErrorHandling) {
        self.on_err = on_err;
    }

    /// Set the display's backlight on or off.
    pub fn backlight(&mut self, on: bool) {
        self.set_bit(3, on);
        self.apply();
    }

    fn set_bit(&mut self, offset: u8, bit: bool) {
        if bit {
            self.data |= 1 << offset;
        } else {
            self.data &= !(1 << offset);
        }
    }
}

impl Hardware for Pcf8574 {
    fn rs(&mut self, bit: bool) {
        self.set_bit(0, bit);
    }

    fn enable(&mut self, bit: bool) {
        self.set_bit(2, bit);
    }

    fn data(&mut self, bits: u8) {
        assert!(bits & 0xF0 == 0, "4-bit mode is required");
        self.data = (self.data & 0x0F) | (bits << 4);
    }

    fn apply(&mut self) {
        if let Err(e) = self.dev.smbus_write_byte(self.data) {
            match &mut self.on_err {
                ErrorHandling::None => (),
                ErrorHandling::Panic => panic!("smbus_write_byte failed: {}", e),
                ErrorHandling::Custom(f) => f(e),
            }
        }
    }
}

impl Delay for Pcf8574 {
    fn delay_us(&mut self, delay_usec: u32) {
        thread::sleep(Duration::from_micros(u64::from(delay_usec)));
    }
}
