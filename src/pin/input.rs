//! Configurations for using pins as input.

use core::marker::PhantomData;

use embedded_hal::digital::InputPin;
use embedded_hal::i2c::I2c;

use super::output::Output;
use super::{InputPinId, Pin, PinId, PinMode};
use crate::error::Error;
use crate::registers::Registers;

/// Marker type for pins set as inputs.
pub struct Input<C: InputConfiguration> {
    config: PhantomData<C>,
}

impl<C: InputConfiguration> PinMode for Input<C> {}

/// Marker trait for input pin configurations.
pub trait InputConfiguration {}

/// Marker type for input pins configured as floating.
pub struct Floating;

impl InputConfiguration for Floating {}

/// Marker type for input pins configured as pull ups.
pub struct PullUp;

impl InputConfiguration for PullUp {}

impl<I: PinId, C: InputConfiguration, S: I2c, const A: u8> InputPin for Pin<'_, I, Input<C>, S, A> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(unsafe { self.registers.get(Registers::<I, S, A>::GPIO)? })
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        self.is_high().map(|x| !x)
    }
}

impl<'a, I: PinId, S: I2c, const A: u8> Pin<'a, I, Input<PullUp>, S, A> {
    /// Reconfigure the pin with the internal pull up disconnected.
    pub fn into_floating_input(mut self) -> Result<Pin<'a, I, Input<Floating>, S, A>, Error<S>> {
        unsafe { self.registers.set(Registers::<I, S, A>::GPPU, false)? }
        Ok(Pin {
            id: PhantomData,
            mode: PhantomData,
            registers: self.registers,
        })
    }
}

impl<'a, I: PinId, S: I2c, const A: u8> TryFrom<Pin<'a, I, Input<PullUp>, S, A>>
    for Pin<'a, I, Input<Floating>, S, A>
{
    type Error = Error<S>;

    fn try_from(input: Pin<'a, I, Input<PullUp>, S, A>) -> Result<Self, Self::Error> {
        input.into_floating_input()
    }
}

impl<'a, I: PinId, S: I2c, const A: u8> Pin<'a, I, Input<Floating>, S, A> {
    /// Reconfigure the pin with the internal pull up connected.
    pub fn into_pull_up_input(mut self) -> Result<Pin<'a, I, Input<PullUp>, S, A>, Error<S>> {
        unsafe { self.registers.set(Registers::<I, S, A>::GPPU, true)? }
        Ok(Pin {
            id: PhantomData,
            mode: PhantomData,
            registers: self.registers,
        })
    }
}

impl<'a, I: PinId, S: I2c, const A: u8> TryFrom<Pin<'a, I, Input<Floating>, S, A>>
    for Pin<'a, I, Input<PullUp>, S, A>
{
    type Error = Error<S>;

    fn try_from(input: Pin<'a, I, Input<Floating>, S, A>) -> Result<Self, Self::Error> {
        input.into_pull_up_input()
    }
}

impl<'a, I: InputPinId, S: I2c, const A: u8> Pin<'a, I, Output, S, A> {
    /// Reconfigure the pin as an input, with the internal pull up disconnected.
    pub fn into_floating_input(mut self) -> Result<Pin<'a, I, Input<Floating>, S, A>, Error<S>> {
        unsafe { self.registers.set(Registers::<I, S, A>::GPPU, false)? }
        unsafe { self.registers.set(Registers::<I, S, A>::IODIR, true)? }
        Ok(Pin {
            id: PhantomData,
            mode: PhantomData,
            registers: self.registers,
        })
    }

    /// Reconfigure the pin as an input, with the internall pull up connected.
    pub fn into_pull_up_input(mut self) -> Result<Pin<'a, I, Input<PullUp>, S, A>, Error<S>> {
        unsafe { self.registers.set(Registers::<I, S, A>::GPPU, true)? }
        unsafe { self.registers.set(Registers::<I, S, A>::IODIR, true)? }
        Ok(Pin {
            id: PhantomData,
            mode: PhantomData,
            registers: self.registers,
        })
    }
}

impl<'a, I: InputPinId, S: I2c, const A: u8> TryFrom<Pin<'a, I, Output, S, A>>
    for Pin<'a, I, Input<Floating>, S, A>
{
    type Error = Error<S>;

    fn try_from(input: Pin<'a, I, Output, S, A>) -> Result<Self, Self::Error> {
        input.into_floating_input()
    }
}

impl<'a, I: InputPinId, S: I2c, const A: u8> TryFrom<Pin<'a, I, Output, S, A>>
    for Pin<'a, I, Input<PullUp>, S, A>
{
    type Error = Error<S>;

    fn try_from(input: Pin<'a, I, Output, S, A>) -> Result<Self, Self::Error> {
        input.into_pull_up_input()
    }
}
