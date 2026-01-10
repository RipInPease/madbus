use crate::serial_port::SerialPort;


/// The master in the modbus line
/// 
pub struct Master {
    port: SerialPort,
}