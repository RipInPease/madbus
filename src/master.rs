use std::io::{Read, Write, Result as IOResult};

use crate::{ReadGet, Command, crc_gen};
use crate::serial_port::SerialPort;
use crate::function_codes::responses::Response;


/// The master in the modbus line
/// 
pub struct Master {
    port: SerialPort,
}


impl Master {
    /// Creates a new master device with given serial port
    /// 
    pub fn new(port: SerialPort) -> Self {
        Self{ port }
    }


    /// Send a command over the serial line
    /// 
    pub fn send_cmd(&mut self, cmd: Command) -> IOResult<()> {
        let bytes: Vec<u8> = cmd.into();
        self.write(&bytes)?;
        Ok(())
    }


    /// Reads the port for incoming responses from slaves
    /// 
    pub fn read_resp(&mut self) -> Option<Response> {
        let resp = Response::read_get(self)?;
        let data: Vec<u8> = resp.clone().into();

        // Check crc
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

        Some(resp)
    }
}


impl Read for Master {
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


impl Write for Master {
    fn write (&mut self, bfr: &[u8]) -> IOResult<usize> {
        self.port.write(bfr)
    }

    fn flush(&mut self) -> IOResult<()> {
        self.port.flush()
    }
}