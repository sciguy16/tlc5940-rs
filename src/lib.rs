#![no_std]

use embedded_hal::blocking::spi::Write;
use embedded_hal::digital::v2::{InputPin, OutputPin};

pub mod connectors;
use connectors::*;

pub mod unconnected;
pub use unconnected::Unconnected;

pub mod error;
pub use error::{Error, Result};

pub enum OperatingMode {
    /// Grayscale PWM Mode
    GrayscalePWM,
    /// Dot Correction Data Input Mode
    DotCorrection,
    /// EEPROM Programming Mode
    Eeprom,
}

///
/// Handles communication with the MAX7219
/// chip for segmented displays. Each display can be
/// connected in series with another and controlled via
/// a single connection. The actual connection interface
/// is selected via constructor functions.
///
pub struct TLC5940<CONNECTOR, BLANK, XERR>
where
    BLANK: OutputPin,
    XERR: OutputPin,
{
    connector: CONNECTOR,

    /// Output enable/blanking. When set HIGH all outputs are disabled
    blank_pin: BLANK,
    /// `xerr` is an open-drain output that goes low if the Thermal Error
    /// Flag or LED Open Detection events trigger. Needs a pullup, active
    /// LOW
    xerr_pin: XERR,
    /// DOT correction values. Each channel should be in the 0-63 range
    /// as the TLC5940 accepts 6-bit values. The upper 2 bits of each
    /// value here are ignored when pushing changes to the chip.
    dot_correction: [u8; 16],
    /// Brightness values for each channel. Each channel should be in the
    /// 0-4095 range as the TLC5940 uses 12-bit PWM. The upper 4 bits of
    /// each value here are ignored when pushing changes to the chip.
    grayscale_values: [u16; 16],
    // /// Status returned from the device
    //status: StatusInformation,
}

// /// Status information returned from the chip
//pub struct StatusInformation;

impl<CONNECTOR, BLANK, XERR> TLC5940<CONNECTOR, BLANK, XERR>
where
    CONNECTOR: Connector,
    BLANK: OutputPin,
    XERR: OutputPin,
{
    ///
    /// Blanks the outputs.
    ///
    /// # Inputs
    ///
    /// * `is_blank: bool`: true for blank, false for not-blank
    ///
    /// # Errors
    ///
    /// * `Error::NotConnected` if a blanking pin was not configured
    ///
    pub fn blank(&mut self, is_blank: bool) -> Result<()> {
        // if not connected then just don't do anything. Not point in
        // overcomplicating the API
        /*if self.blank_pin == Unconnected {
            // No blanking pin set, return appropriate error
            return Err(Error::NotConnected);
        }*/

        if is_blank {
            //TODO Sort the Error type conversions out so that ? can be
            // used to propagate the error
            self.blank_pin.set_high();
        } else {
            self.blank_pin.set_low();
        }
        Ok(())
    }

    /*/// Read status information from the device
    pub fn read_status(&mut self) -> Result<&StatusInformation> {
        // Get status from device
        // Return borrow of self.status_information
        todo!();
    }*/

    /// Store an intensity value
    pub fn set_level(&mut self, output: u8, level: u16) -> Result<()> {
        // There can only be 16 outputs
        if output >= 16 {
            return Err(Error::OutOfRange);
        }

        // Ignore out of range greyscale values by just taking the lower
        // 12 bits
        self.grayscale_values[output as usize] = level & 0x0fff;
        Ok(())
    }

    /// Store all levels at the same time
    pub fn set_levels(&mut self, levels: [u16; 16]) -> Result<()> {
        for (idx, level) in levels.iter().enumerate() {
            self.set_level(idx as u8, *level)?;
        }
        Ok(())
    }

    /// Transfer the stored leves to the chip
    pub fn update(&mut self) -> Result<()> {
        // Pack the intensity values into a 24-byte array
        let mut packed = [0_u8; 6];

        // Write it on the wire
        self.connector.write_raw(&packed);
        todo!();
    }

    /// Set the dot correction values
    pub fn set_dot_correction(&mut self) -> Result<()> {
        // Pack the intensity values into a 24-byte array
        let mut packed = [0_u8; 6];

        // do the thing to make it accept dot correction

        // Write it on the wire
        self.connector.write_raw(&packed);
        todo!();
    }

    // internal constructor, users should call ::from_pins or ::from_spi
    fn new(
        connector: CONNECTOR,
        blank_pin: BLANK,
        xerr_pin: XERR,
    ) -> Result<Self> {
        let mut tlc5940 = Self {
            connector,
            blank_pin,
            xerr_pin,
            dot_correction: [0; 16],
            grayscale_values: [0; 16],
        };

        tlc5940.init()?;
        Ok(tlc5940)
    }

    fn init(&mut self) -> Result<()> {
        // Probably don't need this function
        //self.blank(false);

        Ok(())
    }
}

impl<DATA, CS, SCK, BLANK, XERR>
    TLC5940<PinConnector<DATA, CS, SCK>, BLANK, XERR>
where
    DATA: OutputPin,
    CS: OutputPin,
    SCK: OutputPin,
    BLANK: OutputPin,
    XERR: OutputPin,
{
    ///
    /// Construct a new MAX7219 driver instance from DATA, CS and SCK pins.
    ///
    /// # Arguments
    ///
    /// * `displays` - number of displays connected in series
    /// * `data` - the MOSI/DATA PIN used to send data through to the display set to output mode
    /// * `cs` - the CS PIN used to LOAD register on the display set to output mode
    /// * `sck` - the SCK clock PIN used to drive the clock set to output mode
    ///
    /// # Errors
    ///
    /// * `DataError` - returned in case there was an error during data transfer
    ///
    pub fn from_pins(
        data: DATA,
        cs: CS,
        sck: SCK,
        blank_pin: BLANK,
        xerr_pin: XERR,
    ) -> Result<Self> {
        TLC5940::new(PinConnector::new(data, cs, sck), blank_pin, xerr_pin)
    }
}

impl<SPI, BLANK, XERR> TLC5940<SpiConnector<SPI>, BLANK, XERR>
where
    SPI: Write<u8>,
    BLANK: OutputPin,
    XERR: OutputPin,
{
    ///
    /// Construct a new MAX7219 driver instance from pre-existing SPI in full hardware mode.
    /// The SPI will control CS (LOAD) line according to it's internal mode set.
    /// If you need the CS line to be controlled manually use MAX7219::from_spi_cs
    ///
    /// * `NOTE` - make sure the SPI is initialized in MODE_0 with max 10 Mhz frequency.
    ///
    /// # Arguments
    ///
    /// * `displays` - number of displays connected in series
    /// * `spi` - the SPI interface initialized with MOSI, MISO(unused) and CLK
    ///
    /// # Errors
    ///
    /// * `DataError` - returned in case there was an error during data transfer
    ///
    pub fn from_spi(
        displays: usize,
        spi: SPI,
        blank_pin: BLANK,
        xerr_pin: XERR,
    ) -> Result<Self> {
        TLC5940::new(SpiConnector::new(displays, spi), blank_pin, xerr_pin)
    }
}

impl<SPI, CS, BLANK, XERR> TLC5940<SpiConnectorSW<SPI, CS>, BLANK, XERR>
where
    SPI: Write<u8>,
    CS: OutputPin,
    BLANK: OutputPin,
    XERR: OutputPin,
{
    ///
    /// Construct a new TLC5940 driver instance from pre-existing SPI and CS pin
    /// set to output. This version of the connection uses the CS pin manually
    /// to avoid issues with how the CS mode is handled in hardware SPI implementations.
    ///
    /// * `NOTE` - make sure the SPI is initialized in MODE_0 with max 10 Mhz frequency.
    ///
    /// # Arguments
    ///
    /// * `displays` - number of displays connected in series
    /// * `spi` - the SPI interface initialized with MOSI, MISO(unused) and CLK
    /// * `cs` - the CS PIN used to LOAD register on the display set to output mode
    ///
    /// # Errors
    ///
    /// * `DataError` - returned in case there was an error during data transfer
    ///
    pub fn from_spi_cs(
        displays: usize,
        spi: SPI,
        cs: CS,
        blank_pin: BLANK,
        xerr_pin: XERR,
    ) -> Result<Self> {
        TLC5940::new(
            SpiConnectorSW::new(displays, spi, cs),
            blank_pin,
            xerr_pin,
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
