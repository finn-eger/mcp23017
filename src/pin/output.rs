//! Configurations for using pins as outputs.

use core::marker::PhantomData;

use embedded_hal::digital::{OutputPin, StatefulOutputPin};
use embedded_hal::i2c::I2c;

use super::input::{Input, InputConfiguration};
use super::{Pin, PinId, PinMode};
use crate::error::Error;
use crate::registers::Registers;

/// Marker type for pins set as outputs.
pub struct Output;

impl PinMode for Output {}

impl<I: PinId, S: I2c, const A: u8> OutputPin for Pin<'_, I, Output, S, A> {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe { self.registers.set(Registers::<I, S, A>::GPIO, false)? };
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe { self.registers.set(Registers::<I, S, A>::GPIO, true)? };
        Ok(())
    }
}

impl<I: PinId, S: I2c, const A: u8> StatefulOutputPin for Pin<'_, I, Output, S, A> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(unsafe { self.registers.get(Registers::<I, S, A>::GPIO)? })
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        self.is_set_high().map(|x| !x)
    }
}

impl<'a, I: PinId, C: InputConfiguration, S: I2c, const A: u8> Pin<'a, I, Input<C>, S, A> {
    /// Reconfigure the pin as a push pull output.
    pub fn into_push_pull_output(mut self) -> Result<Pin<'a, I, Output, S, A>, Error<S>> {
        unsafe { self.registers.set(Registers::<I, S, A>::IODIR, false)? }
        Ok(Pin {
            id: PhantomData,
            mode: PhantomData,
            registers: self.registers,
        })
    }
}

impl<'a, I: PinId, C: InputConfiguration, S: I2c, const A: u8> TryFrom<Pin<'a, I, Input<C>, S, A>>
    for Pin<'a, I, Output, S, A>
{
    type Error = Error<S>;

    fn try_from(input: Pin<'a, I, Input<C>, S, A>) -> Result<Self, Self::Error> {
        input.into_push_pull_output()
    }
}
