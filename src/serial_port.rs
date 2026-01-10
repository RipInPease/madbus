use std::time::Duration;
use std::fs::{File, OpenOptions};
use std::io::{Write, Read, Result as IOResult};


/// Used to a build a serial port
/// 
pub struct SerialPortBuilder {
    file        : String,
    baud        : usize,
    parity      : Parity,
    stop_bits   : StopBits,
    data_bits   : DataBits,
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
            data_bits   : DataBits::Eight,
            timeout     : Duration::from_secs(3),
        }
    }


    /// Change which file to link the serial port to
    /// 
    pub fn set_file(&mut self, file: &str) -> &mut Self {
        self.file = file.to_string();
        self
    }


    /// Change the baud rate of the serial port
    /// 
    pub fn set_baud(&mut self, baud: usize) -> &mut Self {
        self.baud = baud;
        self
    }


    /// Change the parity of the serial port
    /// 
    pub fn set_parity(&mut self, parity: Parity) -> &mut Self {
        self.parity = parity;
        self
    }


    /// Change the stop bits of the serial port
    /// 
    pub fn set_stop_bits(&mut self, stop_bits: StopBits) -> &mut Self {
        self.stop_bits = stop_bits;
        self
    }


    /// Change the databits of the serial port
    /// 
    pub fn set_data_bits(&mut self, data_bits: DataBits) -> &mut Self {
        self.data_bits = data_bits;
        self
    }


    /// Change the timeout of the serial port
    /// 
    pub fn set_timeout(&mut self, timeout: Duration) -> &mut Self {
        self.timeout = timeout;
        self
    }


    /// Opens the serial port with given settings. Returns SerialPort
    /// 
    pub fn open(self) -> IOResult<SerialPort> {
        use std::process::Command;

        let mut port_settings = Command::new("stty");
        port_settings.args([
            "-F", &self.file, 
            "raw", 
            "min", "0", 
            &self.baud.to_string(),
            "time", &(self.timeout.as_millis() / 100).to_string()]);

        match self.data_bits {
            DataBits::Five  => port_settings.arg("cs5"),
            DataBits::Six   => port_settings.arg("cs6"),
            DataBits::Seven => port_settings.arg("cs7"),
            DataBits::Eight => port_settings.arg("cs8"),
        };

        match self.parity {
            Parity::Even => port_settings.args(["parenb", "-parodd"]),
            Parity::Odd  => port_settings.args(["parenb", "parodd"]),
            Parity::None => port_settings.arg("-parenb"),
        };

        match self.stop_bits {
            StopBits::One => port_settings.arg("-cstopb"),
            StopBits::Two => port_settings.arg("cstopb"),
        };

        port_settings.status()?;

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.file)?;

        Ok(SerialPort { file })
    }
}


/// Option for serial port
/// 
pub enum Parity {
    Even,
    Odd,
    None
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


/// Represents a serial port. Use SerialPortBuilder to create a serialport.
/// 
pub struct SerialPort {
    file: File,
}


impl Write for SerialPort {
    fn write (&mut self, bfr: &[u8]) -> IOResult<usize> {
        self.file.write(bfr)
    }

    fn flush(&mut self) -> IOResult<()> {
        self.file.flush()
    }
}


impl Read for SerialPort {
    fn read(&mut self, bfr: &mut[u8]) -> IOResult<usize> {
        self.file.read(bfr)
    }
}