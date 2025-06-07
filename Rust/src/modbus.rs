use crate::error::{WitError, WitResult};
use crc::{Crc, CRC_16_MODBUS};

/// Modbus CRC calculator
const MODBUS_CRC: Crc<u16> = Crc::<u16>::new(&CRC_16_MODBUS);

/// Modbus protocol handler for WitMotion sensors
pub struct ModbusProtocol {
    slave_address: u8,
    data_buffer: Vec<u8>,
    read_register_index: u16,
}

impl ModbusProtocol {
    /// Create a new Modbus protocol handler
    pub fn new(slave_address: u8) -> Self {
        Self {
            slave_address,
            data_buffer: Vec::with_capacity(256),
            read_register_index: 0,
        }
    }

    /// Generate a Modbus read request
    pub fn generate_read_request(&mut self, start_register: u16, num_registers: u16) -> Vec<u8> {
        let mut frame = Vec::with_capacity(8);
        
        // Slave address
        frame.push(self.slave_address);
        // Function code (Read Holding Registers)
        frame.push(0x03);
        // Starting register address (high byte, low byte)
        frame.extend_from_slice(&start_register.to_be_bytes());
        // Number of registers (high byte, low byte)
        frame.extend_from_slice(&num_registers.to_be_bytes());
        
        // Calculate and append CRC
        let crc = MODBUS_CRC.checksum(&frame);
        frame.extend_from_slice(&crc.to_le_bytes()); // Modbus uses little-endian CRC
        
        self.read_register_index = start_register;
        frame
    }

    /// Generate a Modbus write request
    pub fn generate_write_request(&self, register: u16, value: u16) -> Vec<u8> {
        let mut frame = Vec::with_capacity(8);
        
        // Slave address
        frame.push(self.slave_address);
        // Function code (Write Single Register)
        frame.push(0x06);
        // Register address (high byte, low byte)
        frame.extend_from_slice(&register.to_be_bytes());
        // Register value (high byte, low byte)
        frame.extend_from_slice(&value.to_be_bytes());
        
        // Calculate and append CRC
        let crc = MODBUS_CRC.checksum(&frame);
        frame.extend_from_slice(&crc.to_le_bytes());
        
        frame
    }

    /// Process incoming byte and return parsed register data if complete frame received
    pub fn process_byte(&mut self, byte: u8) -> WitResult<Option<(u16, Vec<i16>)>> {
        self.data_buffer.push(byte);

        // Need at least 5 bytes for a valid response (addr + func + len + 2*CRC)
        if self.data_buffer.len() < 5 {
            return Ok(None);
        }

        // Check if we have enough bytes for a complete frame
        if self.data_buffer.len() >= 3 {
            let expected_length = self.data_buffer[2] as usize + 5; // data length + overhead
            
            if self.data_buffer.len() < expected_length {
                return Ok(None); // Wait for more data
            }

            // We have a complete frame, process it
            let result = self.parse_response();
            self.data_buffer.clear();
            return result.map(Some);
        }

        Ok(None)
    }

    /// Parse a complete Modbus response
    fn parse_response(&self) -> WitResult<(u16, Vec<i16>)> {
        if self.data_buffer.len() < 5 {
            return Err(WitError::InvalidParameter("Frame too short".to_string()));
        }

        // Check function code
        if self.data_buffer[1] != 0x03 {
            return Err(WitError::InvalidParameter("Invalid function code".to_string()));
        }

        let data_length = self.data_buffer[2] as usize;
        let expected_frame_length = data_length + 5;

        if self.data_buffer.len() != expected_frame_length {
            return Err(WitError::InvalidParameter("Invalid frame length".to_string()));
        }

        // Verify CRC
        let received_crc = u16::from_le_bytes([
            self.data_buffer[expected_frame_length - 2],
            self.data_buffer[expected_frame_length - 1],
        ]);
        
        let calculated_crc = MODBUS_CRC.checksum(&self.data_buffer[0..expected_frame_length - 2]);
        
        if received_crc != calculated_crc {
            return Err(WitError::CrcMismatch);
        }

        // Extract register values
        let mut registers = Vec::new();
        let num_registers = data_length / 2;
        
        for i in 0..num_registers {
            let offset = 3 + i * 2;
            let value = i16::from_be_bytes([
                self.data_buffer[offset],
                self.data_buffer[offset + 1],
            ]);
            registers.push(value);
        }

        Ok((self.read_register_index, registers))
    }

    /// Clear the internal data buffer
    pub fn clear_buffer(&mut self) {
        self.data_buffer.clear();
    }

    /// Check if buffer should be reset (too much data accumulated)
    pub fn should_reset_buffer(&self) -> bool {
        self.data_buffer.len() > 256
    }
}

/// Modbus command types
#[derive(Debug, Clone, Copy)]
pub enum ModbusCommand {
    ReadHoldingRegisters,
    WriteMultipleRegisters,
}

/// Create a Modbus read request frame
pub fn create_read_request(slave_address: u8, start_register: u16, num_registers: u16) -> Vec<u8> {
    let mut protocol = ModbusProtocol::new(slave_address);
    protocol.generate_read_request(start_register, num_registers)
}

/// Parse a Modbus response frame
pub fn parse_response(frame: &[u8]) -> WitResult<Vec<u16>> {
    if frame.len() < 5 {
        return Err(WitError::InvalidParameter("Frame too short".to_string()));
    }

    // Check function code
    if frame[1] != 0x03 {
        return Err(WitError::InvalidParameter("Invalid function code".to_string()));
    }

    let data_length = frame[2] as usize;
    let expected_frame_length = data_length + 5;

    if frame.len() != expected_frame_length {
        return Err(WitError::InvalidParameter("Invalid frame length".to_string()));
    }

    // Verify CRC
    let received_crc = u16::from_le_bytes([
        frame[expected_frame_length - 2],
        frame[expected_frame_length - 1],
    ]);
    
    let calculated_crc = MODBUS_CRC.checksum(&frame[0..expected_frame_length - 2]);
    
    if received_crc != calculated_crc {
        return Err(WitError::CrcMismatch);
    }

    // Extract register values as u16 (unsigned)
    let mut registers = Vec::new();
    let num_registers = data_length / 2;
    
    for i in 0..num_registers {
        let offset = 3 + i * 2;
        let value = u16::from_be_bytes([
            frame[offset],
            frame[offset + 1],
        ]);
        registers.push(value);
    }

    Ok(registers)
}
