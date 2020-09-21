use crate::Error;
use embedded_hal::digital::v2::{InputPin, OutputPin};

/// Struct representing an unconnected pin. Can be substituted for any
/// of the optional input or output pins.
///
/// Any attempt to read or write this pin state returns an
/// Error::NotConnected
pub struct Unconnected;

impl InputPin for Unconnected {
    type Error = Error;
    fn is_high(&self) -> Result<bool, Self::Error> {
        Err(Error::NotConnected)
    }
    fn is_low(&self) -> Result<bool, Self::Error> {
        Err(Error::NotConnected)
    }
}

impl OutputPin for Unconnected {
    type Error = Error;
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Err(Error::NotConnected)
    }
    fn set_high(&mut self) -> Result<(), Self::Error> {
        Err(Error::NotConnected)
    }
}
