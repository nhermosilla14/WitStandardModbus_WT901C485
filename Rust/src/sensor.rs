use crate::{
    error::{WitError, WitResult},
    modbus::ModbusProtocol,
    registers::*,
    serial::WitSerial,
    SUPPORTED_BAUD_RATES, DEFAULT_READ_COUNT,
};
use bitflags::bitflags;
use std::{collections::HashMap, thread, time::Duration};

/// Scaling factors for sensor data conversion
/// Accelerometer: ±16g range over 16-bit signed integer
pub const ACC_SCALE: f32 = 16.0 / 32768.0;
/// Gyroscope: ±2000°/s range over 16-bit signed integer  
pub const GYRO_SCALE: f32 = 2000.0 / 32768.0;
/// Angle: ±180° range over 16-bit signed integer
pub const ANGLE_SCALE: f32 = 180.0 / 32768.0;
/// Magnetometer: raw values (no scaling)
pub const MAG_SCALE: f32 = 1.0;

bitflags! {
    /// Flags indicating which sensor data has been updated
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DataUpdateFlags: u8 {
        const ACC = 0x01;      // Accelerometer data updated
        const GYRO = 0x02;     // Gyroscope data updated  
        const ANGLE = 0x04;    // Angle data updated
        const MAG = 0x08;      // Magnetometer data updated
        const READ = 0x80;     // Generic read update
    }
}

/// Sensor data structure containing scaled measurements
#[derive(Debug, Clone)]
pub struct SensorData {
    pub accelerometer: [f32; 3], // [x, y, z]
    pub gyroscope: [f32; 3], // [x, y, z]
    pub angles: [f32; 3], // [roll, pitch, yaw]
    pub magnetometer: [i16; 3], // [x, y, z]
    pub temperature: f32,
    /// Flags indicating which data was updated
    pub update_flags: DataUpdateFlags,
}

impl Default for SensorData {
    fn default() -> Self {
        Self {
            accelerometer: [0.0; 3],
            gyroscope: [0.0; 3],
            angles: [0.0; 3],
            magnetometer: [0; 3],
            temperature: 0.0,
            update_flags: DataUpdateFlags::empty(),
        }
    }
}

impl SensorData {
    /// Create new empty sensor data
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if accelerometer data was updated
    pub fn has_accelerometer_update(&self) -> bool {
        self.update_flags.contains(DataUpdateFlags::ACC)
    }

    /// Check if gyroscope data was updated
    pub fn has_gyroscope_update(&self) -> bool {
        self.update_flags.contains(DataUpdateFlags::GYRO)
    }

    /// Check if angle data was updated
    pub fn has_angle_update(&self) -> bool {
        self.update_flags.contains(DataUpdateFlags::ANGLE)
    }

    /// Check if magnetometer data was updated
    pub fn has_magnetometer_update(&self) -> bool {
        self.update_flags.contains(DataUpdateFlags::MAG)
    }
}

/// Main WitMotion sensor interface
pub struct WitSensor {
    serial: WitSerial,
    modbus: ModbusProtocol,
    registers: HashMap<u16, i16>,
    current_baud: u32,
}

impl WitSensor {
    /// Create a new WitMotion sensor interface
    pub fn new(
        device_path: &str,
        slave_address: u8,
    ) -> WitResult<Self> {
        // Start with 9600 as default baud rate for initial connection
        let default_baud = 9600;
        let serial = WitSerial::open(device_path, default_baud)?;        
        let modbus = ModbusProtocol::new(slave_address);
        let registers = HashMap::new();

        Ok(Self {
            serial,
            modbus,
            registers,
            current_baud: default_baud,
        })
    }

    /// Initialize the sensor
    pub fn init(&mut self) -> WitResult<()> {
        // No initialization needed for PC-based operation
        Ok(())
    }

    /// Auto-scan for the sensor by trying different baud rates
    pub fn auto_scan(&mut self) -> WitResult<u32> {
        println!("Scanning for sensor...");
        
        for &baud_rate in SUPPORTED_BAUD_RATES {
            println!("Trying baud rate: {}", baud_rate);
            
            if let Ok(()) = self.serial.set_baud_rate(baud_rate) {
                self.current_baud = baud_rate;
                
                // Clear any existing data
                self.serial.clear_input_buffer()?;
                
                // Try to read some registers
                for _retry in 0..2 {
                    if let Ok(()) = self.read_registers(AX, 3) {
                        thread::sleep(Duration::from_millis(200));
                        
                        // Check if we received any data
                        if self.process_incoming_data()?.is_some() {
                            println!("Found sensor at {} baud", baud_rate);
                            return Ok(baud_rate);
                        }
                    }
                }
            }
        }
        
        Err(WitError::SensorNotFound)
    }

    /// Read registers from the sensor
    pub fn read_registers(&mut self, start_register: u16, count: u16) -> WitResult<()> {
        let request = self.modbus.generate_read_request(start_register, count);
        self.send_data(&request)?;
        Ok(())
    }
    

    /// Write a register value to the sensor
    pub fn write_register(&mut self, register: u16, value: u16) -> WitResult<()> {
        let request = self.modbus.generate_write_request(register, value);
        self.send_data(&request)?;
        Ok(())
    }


    /// Send data via RS485 with proper direction control
    fn send_data(&mut self, data: &[u8]) -> WitResult<()> {
        // Calculate transmission delay based on baud rate
        let delay_us = (1_000_000 * data.len() as u64 * 10) / self.current_baud as u64 + 300;
        
        // Send data (no GPIO control needed for PC)
        self.serial.write(data)?;
        self.serial.flush()?;
        
        // Wait for transmission to complete
        thread::sleep(Duration::from_micros(delay_us));
        
        Ok(())
    }

    /// Process incoming data and return sensor data if available
    pub fn process_incoming_data(&mut self) -> WitResult<Option<SensorData>> {
        let mut sensor_data = None;
        
        // Read all available bytes
        loop {
            match self.serial.read_byte()? {
                Some(byte) => {
                    if let Some((start_reg, values)) = self.modbus.process_byte(byte)? {
                        // Update internal register storage
                        for (i, &value) in values.iter().enumerate() {
                            self.registers.insert(start_reg + i as u16, value);
                        }
                        
                        // Convert to sensor data
                        sensor_data = Some(self.extract_sensor_data(start_reg, &values));
                    }
                }
                None => break, // No more data available
            }
            
            // Reset buffer if it gets too large
            if self.modbus.should_reset_buffer() {
                self.modbus.clear_buffer();
            }
        }
        
        Ok(sensor_data)
    }

    /// Extract and scale sensor data from raw register values
    fn extract_sensor_data(&self, start_register: u16, values: &[i16]) -> SensorData {
        let mut data = SensorData::new();
        let mut update_flags = DataUpdateFlags::empty();
        
        // Process each register value
        for (i, &value) in values.iter().enumerate() {
            let reg = start_register + i as u16;
            
            match reg {
                // Accelerometer registers (±16g range)
                AX..=AZ => {
                    let axis = (reg - AX) as usize;
                    data.accelerometer[axis] = value as f32 / 32768.0 * 16.0;
                    if reg == AZ {
                        update_flags |= DataUpdateFlags::ACC;
                    }
                }
                // Gyroscope registers (±2000°/s range)
                GX..=GZ => {
                    let axis = (reg - GX) as usize;
                    data.gyroscope[axis] = value as f32 / 32768.0 * 2000.0;
                    if reg == GZ {
                        update_flags |= DataUpdateFlags::GYRO;
                    }
                }
                // Magnetometer registers
                HX..=HZ => {
                    let axis = (reg - HX) as usize;
                    data.magnetometer[axis] = value;
                    if reg == HZ {
                        update_flags |= DataUpdateFlags::MAG;
                    }
                }
                // Angle registers (±180° range)
                ROLL..=YAW => {
                    let axis = (reg - ROLL) as usize;
                    data.angles[axis] = value as f32 / 32768.0 * 180.0;
                    if reg == YAW {
                        update_flags |= DataUpdateFlags::ANGLE;
                    }
                }
                // Temperature register
                TEMP => {
                    data.temperature = value as f32 / 100.0; // Assuming temperature scaling
                }
                _ => {
                    update_flags |= DataUpdateFlags::READ;
                }
            }
        }
        
        data.update_flags = update_flags;
        data
    }

    /// Read sensor data continuously
    pub fn read_sensor_data(&mut self) -> WitResult<SensorData> {
        // Request standard sensor data (accelerometer, gyroscope, angles)
        self.read_registers(AX, DEFAULT_READ_COUNT)?;
        
        // Wait a bit for response
        thread::sleep(Duration::from_millis(50));
        
        // Process incoming data
        match self.process_incoming_data()? {
            Some(data) => Ok(data),
            None => Ok(SensorData::new()), // Return empty data if nothing received
        }
    }

    /// Get the current baud rate
    pub fn current_baud_rate(&self) -> u32 {
        self.current_baud
    }

    /// Get a register value by address
    pub fn get_register(&self, register: u16) -> Option<i16> {
        self.registers.get(&register).copied()
    }

    /// Get all register values
    pub fn get_all_registers(&self) -> &HashMap<u16, i16> {
        &self.registers
    }
}
