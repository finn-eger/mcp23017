use core::fmt::Debug;

use embedded_hal::digital::{Error as DigitalError, ErrorKind as DigitalErrorKind};
use embedded_hal::i2c::ErrorType;
use embedded_hal_bus::i2c::AtomicError;
use thiserror::Error;

/// An error interacting with an expander.
#[derive(Error)]
pub enum Error<S: ErrorType> {
    /// An error communicating with an expander.
    Communication(S::Error),
}

impl<S: ErrorType<Error = impl Debug>> Debug for Error<S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Communication(e) => f.debug_tuple("Communication").field(e).finish(),
        }
    }
}

impl<S: ErrorType<Error = E>, E: Debug> DigitalError for Error<S> {
    fn kind(&self) -> DigitalErrorKind {
        DigitalErrorKind::Other
    }
}

impl<S: ErrorType> From<AtomicError<S::Error>> for Error<S> {
    fn from(value: AtomicError<S::Error>) -> Self {
        match value {
            AtomicError::Busy => unreachable!(),
            AtomicError::Other(error) => Self::Communication(error),
        }
    }
}
