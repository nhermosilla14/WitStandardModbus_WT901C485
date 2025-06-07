use clap::Parser;
use std::{thread, time::Duration};
use witmotion_modbus::{WitSensor, DEFAULT_POLL_INTERVAL_MS};

/// Parse address argument that can be in hex (0x50) or decimal (80) format
fn parse_address(s: &str) -> Result<u8, String> {
    if s.starts_with("0x") || s.starts_with("0X") {
        u8::from_str_radix(&s[2..], 16)
            .map_err(|_| format!("Invalid hex address: {}", s))
    } else {
        s.parse::<u8>()
            .map_err(|_| format!("Invalid decimal address: {}", s))
    }
}

/// Command line arguments
#[derive(Parser, Debug)]
#[command(name = "test-reader")]
#[command(about = "WitMotion Modbus sensor reader")]
struct Args {
    /// Serial device path (e.g., /dev/ttyUSB0)
    #[arg(short, long)]
    device: String,

    /// Modbus slave address (default: 0xFF for broadcast)
    /// Accepts hex format (0x50) or decimal format (80)
    #[arg(short = 'a', long, default_value = "255", value_parser = parse_address)]
    address: u8,

    /// Polling interval in milliseconds
    #[arg(short = 'i', long, default_value_t = DEFAULT_POLL_INTERVAL_MS)]
    interval: u64,

    /// Skip auto-scan and use specified baud rate
    #[arg(short = 'b', long)]
    baud_rate: Option<u32>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("WitMotion Modbus Sensor Reader");
    println!("==============================");

    // Create sensor instance
    let mut sensor = WitSensor::new(
        &args.device,
        args.address,
    )?;

    // Initialize sensor
    sensor.init()?;
    println!("Sensor initialized");

    // Auto-scan or use specified baud rate
    let baud_rate = match args.baud_rate {
        Some(baud) => {
            println!("Using specified baud rate: {}", baud);
            baud
        }
        None => {
            println!("Auto-scanning for sensor...");
            sensor.auto_scan()?
        }
    };

    println!("Communication established at {} baud", baud_rate);
    println!("Starting data acquisition...");
    println!();

    // Main data acquisition loop
    loop {
        match sensor.read_sensor_data() {
            Ok(data) => {
                // Display data based on update flags
                if data.has_accelerometer_update() {
                    println!(
                        "Accelerometer: X={:.3}g, Y={:.3}g, Z={:.3}g",
                        data.accelerometer[0], data.accelerometer[1], data.accelerometer[2]
                    );
                }

                if data.has_gyroscope_update() {
                    println!(
                        "Gyroscope: X={:.3}°/s, Y={:.3}°/s, Z={:.3}°/s",
                        data.gyroscope[0], data.gyroscope[1], data.gyroscope[2]
                    );
                }

                if data.has_angle_update() {
                    println!(
                        "Angles: Roll={:.3}°, Pitch={:.3}°, Yaw={:.3}°",
                        data.angles[0], data.angles[1], data.angles[2]
                    );
                }

                if data.has_magnetometer_update() {
                    println!(
                        "Magnetometer: X={}, Y={}, Z={}",
                        data.magnetometer[0], data.magnetometer[1], data.magnetometer[2]
                    );
                }

                // Print separator if any data was displayed
                if !data.update_flags.is_empty() {
                    println!();
                }

                // Verbose mode: show all register values
                if args.verbose && !data.update_flags.is_empty() {
                    println!("Register dump:");
                    let mut regs: Vec<_> = sensor.get_all_registers().iter().collect();
                    regs.sort_by_key(|(addr, _)| *addr);
                    for (addr, value) in regs {
                        println!("  0x{:04X}: {}", addr, value);
                    }
                    println!();
                }
            }
            Err(e) => {
                eprintln!("Error reading sensor data: {}", e);
            }
        }

        // Wait before next poll
        thread::sleep(Duration::from_millis(args.interval));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_parsing() {
        // Test basic argument parsing
        let args = Args::try_parse_from(&[
            "test-reader",
            "--device", "/dev/ttyUSB0",
        ]).unwrap();
        
        assert_eq!(args.device, "/dev/ttyUSB0");
        assert_eq!(args.address, 255);
        assert_eq!(args.interval, DEFAULT_POLL_INTERVAL_MS);
    }

    #[test]
    fn test_args_with_options() {
        let args = Args::try_parse_from(&[
            "test-reader",
            "--device", "/dev/ttyUSB0",
            "--address", "50",
            "--interval", "1000",
            "--verbose",
        ]).unwrap();
        
        assert_eq!(args.device, "/dev/ttyUSB0");
        assert_eq!(args.address, 50);
        assert_eq!(args.interval, 1000);
        assert_eq!(args.verbose, true);
    }
}
