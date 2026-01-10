use std::time::Duration;

/// Used to a build a serial port
/// 
pub struct SerialPortBuilder {
    file        : String,
    baud        : usize,
    parity      : Parity,
    stop_bits   : StopBits,
    timeout     : Duration,
}


impl SerialPortBuilder {
    
    /// Creates a new serial port with default settings:
    /// 
    /// Baudrate: 19200
    /// 
    /// Parity: Even
    /// 
    /// Stopbits: 1
    /// 
    /// Databits: 8
    /// 
    /// Timeout: 3s
    /// 
    pub fn new() -> Self {
        Self { 
            file        : "/dev/sttyS0".to_string(),
            baud        : 19200, 
            parity      : Parity::Even,
            stop_bits   : StopBits::One,
            timeout     : Duration::from_secs(3),
        }
    }
}


/// Option for serial port
/// 
pub enum Parity {
    Even,
    Odd
}


/// Option for serial port
/// 
pub enum StopBits {
    One = 1,
    Two = 2,
}


/// Option for serial port
/// 
pub enum DataBits {
    Five    = 5,
    Six     = 6,
    Seven   = 7,
    Eight   = 8,
}