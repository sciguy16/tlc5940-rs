///
/// Error raised in case there was an error
/// during communication with the TLC5940 chip.
///
#[derive(Debug)]
pub enum Error {
    /// An attempt was made to use an unconnected function (e.g. blank
    /// while the blanking pin is not wired up)
    NotConnected,
    /// An attempt was made to access an index out of range
    OutOfRange,
    /// An error occurred when working with SPI
    Spi,
    /// An error occurred when working with a PIN
    Pin,
}

/// Result wrapping the Error type
pub type Result<T> = core::result::Result<T, Error>;

/*impl<T> From<<T as embedded_hal::digital::v2::OutputPin>::Error> for Error where

{
}
*/
