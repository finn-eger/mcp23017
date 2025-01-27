//! Configurations for using pins to trigger interrupts.
//!
//! Interrupt handling is centralized via an [`InterruptController`], obtained
//! when splitting the driver. Rather querying the expander separately for each
//! pin when an interrupt is raised, the controller requests interrupt details
//! once and stores them locally for individual pins to check. This
//! significantly shortens the time taken to service and clear an interrupt,
//! minimizing timing quirks like missed edges.

use core::marker::PhantomData;
use core::sync::atomic::{AtomicU8, Ordering};

use embedded_hal::digital::InputPin;
use embedded_hal::i2c::I2c;
use embedded_hal_bus::i2c::{AtomicDevice, AtomicError};

use super::{Bank, PinMode};
use crate::error::Error;
use crate::pin::input::{Input, InputConfiguration};
use crate::pin::{Pin, PinId};
use crate::registers::Registers;

/// Marker type for pins set as interrupts.
pub struct Interrupt<C: InputConfiguration> {
    config: PhantomData<C>,
}

impl<C: InputConfiguration> PinMode for Interrupt<C> {}

impl<I: PinId, C: InputConfiguration, S: I2c, const A: u8> InputPin
    for Pin<'_, I, Interrupt<C>, S, A>
{
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(unsafe { self.registers.get(Registers::<I, S, A>::GPIO)? })
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        self.is_high().map(|x| !x)
    }
}

impl<'a, I: PinId, C: InputConfiguration, S: I2c, const A: u8> Pin<'a, I, Input<C>, S, A> {
    /// Reconfigure the pin to trigger interrupts.
    pub fn enable_interrupt(
        mut self,
        sense: Sense,
    ) -> Result<Pin<'a, I, Interrupt<C>, S, A>, Error<S>> {
        match sense {
            Sense::High => unsafe {
                self.registers.set(Registers::<I, S, A>::INTCON, true)?;
                self.registers.set(Registers::<I, S, A>::DEFVAL, false)?;
            },
            Sense::Low => unsafe {
                self.registers.set(Registers::<I, S, A>::INTCON, true)?;
                self.registers.set(Registers::<I, S, A>::DEFVAL, true)?;
            },
            Sense::Edge => unsafe {
                self.registers.set(Registers::<I, S, A>::INTCON, false)?;
            },
        }
        unsafe { self.registers.set(Registers::<I, S, A>::GPINTEN, true)? }

        Ok(Pin {
            id: PhantomData,
            mode: PhantomData,
            registers: self.registers,
        })
    }
}

/// An interrupt trigger condition.
#[derive(Clone, Copy)]
pub enum Sense {
    /// Trigger on low level.
    High,
    /// Trigger on high level.
    Low,
    /// Trigger on either edge.
    Edge,
}

impl<'a, I: PinId, C: InputConfiguration, S: I2c, const A: u8> Pin<'a, I, Interrupt<C>, S, A> {
    /// Reconfigure the pin not to trigger interrupts.
    pub fn disable_interrupt(mut self) -> Result<Pin<'a, I, Input<C>, S, A>, Error<S>> {
        unsafe { self.registers.set(Registers::<I, S, A>::GPINTEN, false)? };
        Ok(Pin {
            id: PhantomData,
            mode: PhantomData,
            registers: self.registers,
        })
    }
}

/// A centralized hub for coordinating interrupts across all pins on an
/// expander.
pub struct InterruptController<'a, S: I2c, const A: u8> {
    i2c: AtomicDevice<'a, S>,

    interrupt_flag: (AtomicU8, AtomicU8),
    interrupt_capture: (AtomicU8, AtomicU8),
}

impl<'a, S: I2c, const A: u8> InterruptController<'a, S, A> {
    pub(crate) unsafe fn new(i2c: AtomicDevice<'a, S>) -> Self {
        Self {
            i2c,
            interrupt_flag: (AtomicU8::new(0), AtomicU8::new(0)),
            interrupt_capture: (AtomicU8::new(0), AtomicU8::new(0)),
        }
    }

    /// Handle an interrupt on a bank.
    ///
    /// Calling this method clears the interrupt condition and records the cause
    /// internally. It should be called immediately in an interrupt service
    /// routine. To check if a specific pin triggered an interrupt, use
    /// [`Self::triggered()`] at any time.
    pub fn interrupt(&mut self, bank: Bank) -> Result<(), AtomicError<S::Error>> {
        let intf_address = match bank {
            #[allow(clippy::identity_op)]
            Bank::A => Registers::<crate::pin::A0, S, A>::INTF + 0,
            Bank::B => Registers::<crate::pin::B0, S, A>::INTF + 1,
        };

        let mut intf_read = [0x00];
        self.i2c.write_read(A, &[intf_address], &mut intf_read)?;

        match bank {
            Bank::A => self
                .interrupt_flag
                .0
                .fetch_or(intf_read[0], Ordering::Relaxed),
            Bank::B => self
                .interrupt_flag
                .1
                .fetch_or(intf_read[0], Ordering::Relaxed),
        };

        let intcap_address = match bank {
            #[allow(clippy::identity_op)]
            Bank::A => Registers::<crate::pin::A0, S, A>::INTCAP + 0,
            Bank::B => Registers::<crate::pin::B0, S, A>::INTCAP + 1,
        };

        let mut intcap_read = [0x00];
        self.i2c
            .write_read(A, &[intcap_address], &mut intcap_read)?;

        let masked_intcap_read = intcap_read[0] & intf_read[0];

        match bank {
            Bank::A => {
                let intcap = self.interrupt_capture.0.load(Ordering::Relaxed);
                let masked_intcap = intcap & !intf_read[0];

                let modified = masked_intcap + masked_intcap_read;
                self.interrupt_capture.0.store(modified, Ordering::Relaxed)
            }
            Bank::B => {
                let intcap = self.interrupt_capture.1.load(Ordering::Relaxed);
                let masked_intcap = intcap & !intf_read[0];

                let modified = masked_intcap + masked_intcap_read;
                self.interrupt_capture.1.store(modified, Ordering::Relaxed)
            }
        }

        Ok(())
    }

    /// Check whether a pin has triggered an interrupt since the last call to
    /// this method, and if so get the state at the pin's last interrupt.
    pub fn triggered<I: PinId, C: InputConfiguration>(
        &self,
        _pin: &Pin<'_, I, Interrupt<C>, S, A>,
    ) -> Option<bool> {
        let mask = 1 << I::NUMBER;

        let read = match I::BANK {
            Bank::A => self.interrupt_flag.0.fetch_and(!mask, Ordering::Relaxed),
            Bank::B => self.interrupt_flag.1.fetch_and(!mask, Ordering::Relaxed),
        };

        if read & mask != 0 {
            let read = match I::BANK {
                Bank::A => self.interrupt_capture.0.fetch_and(!mask, Ordering::Relaxed),
                Bank::B => self.interrupt_capture.1.fetch_and(!mask, Ordering::Relaxed),
            };

            Some(read & mask != 0)
        } else {
            None
        }
    }
}
