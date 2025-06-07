# WitMotion WT901C485 Modbus Sensor Interface - Rust Implementation

A Rust reimplementation of the WitMotion Modbus sensor interface, providing communication with WitMotion IMU sensors over RS485 serial connections.

## Features

- **Modbus RTU Protocol**: Implements the Modbus RTU protocol for communication with WitMotion sensors using the RS485 interface.
- **Sensor Data Retrieval**: Fetches sensor data such as acceleration, gyroscope, and magnetometer readings.
- **Windows and Linux Support**: Compatible with both Windows and Linux operating systems (it might be compatible with macOS as well, but this has not been tested).

## Usage

 If you just want to give this a try, you can run the example provided as "test-reader.rs". To build it, just run:
```bash
cargo build --release
```

Then, to run the example, use:
```bash
./target/release/test-reader --help
```

Or, on Windows:
```bash
.\target\release\test-reader.exe --help
```

## Cross-Platform Compatibility
This implementation is designed to be cross-platform, allowing it to run on both Windows and Linux systems. It uses the `serial2` crate for serial communication, which has worked way better than the `serialport` crate in my experience. It has been tested and build successfully on Linux targetting both platforms, using both mingw-w64 and MSVC toolchains. You can try this yourself by running:
```bash
cargo build --target x86_64-pc-windows-gnu --release
```

Or, to target the MSVC toolchain, you can use the awesome cargo-xwin:
```bash
cargo add --locked cargo-xwin
cargo xwin build --target x86_64-pc-windows-msvc --release
```

## Command line arguments
The example program accepts several command line arguments to configure the serial port and the sensor address. You can see the available options by running:
```bash
test-reader --help

WitMotion Modbus sensor reader

Usage: test-reader [OPTIONS] --device <DEVICE>

Options:
  -d, --device <DEVICE>        Serial device path (e.g., /dev/ttyUSB0)
  -a, --address <ADDRESS>      Modbus slave address (default: 0xFF for broadcast) Accepts hex format (0x50) or decimal format (80) [default: 255]
  -i, --interval <INTERVAL>    Polling interval in milliseconds [default: 500]
  -b, --baud-rate <BAUD_RATE>  Skip auto-scan and use specified baud rate
  -v, --verbose                Enable verbose output
  -h, --help                   Print help
```


## Limitations
Although most of this could be modified to work even on embedded systems, this implementation has not been really been made to do that. It might get modified to allow for this in the future, but there is not an ETA for that.