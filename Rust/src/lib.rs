//! WitMotion Modbus sensor interface library
//! 
//! This library provides functionality to interface with WitMotion IMU sensors
//! using the Modbus protocol over RS485 serial communication.

pub mod registers;
pub mod modbus;
pub mod sensor;
pub mod serial;
pub mod error;

pub use error::{WitError, WitResult};
pub use sensor::{WitSensor, SensorData, DataUpdateFlags};
pub use registers::*;

/// Common baud rates for auto-scanning
pub const SUPPORTED_BAUD_RATES: &[u32] = &[
    9600, 19200, 38400, 57600, 115200, 2400, 4800, 230400, 460800, 921600
];

/// Default polling interval in milliseconds
pub const DEFAULT_POLL_INTERVAL_MS: u64 = 500;

/// Default number of registers to read (covers accelerometer, gyroscope, and angles)
pub const DEFAULT_READ_COUNT: u16 = 12;
