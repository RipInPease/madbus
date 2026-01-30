use crate::ReadGet;
use std::io::prelude::*;

use crate::helpers::*;

/// A command sent from the client(Master) to the server(Slave)
pub enum Command {
    /// Function code 0x01
    ReadCoils{
        start: u16,
        count: u16,
    },

    /// Function code 0x02
    ReadDI{
        start: u16,
        count: u16,
    },

    /// Function code 0x03
    ReadHolding{
        start: u16,
        count: u16,
    },

    /// Function code 0x04
    ReadInput{
        start: u16,
        count: u16,
    }
}


impl ReadGet for Command {
    fn read_get(reader: &mut impl Read) -> Option<Self> where Self: Sized {
        let mut bfr = [0];

        match reader.read(&mut bfr) {
            Ok(count) => if count < 1 { return None },
            Err(_)    => return None
        }
        let function_code = bfr[0];

        match function_code {
            // Read Coils
            1 => {
                let mut bfr = [0;4];
                match reader.read(&mut bfr) {
                    Ok(count) => if count < 4 { return None },
                    Err(_)    => return None
                }

                let start = u16::from_be_bytes([bfr[0], bfr[1]]);
                let count = u16::from_be_bytes([bfr[2], bfr[3]]);

                let cmd = Self::ReadCoils { start, count };
                Some(cmd)
            },

            // Read DI
            2 => {
                let mut bfr = [0;4];
                match reader.read(&mut bfr) {
                    Ok(count) => if count < 4 { return None },
                    Err(_)    => return None
                }

                let start = u16::from_be_bytes([bfr[0], bfr[1]]);
                let count = u16::from_be_bytes([bfr[2], bfr[3]]);

                let cmd = Self::ReadDI { start, count };
                Some(cmd)
            },

            // Read Holding
            3 => {
                let mut bfr = [0;4];
                match reader.read(&mut bfr) {
                    Ok(count) => if count < 4 { return None },
                    Err(_)    => return None
                }

                let start = u16::from_be_bytes([bfr[0], bfr[1]]);
                let count = u16::from_be_bytes([bfr[2], bfr[3]]);

                let cmd = Self::ReadHolding { start, count };
                Some(cmd)
            },

            // Read Input
            5 => {
                let mut bfr = [0;4];
                match reader.read(&mut bfr) {
                    Ok(count) => if count < 4 { return None },
                    Err(_)    => return None
                }

                let start = u16::from_be_bytes([bfr[0], bfr[1]]);
                let count = u16::from_be_bytes([bfr[2], bfr[3]]);

                let cmd = Self::ReadInput { start, count };
                Some(cmd)
            },

            _ => None
        }

    }
}


impl Command {
    /// Gets the function code of the associated command
    /// 
    pub fn function_code(&self) -> u8 {
        match self {
            Self::ReadCoils{..}   => 1,
            Self::ReadDI{..}      => 2,
            Self::ReadHolding{..} => 3,
            Self::ReadInput{..}   => 4,
        }
    }
}


impl Into<Vec<u8>> for &Command {
    fn into(self) -> Vec<u8> {
        let mut v = Vec::with_capacity(10);
        v.push(self.function_code());

        match self {
            Command::ReadCoils{start, count}   => {
                v.extend_from_slice(&start.to_be_bytes());
                v.extend_from_slice(&count.to_be_bytes());
            },
            Command::ReadDI{start, count}   => {
                v.extend_from_slice(&start.to_be_bytes());
                v.extend_from_slice(&count.to_be_bytes());
            },
            Command::ReadHolding{start, count}   => {
                v.extend_from_slice(&start.to_be_bytes());
                v.extend_from_slice(&count.to_be_bytes());
            },
            Command::ReadInput{start, count}   => {
                v.extend_from_slice(&start.to_be_bytes());
                v.extend_from_slice(&count.to_be_bytes());
            },
        } 

        v
    }
}


impl Into<Vec<u8>> for Command {
    fn into(self) -> Vec<u8> {
        (&self).into()
    }
}



/// Response a server(Slave) sends in response to a command
/// 
pub enum Response {
    /// Function code 0x01
    ReadCoils{
        byte_count: u8,
        status: Vec<bool>
    },

    /// Function code 0x02
    ReadDI{
        byte_count: u8,
        status: Vec<bool>
    },

    /// Function code 0x03
    ReadHolding{
        byte_count: u8,
        status: Vec<u16>
    },

    /// Function code 0x04
    ReadInput{
        byte_count: u8,
        status: Vec<u16>
    },
}


impl Response {
    pub fn read_coils(coils: &[bool]) -> Self {
        let byte_count = if coils.len() % 8 > 0 {
            coils.len() as u8 / 8 + 1
        } else {
            coils.len() as u8 / 8
        };

        let mut status = Vec::with_capacity(coils.len());
        status.clone_from_slice(coils);

        Self::ReadCoils { byte_count, status }
    }

    pub fn read_di(di: &[bool]) -> Self {
        let byte_count = if di.len() % 8 > 0 {
            di.len() as u8 / 8 + 1
        } else {
            di.len() as u8 / 8
        };

        let mut status = Vec::with_capacity(di.len());
        status.clone_from_slice(di);
        
        Self::ReadDI { byte_count, status }
    }

    pub fn read_holding(addresses: &[u16]) -> Self {
        let byte_count = addresses.len() as u8 * 2;

        let mut status = Vec::with_capacity(addresses.len());
        status.clone_from_slice(addresses);
        
        Self::ReadHolding { byte_count, status }
    }

    pub fn read_input(addresses: &[u16]) -> Self {
        let byte_count = addresses.len() as u8 * 2;

        let mut status = Vec::with_capacity(addresses.len());
        status.clone_from_slice(addresses);
        
        Self::ReadInput { byte_count, status }
    }
}


impl Into<Vec<u8>> for Response {
    fn into(self) -> Vec<u8> {
        (&self).into()
    }
}


impl Into<Vec<u8>> for &Response {
    fn into(self) -> Vec<u8> {
        match self {
            Response::ReadCoils { byte_count, status } => {
                //The byte count + the byte count itself + function code
                let mut v = Vec::with_capacity(*byte_count as usize + 2);

                // Function code
                v.push(1);

                //Byte count
                v.push(*byte_count);

                //Coils status
                v.extend_from_slice(&bools_to_bytes(&status));

                v
            },

            Response::ReadDI { byte_count, status } => {
                //The byte count + the byte count itself + function code
                let mut v = Vec::with_capacity(*byte_count as usize + 2);

                // Function code
                v.push(2);

                //Byte count
                v.push(*byte_count);

                //Coils status
                v.extend_from_slice(&bools_to_bytes(&status));

                v
            },

            Response::ReadHolding { byte_count, status } => {
                //The byte count + the byte count itself + function code
                let mut v = Vec::with_capacity(*byte_count as usize + 2);

                // Function code
                v.push(3);

                //Byte count
                v.push(*byte_count);

                //Coils status
                for word in status {
                    let bytes = word.to_be_bytes();
                    v.extend_from_slice(&bytes);
                }

                v
            },

            Response::ReadInput { byte_count, status } => {
                //The byte count + the byte count itself + function code
                let mut v = Vec::with_capacity(*byte_count as usize + 2);

                // Function code
                v.push(3);

                //Byte count
                v.push(*byte_count);

                //Coils status
                for word in status {
                    let bytes = word.to_be_bytes();
                    v.extend_from_slice(&bytes);
                }

                v
            }
        }
    }
}


impl ReadGet for Response {
    fn read_get(reader: &mut impl Read) -> Option<Self> where Self: Sized {
        let mut bfr = [0];

        match reader.read(&mut bfr) {
            Ok(count) => if count < 1 { return None },
            Err(_)    => return None,
        }
        let function_code = bfr[0];

        match function_code {
            //Read coils
            1 => {
                let mut bfr = [0];
                match reader.read(&mut bfr) {
                    Ok(count) => if count < 1 { return None },
                    Err(_)    => return None,
                }

                let byte_count = bfr[0];

                let mut bfr = vec![0; byte_count as usize];
                match reader.read(&mut bfr) {
                    Ok(count) => if count < byte_count as usize { return None },
                    Err(_)    => return None
                }

                let status = bytes_to_bools(&bfr);
                
                let response = Self::ReadCoils { byte_count, status };
                Some(response)
            },

            //Read DI
            2 => {
                let mut bfr = [0];
                match reader.read(&mut bfr) {
                    Ok(count) => if count < 1 { return None },
                    Err(_)    => return None,
                }

                let byte_count = bfr[0];

                let mut bfr = vec![0; byte_count as usize];
                match reader.read(&mut bfr) {
                    Ok(count) => if count < byte_count as usize { return None },
                    Err(_)    => return None
                }

                let status = bytes_to_bools(&bfr);
                
                let response = Self::ReadDI { byte_count, status };
                Some(response)
            },

            //Read Holding
            3 => {
                let mut bfr = [0];
                match reader.read(&mut bfr) {
                    Ok(count) => if count < 1 { return None },
                    Err(_)    => return None,
                }

                let byte_count = bfr[0];

                let mut bfr = vec![0; byte_count as usize * 2];
                match reader.read(&mut bfr) {
                    Ok(count) => if count < byte_count as usize { return None },
                    Err(_)    => return None
                }

                let mut status = Vec::with_capacity(byte_count as usize / 2);
                for bytes in bfr.windows(2).step_by(2) {
                    let word = u16::from_be_bytes([bytes[0], bytes[1]]);
                    status.push(word);
                }
                
                let response = Self::ReadHolding { byte_count, status };
                Some(response)
            },

            //Read Input
            4 => {
                let mut bfr = [0];
                match reader.read(&mut bfr) {
                    Ok(count) => if count < 1 { return None },
                    Err(_)    => return None,
                }

                let byte_count = bfr[0];

                let mut bfr = vec![0; byte_count as usize * 2];
                match reader.read(&mut bfr) {
                    Ok(count) => if count < byte_count as usize { return None },
                    Err(_)    => return None
                }

                let mut status = Vec::with_capacity(byte_count as usize / 2);
                for bytes in bfr.windows(2).step_by(2) {
                    let word = u16::from_be_bytes([bytes[0], bytes[1]]);
                    status.push(word);
                }
                
                let response = Self::ReadInput { byte_count, status };
                Some(response)
            },

            _ => None
        }
    }
}