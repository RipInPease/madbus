use std::io::{Read, Write, Result as IOResult};
use crate::{
    ReadGet, 
    crc_gen, 
    function_codes::{FunctionCode, responses::{Response, ResponseData}}, 
    serial_port::SerialPort
};


/// A slave unit in the modbus line
/// 
pub struct Slave {
    addr: u8,
    port: SerialPort,
}


impl Slave {
    /// Creates a new slave device with given serial port and slave address
    /// 
    pub fn new(port: SerialPort, addr: u8) -> Self {
        Self{ port, addr }
    }


    /// Read the port for incoming commands. 
    /// Returns None if either failed or no incoming commands
    /// 
    pub fn read_port(&mut self) -> Option<FunctionCode> {
        let mut bfr = [0];

        // Return none if there was nothing to be read or error occured
        match self.read(&mut bfr) {
            Ok(count)   => if count < 1 { return None },
            Err(_)      => return None,
        }

        // Check command was meant for this slave
        if bfr[0] != self.addr { return None }


        let cmd = FunctionCode::read_get(&mut self.port)?;

        // Check Crc
        let data: Vec<u8> = cmd.clone().into();

        let mut bfr = [0; 2];
        match self.read(&mut bfr) {
            Ok(count)   => if count < 2 { return None },
            Err(_)      => return None,
        }

        let crc_read = bfr;
        let crc_generated = crc_gen(&data);

        if crc_read != crc_generated {
            return None
        }

        Some(cmd)
    }


    /// Send a response over the serial line
    /// 
    pub fn send_resp(&mut self, data: ResponseData) -> IOResult<()> {
        let response = Response{ addr: self.addr, data };
        let mut bytes: Vec<u8> = response.into();

        let crc = crc_gen(&bytes);
        bytes.extend_from_slice(&crc);

        self.write(&bytes)?;

        Ok(())
    }
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