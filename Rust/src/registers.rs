/// Register addresses for WitMotion sensors
/// These correspond to the definitions in REG.h

// Control and configuration registers
pub const SAVE: u16 = 0x00;
pub const CALSW: u16 = 0x01;
pub const RSW: u16 = 0x02;
pub const RRATE: u16 = 0x03;
pub const BAUD: u16 = 0x04;

// Offset registers
pub const AXOFFSET: u16 = 0x05;
pub const AYOFFSET: u16 = 0x06;
pub const AZOFFSET: u16 = 0x07;
pub const GXOFFSET: u16 = 0x08;
pub const GYOFFSET: u16 = 0x09;
pub const GZOFFSET: u16 = 0x0A;
pub const HXOFFSET: u16 = 0x0B;
pub const HYOFFSET: u16 = 0x0C;
pub const HZOFFSET: u16 = 0x0D;

// Digital port configuration
pub const D0MODE: u16 = 0x0E;
pub const D1MODE: u16 = 0x0F;
pub const D2MODE: u16 = 0x10;
pub const D3MODE: u16 = 0x11;

// PWM configuration
pub const D0PWMH: u16 = 0x12;
pub const D1PWMH: u16 = 0x13;
pub const D2PWMH: u16 = 0x14;
pub const D3PWMH: u16 = 0x15;
pub const D0PWMT: u16 = 0x16;
pub const D1PWMT: u16 = 0x17;
pub const D2PWMT: u16 = 0x18;
pub const D3PWMT: u16 = 0x19;

// Device configuration
pub const IICADDR: u16 = 0x1A;
pub const LEDOFF: u16 = 0x1B;
pub const BANDWIDTH: u16 = 0x1F;
pub const GYRORANGE: u16 = 0x20;
pub const ACCRANGE: u16 = 0x21;
pub const SLEEP: u16 = 0x22;
pub const ORIENT: u16 = 0x23;
pub const AXIS6: u16 = 0x24;

// Timestamp registers
pub const YYMM: u16 = 0x30;
pub const DDHH: u16 = 0x31;
pub const MMSS: u16 = 0x32;
pub const MS: u16 = 0x33;

// Primary sensor data registers
pub const AX: u16 = 0x34;    // Accelerometer X
pub const AY: u16 = 0x35;    // Accelerometer Y
pub const AZ: u16 = 0x36;    // Accelerometer Z
pub const GX: u16 = 0x37;    // Gyroscope X
pub const GY: u16 = 0x38;    // Gyroscope Y
pub const GZ: u16 = 0x39;    // Gyroscope Z
pub const HX: u16 = 0x3A;    // Magnetometer X
pub const HY: u16 = 0x3B;    // Magnetometer Y
pub const HZ: u16 = 0x3C;    // Magnetometer Z
pub const ROLL: u16 = 0x3D;  // Roll angle
pub const PITCH: u16 = 0x3E; // Pitch angle
pub const YAW: u16 = 0x3F;   // Yaw angle
pub const TEMP: u16 = 0x40;  // Temperature

// High precision angle registers (for WT905x series)
pub const LROLL: u16 = 0x3D;
pub const HROLL: u16 = 0x3E;
pub const LPITCH: u16 = 0x3F;
pub const HPITCH: u16 = 0x40;
pub const LYAW: u16 = 0x41;
pub const HYAW: u16 = 0x42;
pub const TEMP905X: u16 = 0x43;

// Digital port status
pub const D0STATUS: u16 = 0x41;
pub const D1STATUS: u16 = 0x42;
pub const D2STATUS: u16 = 0x43;
pub const D3STATUS: u16 = 0x44;

// Pressure and GPS registers
pub const PRESSUREL: u16 = 0x45;
pub const PRESSUREH: u16 = 0x46;
pub const HEIGHTL: u16 = 0x47;
pub const HEIGHTH: u16 = 0x48;
pub const LONL: u16 = 0x49;
pub const LONH: u16 = 0x4A;
pub const LATL: u16 = 0x4B;
pub const LATH: u16 = 0x4C;

// Quaternion registers
pub const Q0: u16 = 0x51;
pub const Q1: u16 = 0x52;
pub const Q2: u16 = 0x53;
pub const Q3: u16 = 0x54;

// Register size definition
pub const REGSIZE: usize = 0x90;

// Modbus function codes
pub const FUNC_READ: u8 = 0x03;
pub const FUNC_WRITE: u8 = 0x06;

// Output data packet headers
pub const WIT_TIME: u8 = 0x50;
pub const WIT_ACC: u8 = 0x51;
pub const WIT_GYRO: u8 = 0x52;
pub const WIT_ANGLE: u8 = 0x53;
pub const WIT_MAGNETIC: u8 = 0x54;
pub const WIT_DPORT: u8 = 0x55;
pub const WIT_PRESS: u8 = 0x56;
pub const WIT_GPS: u8 = 0x57;
pub const WIT_VELOCITY: u8 = 0x58;
pub const WIT_QUATER: u8 = 0x59;
pub const WIT_GSA: u8 = 0x5A;
pub const WIT_REGVALUE: u8 = 0x5F;

// Calibration modes
pub const NORMAL: u16 = 0x00;
pub const CALGYROACC: u16 = 0x01;
pub const CALMAG: u16 = 0x02;
pub const CALALTITUDE: u16 = 0x03;
pub const CALANGLEZ: u16 = 0x04;

// Baud rate constants
pub const WIT_BAUD_4800: u16 = 1;
pub const WIT_BAUD_9600: u16 = 2;
pub const WIT_BAUD_19200: u16 = 3;
pub const WIT_BAUD_38400: u16 = 4;
pub const WIT_BAUD_57600: u16 = 5;
pub const WIT_BAUD_115200: u16 = 6;
pub const WIT_BAUD_230400: u16 = 7;
pub const WIT_BAUD_460800: u16 = 8;
pub const WIT_BAUD_921600: u16 = 9;

// Output rate constants
pub const RRATE_NONE: u16 = 0x0D;
pub const RRATE_02HZ: u16 = 0x01;
pub const RRATE_05HZ: u16 = 0x02;
pub const RRATE_1HZ: u16 = 0x03;
pub const RRATE_2HZ: u16 = 0x04;
pub const RRATE_5HZ: u16 = 0x05;
pub const RRATE_10HZ: u16 = 0x06;
pub const RRATE_20HZ: u16 = 0x07;
pub const RRATE_50HZ: u16 = 0x08;
pub const RRATE_100HZ: u16 = 0x09;
pub const RRATE_125HZ: u16 = 0x0A;
pub const RRATE_200HZ: u16 = 0x0B;
pub const RRATE_ONCE: u16 = 0x0C;
