use std::io::{Read, Write, Result as IOResult};
use crate::{
    ReadGet, 
    crc_gen, 
    Command,
    function_codes::responses::{Response, ResponseData}, 
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
    pub fn read_cmd(&mut self) -> Option<Command> {
        let cmd = Command::read_get(self)?;
        

        // Crc stuff
        let mut bfr = [0;2];
        match self.read(&mut bfr) {
            Ok(count) => if count < 2 { return None},
            Err(_)    => return None,
        }

        let bytes: Vec<u8> = cmd.clone().into();
        let len = bytes.len();
        let crc_generated = crc_gen(&bytes[..len-2]);

        if bfr != crc_generated {
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
        let mut count = 0;
        let mut read_bytes: Vec<u8> = Vec::with_capacity(bfr.len());
        let mut read_bfr = [0];

        while count < bfr.len() && self.port.read(&mut read_bfr)? > 0 {
            count += 1;
            read_bytes.push(read_bfr[0]);
        }

        for i in 0..count {
            let i = i as usize;
            bfr[i] = read_bytes[i];
        }

        Ok(count)
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