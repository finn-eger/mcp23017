use core::marker::PhantomData;

use embedded_hal::i2c::I2c;

use crate::pin::{Bank, PinId};

/// A pin's control registers.
pub(crate) struct Registers<I: PinId, S: I2c, const A: u8> {
    id: PhantomData<I>,
    i2c: S,
}

impl<I: PinId, S: I2c, const A: u8> Registers<I, S, A> {
    pub(crate) unsafe fn new(i2c: S) -> Self {
        Self {
            id: PhantomData,
            i2c,
        }
    }

    /// I/O Direction Register
    pub(crate) const IODIR: u8 = 0x00;
    /// Input Polarity Register
    #[expect(unused)]
    pub(crate) const IOPOL: u8 = 0x02;
    /// Interrupt-on-change Control Register
    pub(crate) const GPINTEN: u8 = 0x04;
    /// Default Compare Register for Interrupt-on-change
    pub(crate) const DEFVAL: u8 = 0x06;
    /// Interrupt Control Register
    pub(crate) const INTCON: u8 = 0x08;
    /// Configuration Register
    #[expect(unused)]
    pub(crate) const IOCON: u8 = 0x0A;
    /// Pull-up Resistor Configuration Register
    pub(crate) const GPPU: u8 = 0x0C;
    /// Interrupt Flag Register
    pub(crate) const INTF: u8 = 0x0E;
    /// Interrupt Captured Register
    pub(crate) const INTCAP: u8 = 0x10;
    /// Port Register
    pub(crate) const GPIO: u8 = 0x12;
    /// Output Latch Register
    #[expect(unused)]
    pub(crate) const OLAT: u8 = 0x14;

    /// Read the pin's bit in a register.
    pub(crate) unsafe fn get(&mut self, register: u8) -> Result<bool, S::Error> {
        let mut read = [0x00];
        self.i2c
            .write_read(A, &[Self::address(register)], &mut read)?;
        Ok(read[0] & (1 << I::NUMBER) != 0)
    }

    /// Modify the pin's bit in a register.
    pub(crate) unsafe fn set(&mut self, register: u8, bit: bool) -> Result<(), S::Error> {
        let mut read = [0x00];
        self.i2c
            .write_read(A, &[Self::address(register)], &mut read)?;
        self.i2c.write(
            A,
            &[
                Self::address(register),
                if bit {
                    read[0] | (1 << I::NUMBER)
                } else {
                    read[0] & !(1 << I::NUMBER)
                },
            ],
        )
    }

    /// Shift a register base address to the pin's bank.
    const fn address(base: u8) -> u8 {
        base + match I::BANK {
            Bank::A => 0,
            Bank::B => 1,
        }
    }
}
