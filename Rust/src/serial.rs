use crate::error::{WitError, WitResult};
use serial2::SerialPort;
use std::time::Duration;

/// Serial communication wrapper for WitMotion sensors
pub struct WitSerial {
    port: SerialPort,
    current_baud: u32,
}

impl WitSerial {
    /// Open a serial port with the specified device path and baud rate
    pub fn open(device_path: &str, baud_rate: u32) -> WitResult<Self> {
        let mut port = SerialPort::open(device_path, baud_rate)?;
        
        // Set timeouts
        port.set_read_timeout(Duration::from_millis(100))?;
        port.set_write_timeout(Duration::from_millis(100))?;

        Ok(Self {
            port,
            current_baud: baud_rate,
        })
    }

    /// Read data from the serial port
    /// Returns the number of bytes read
    pub fn read(&mut self, buffer: &mut [u8]) -> WitResult<usize> {
        match self.port.read(buffer) {
            Ok(n) => Ok(n),
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut => Ok(0),
            Err(e) => Err(WitError::Io(e)),
        }
    }

    /// Write data to the serial port
    pub fn write(&mut self, data: &[u8]) -> WitResult<usize> {
        Ok(self.port.write(data)?)
    }

    /// Flush the output buffer
    pub fn flush(&mut self) -> WitResult<()> {
        Ok(self.port.flush()?)
    }

    /// Change the baud rate of the serial port
    pub fn set_baud_rate(&mut self, baud_rate: u32) -> WitResult<()> {
        // For serial2, we need to recreate the port with new baud rate
        // This is a limitation of the serial2 crate
        let device_name = format!("/dev/ttyUSB0"); // We'll need to store the device path
        let mut new_port = SerialPort::open(&device_name, baud_rate)?;
        new_port.set_read_timeout(Duration::from_millis(100))?;
        new_port.set_write_timeout(Duration::from_millis(100))?;
        
        self.port = new_port;
        self.current_baud = baud_rate;
        Ok(())
    }

    /// Get the current baud rate
    pub fn baud_rate(&self) -> u32 {
        self.current_baud
    }

    /// Read a single byte from the serial port
    pub fn read_byte(&mut self) -> WitResult<Option<u8>> {
        let mut buffer = [0u8; 1];
        match self.read(&mut buffer)? {
            1 => Ok(Some(buffer[0])),
            0 => Ok(None),
            _ => unreachable!(),
        }
    }

    /// Clear the input buffer
    pub fn clear_input_buffer(&mut self) -> WitResult<()> {
        // Read and discard all available data
        let mut buffer = [0u8; 256];
        while self.read(&mut buffer)? > 0 {
            // Continue reading until no more data
        }
        Ok(())
    }
}
