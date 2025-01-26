use core::fmt::Debug;

use embedded_hal::digital::{Error as DigitalError, ErrorKind as DigitalErrorKind};
use embedded_hal::i2c::ErrorType;
use embedded_hal_bus::i2c::AtomicError;

/// An error communicating with an expander.
pub struct Error<S: ErrorType> {
    error: S::Error,
}

impl<S: ErrorType> Debug for Error<S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Error").field("error", &self.error).finish()
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
            AtomicError::Other(error) => Self { error },
        }
    }
}
