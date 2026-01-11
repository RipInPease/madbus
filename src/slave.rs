use std::io::{Read, Write, Result as IOResult};
use crate::serial_port::SerialPort;


/// A slave unit in the modbus line
/// 
pub struct Slave {
    addr: u8,
    port: SerialPort,
}


impl Read for Slave {
    fn read(&mut self, bfr: &mut[u8]) -> IOResult<usize> {
        self.port.read(bfr)
    }
}


impl Write for Slave {
    fn write (&mut self, bfr: &[u8]) -> IOResult<usize> {
        self.port.write(bfr)
    }

    fn flush(&mut self) -> IOResult<()> {
        self.port.flush()
    }
}