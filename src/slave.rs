use crate::serial_port::SerialPort;


/// A slave unit in the modbus line
/// 
pub struct Slave {
    addr: u8,
    port: SerialPort,
}