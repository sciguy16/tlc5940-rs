use embedded_hal::blocking::spi::Write;
use embedded_hal::digital::v2::OutputPin;

use crate::{Error, Result};

/// Describes the interface used to connect to the MX7219
pub trait Connector {
    ///
    /// Writes a byte array to the device
    ///
    /// # Arguments
    ///
    /// * `data` - the data byte value to write
    ///
    /// # Errors
    ///
    /// * `DataError` - returned in case there was an error during data transfer
    ///
    fn write_raw(&mut self, data: &[u8]) -> Result<()>;
}

/// Direct GPIO pins connector
pub struct PinConnector<DATA, CS, SCK>
where
    DATA: OutputPin,
    CS: OutputPin,
    SCK: OutputPin,
{
    data: DATA,
    cs: CS,
    sck: SCK,
}

impl<DATA, CS, SCK> PinConnector<DATA, CS, SCK>
where
    DATA: OutputPin,
    CS: OutputPin,
    SCK: OutputPin,
{
    pub(crate) fn new(data: DATA, cs: CS, sck: SCK) -> Self {
        PinConnector { data, cs, sck }
    }
}

impl<DATA, CS, SCK> Connector for PinConnector<DATA, CS, SCK>
where
    DATA: OutputPin,
    CS: OutputPin,
    SCK: OutputPin,
{
    fn write_raw(&mut self, data: &[u8]) -> Result<()> {
        self.cs.set_low().map_err(|_| Error::Pin)?;
        // Iterate over byte array
        for value in data {
            // Iterate over bits in byte
            for i in 0..8 {
                if value & (1 << (7 - i)) > 0 {
                    self.data.set_high().map_err(|_| Error::Pin)?;
                } else {
                    self.data.set_low().map_err(|_| Error::Pin)?;
                }

                self.sck.set_high().map_err(|_| Error::Pin)?;
                self.sck.set_low().map_err(|_| Error::Pin)?;
            }
        }
        self.cs.set_high().map_err(|_| Error::Pin)?;

        Ok(())
    }
}

pub struct SpiConnector<SPI>
where
    SPI: Write<u8>,
{
    devices: usize,
    buffer: [u8; 2],
    spi: SPI,
}

/// Hardware controlled CS connector with SPI transfer
impl<SPI> SpiConnector<SPI>
where
    SPI: Write<u8>,
{
    pub(crate) fn new(displays: usize, spi: SPI) -> Self {
        SpiConnector {
            devices: displays,
            buffer: [0; 2],
            spi,
        }
    }
}

impl<SPI> Connector for SpiConnector<SPI>
where
    SPI: Write<u8>,
{
    fn write_raw(&mut self, data: &[u8]) -> Result<()> {
        self.spi.write(data).map_err(|_| Error::Spi)?;

        Ok(())
    }
}

/// Software controlled CS connector with SPI transfer
pub struct SpiConnectorSW<SPI, CS>
where
    SPI: Write<u8>,
    CS: OutputPin,
{
    spi_c: SpiConnector<SPI>,
    cs: CS,
}

impl<SPI, CS> SpiConnectorSW<SPI, CS>
where
    SPI: Write<u8>,
    CS: OutputPin,
{
    pub(crate) fn new(displays: usize, spi: SPI, cs: CS) -> Self {
        SpiConnectorSW {
            spi_c: SpiConnector::new(displays, spi),
            cs,
        }
    }
}

impl<SPI, CS> Connector for SpiConnectorSW<SPI, CS>
where
    SPI: Write<u8>,
    CS: OutputPin,
{
    fn write_raw(&mut self, data: &[u8]) -> Result<()> {
        self.cs.set_low().map_err(|_| Error::Pin)?;
        self.spi_c.write_raw(data).map_err(|_| Error::Spi)?;
        self.cs.set_high().map_err(|_| Error::Pin)?;

        Ok(())
    }
}
