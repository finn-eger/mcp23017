use core::marker::PhantomData;

use embedded_hal::digital::ErrorType as DigitalErrorType;
use embedded_hal::i2c::I2c;
use embedded_hal_bus::i2c::AtomicDevice;
use input::{Floating, Input};
use output::Output;

use crate::error::Error;
use crate::registers::Registers;

pub mod input;
pub mod interrupt;
pub mod output;

/// An individually controllable pin on an expander.
///
/// All methods may error if communication with the device fails.
pub struct Pin<'a, I: PinId, M: PinMode, S: I2c, const A: u8> {
    id: PhantomData<I>,
    mode: PhantomData<M>,

    pub(crate) registers: Registers<I, AtomicDevice<'a, S>, A>,
}

impl<'a, I: PinId, S: I2c, const A: u8> Pin<'a, I, Input<Floating>, S, A> {
    pub(crate) unsafe fn new(i2c: AtomicDevice<'a, S>) -> Self {
        Self {
            id: PhantomData,
            mode: PhantomData,
            registers: Registers::new(i2c),
        }
    }
}

impl<I: PinId, M: PinMode, S: I2c, const A: u8> DigitalErrorType for Pin<'_, I, M, S, A> {
    type Error = Error<S>;
}

/// Marker trait for pin modes.
pub trait PinMode {}

/// All pins on an expander, in their default configurations.
pub struct Pins<'a, S: I2c, const A: u8> {
    pub a0: Pin<'a, A0, Input<Floating>, S, A>,
    pub a1: Pin<'a, A1, Input<Floating>, S, A>,
    pub a2: Pin<'a, A2, Input<Floating>, S, A>,
    pub a3: Pin<'a, A3, Input<Floating>, S, A>,
    pub a4: Pin<'a, A4, Input<Floating>, S, A>,
    pub a5: Pin<'a, A5, Input<Floating>, S, A>,
    pub a6: Pin<'a, A6, Input<Floating>, S, A>,
    pub a7: Pin<'a, A7, Output, S, A>,
    pub b0: Pin<'a, B0, Input<Floating>, S, A>,
    pub b1: Pin<'a, B1, Input<Floating>, S, A>,
    pub b2: Pin<'a, B2, Input<Floating>, S, A>,
    pub b3: Pin<'a, B3, Input<Floating>, S, A>,
    pub b4: Pin<'a, B4, Input<Floating>, S, A>,
    pub b5: Pin<'a, B5, Input<Floating>, S, A>,
    pub b6: Pin<'a, B6, Input<Floating>, S, A>,
    pub b7: Pin<'a, B7, Output, S, A>,
}

/// Bank A pin 0
pub struct A0;
/// Bank A pin 1
pub struct A1;
/// Bank A pin 2
pub struct A2;
/// Bank A pin 3
pub struct A3;
/// Bank A pin 4
pub struct A4;
/// Bank A pin 5
pub struct A5;
/// Bank A pin 6
pub struct A6;
/// Bank A pin 7
pub struct A7;

/// Bank B pin 0
pub struct B0;
/// Bank B pin 1
pub struct B1;
/// Bank B pin 2
pub struct B2;
/// Bank B pin 3
pub struct B3;
/// Bank B pin 4
pub struct B4;
/// Bank B pin 5
pub struct B5;
/// Bank B pin 6
pub struct B6;
/// Bank B pin 7
pub struct B7;

/// Marker trait for a pin identifier.
pub trait PinId {
    /// The pin's bank.
    const BANK: Bank;
    /// The pin's number.
    const NUMBER: u8;
}

/// Marker type for a bank/port.
pub enum Bank {
    /// Bank A
    A,
    /// Bank B
    B,
}

impl PinId for A0 {
    const BANK: Bank = Bank::A;
    const NUMBER: u8 = 0;
}
impl PinId for A1 {
    const BANK: Bank = Bank::A;
    const NUMBER: u8 = 1;
}
impl PinId for A2 {
    const BANK: Bank = Bank::A;
    const NUMBER: u8 = 2;
}
impl PinId for A3 {
    const BANK: Bank = Bank::A;
    const NUMBER: u8 = 3;
}
impl PinId for A4 {
    const BANK: Bank = Bank::A;
    const NUMBER: u8 = 4;
}
impl PinId for A5 {
    const BANK: Bank = Bank::A;
    const NUMBER: u8 = 5;
}
impl PinId for A6 {
    const BANK: Bank = Bank::A;
    const NUMBER: u8 = 6;
}
impl PinId for A7 {
    const BANK: Bank = Bank::A;
    const NUMBER: u8 = 7;
}

impl PinId for B0 {
    const BANK: Bank = Bank::B;
    const NUMBER: u8 = 0;
}
impl PinId for B1 {
    const BANK: Bank = Bank::B;
    const NUMBER: u8 = 1;
}
impl PinId for B2 {
    const BANK: Bank = Bank::B;
    const NUMBER: u8 = 2;
}
impl PinId for B3 {
    const BANK: Bank = Bank::B;
    const NUMBER: u8 = 3;
}
impl PinId for B4 {
    const BANK: Bank = Bank::B;
    const NUMBER: u8 = 4;
}
impl PinId for B5 {
    const BANK: Bank = Bank::B;
    const NUMBER: u8 = 5;
}
impl PinId for B6 {
    const BANK: Bank = Bank::B;
    const NUMBER: u8 = 6;
}
impl PinId for B7 {
    const BANK: Bank = Bank::B;
    const NUMBER: u8 = 7;
}

/// Marker trait for pins that may be configured as inputs.
///
/// Pins A7 and B7 must not be configured as inputs, as mandated by the
/// datasheet.
pub trait InputPinId: PinId {}

impl InputPinId for A0 {}
impl InputPinId for A1 {}
impl InputPinId for A2 {}
impl InputPinId for A3 {}
impl InputPinId for A4 {}
impl InputPinId for A5 {}
impl InputPinId for A6 {}

impl InputPinId for B0 {}
impl InputPinId for B1 {}
impl InputPinId for B2 {}
impl InputPinId for B3 {}
impl InputPinId for B4 {}
impl InputPinId for B5 {}
impl InputPinId for B6 {}
