#![no_std]

use embedded_hal::i2c::I2c;
use embedded_hal_bus::i2c::AtomicDevice;
use embedded_hal_bus::util::AtomicCell;
use error::Error;
use pin::interrupt::InterruptController;
use pin::{Pin, Pins};

pub mod error;
pub mod pin;

pub(crate) mod registers;

/// A driver representing a single Microchip MCP23017.
///
/// Generic over an I2C bus `S` and device address `A`.
pub struct Mcp23017<S: I2c, const A: u8> {
    cell: AtomicCell<S>,
}

impl<S: I2c, const A: u8> Mcp23017<S, A> {
    /// Construct a new driver for a device accessible over the bus.
    pub fn new(i2c: S) -> Self {
        Self {
            cell: AtomicCell::new(i2c),
        }
    }

    /// Extract individually controllable pins and an interrupt controller from the device.
    ///
    /// Pins A7 and B7 are pre-configured as outputs, as mandated by the
    /// datasheet.
    ///
    /// Errors if communication with the device fails.
    pub fn split(&mut self) -> Result<(Pins<S, A>, InterruptController<S, A>), Error<S>> {
        unsafe {
            Ok((
                Pins {
                    a0: Pin::new(AtomicDevice::new(&self.cell)),
                    a1: Pin::new(AtomicDevice::new(&self.cell)),
                    a2: Pin::new(AtomicDevice::new(&self.cell)),
                    a3: Pin::new(AtomicDevice::new(&self.cell)),
                    a4: Pin::new(AtomicDevice::new(&self.cell)),
                    a5: Pin::new(AtomicDevice::new(&self.cell)),
                    a6: Pin::new(AtomicDevice::new(&self.cell)),
                    a7: Pin::new(AtomicDevice::new(&self.cell)).try_into()?,

                    b0: Pin::new(AtomicDevice::new(&self.cell)),
                    b1: Pin::new(AtomicDevice::new(&self.cell)),
                    b2: Pin::new(AtomicDevice::new(&self.cell)),
                    b3: Pin::new(AtomicDevice::new(&self.cell)),
                    b4: Pin::new(AtomicDevice::new(&self.cell)),
                    b5: Pin::new(AtomicDevice::new(&self.cell)),
                    b6: Pin::new(AtomicDevice::new(&self.cell)),
                    b7: Pin::new(AtomicDevice::new(&self.cell)).try_into()?,
                },
                InterruptController::new(AtomicDevice::new(&self.cell)),
            ))
        }
    }
}
