use crate::{Exception, ReadGet};
use std::io::prelude::*;

use crate::helpers::*;

/// A command sent from the client(Master) to the server(Slave)
/// 
#[derive(Clone, Debug)]
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
    },

    /// Function code 0x05
    WriteCoil {
        coil: u16,
        state: bool
    },

    /// Function code 0x06
    WriteHolding {
        address: u16,
        value: u16
    },

    /// Function code 0x0F
    WriteMultCoil {
        start: u16,
        count: u16,
        vals: Vec<bool>,
    },

    /// Function code 0x10
    WriteMultHolding {
        start: u16,
        count: u16,
        vals: Vec<u16>
    }
}


impl ReadGet for Command {
    fn read_get(reader: &mut impl Read) -> Result<Self, Exception> where Self: Sized {
        let mut bfr = [0];

        match reader.read(&mut bfr) {
            Ok(count) => if count < 1 { return Err(Exception::FailedRead) },
            Err(e)    => return Err(Exception::IOError(e))
        }
        let function_code = bfr[0];

        match function_code {
            // Exception code for read coils
            0x81 => Err(Exception::read_get(reader)?),

            // Exception code for read DI
            0x82 => Err(Exception::read_get(reader)?),

            // Exception code for read holding
            0x83 => Err(Exception::read_get(reader)?),

            // Exception code for read input
            0x84 => Err(Exception::read_get(reader)?),

            // Exception code for write coil
            0x85 => Err(Exception::read_get(reader)?),

            // Exception code write holding
            0x86 => Err(Exception::read_get(reader)?),

            // Exception code for write mult coil
            0x8F => Err(Exception::read_get(reader)?),

            // Exception code for write mult holding
            0x90 => Err(Exception::read_get(reader)?),

            // Read Coils
            1 => {
                let mut bfr = [0;4];
                read_bfr(reader, &mut bfr)?;

                let start = u16::from_be_bytes([bfr[0], bfr[1]]);
                let count = u16::from_be_bytes([bfr[2], bfr[3]]);

                let cmd = Self::ReadCoils { start, count };
                Ok(cmd)
            },

            // Read DI
            2 => {
                let mut bfr = [0;4];
                read_bfr(reader, &mut bfr)?;

                let start = u16::from_be_bytes([bfr[0], bfr[1]]);
                let count = u16::from_be_bytes([bfr[2], bfr[3]]);

                let cmd = Self::ReadDI { start, count };
                Ok(cmd)
            },

            // Read Holding
            3 => {
                let mut bfr = [0;4];
                read_bfr(reader, &mut bfr)?;

                let start = u16::from_be_bytes([bfr[0], bfr[1]]);
                let count = u16::from_be_bytes([bfr[2], bfr[3]]);

                let cmd = Self::ReadHolding { start, count };
                Ok(cmd)
            },

            // Read Input
            4 => {
                let mut bfr = [0;4];
                read_bfr(reader, &mut bfr)?;

                let start = u16::from_be_bytes([bfr[0], bfr[1]]);
                let count = u16::from_be_bytes([bfr[2], bfr[3]]);

                let cmd = Self::ReadInput { start, count };
                Ok(cmd)
            },

            // Write single coil
            5 => {
                let mut bfr = [0;4];
                read_bfr(reader, &mut bfr)?;

                let coil = u16::from_be_bytes([bfr[0], bfr[1]]);
                let state = u16::from_be_bytes([bfr[2], bfr[3]]);

                let state = if state == 0xff00 {
                    true
                } else if state == 0x0000 {
                    false
                } else {
                    return Err(Exception::FailedRead)
                };

                let cmd = Self::WriteCoil { coil, state };
                Ok(cmd)
            },

            // Write single holding
            6 => {
                let mut bfr = [0;4];
                read_bfr(reader, &mut bfr)?;

                let address = u16::from_be_bytes([bfr[0], bfr[1]]);
                let value = u16::from_be_bytes([bfr[2], bfr[3]]);

                let cmd = Self::WriteHolding {address, value};
                Ok(cmd)
            },

            // Write multiple coils
            15 => {
                let mut bfr = [0;5];
                read_bfr(reader, &mut bfr)?;

                let start = u16::from_be_bytes([bfr[0], bfr[1]]);
                let count = u16::from_be_bytes([bfr[2], bfr[3]]);
                let byte_count = bfr[4] as usize;

                let mut bfr = vec![0;byte_count];
                match reader.read(&mut bfr) {
                    Ok(count) => if count < byte_count { return Err(Exception::FailedRead) },
                    Err(e)    => return Err(Exception::IOError(e))
                }

                let vals = bytes_to_bools(&bfr);

                let cmd = Self::WriteMultCoil{start, count, vals};

                Ok(cmd)
            },

            // Write multiple holding
            16 => {
                let mut bfr = [0;5];
                read_bfr(reader, &mut bfr)?;

                let start = u16::from_be_bytes([bfr[0], bfr[1]]);
                let count = u16::from_be_bytes([bfr[2], bfr[3]]);
                let byte_count = bfr[4] as usize;

                let mut bfr = vec![0;byte_count];
                read_bfr(reader, &mut bfr)?;

                let mut vals = Vec::with_capacity(byte_count as usize / 2);
                for bytes in bfr.windows(2).step_by(2) {
                    let val = u16::from_be_bytes([bytes[0], bytes[1]]);
                    vals.push(val)
                }

                let cmd = Self::WriteMultHolding { start, count, vals };

                Ok(cmd)
            }

            _ => Err(Exception::FailedRead)
        }

    }
}


impl Command {
    /// Gets the function code of the associated command
    /// 
    pub fn function_code(&self) -> u8 {
        match self {
            Self::ReadCoils{..}        => 1,
            Self::ReadDI{..}           => 2,
            Self::ReadHolding{..}      => 3,
            Self::ReadInput{..}        => 4,
            Self::WriteCoil{..}        => 5,
            Self::WriteHolding{..}     => 6,
            Self::WriteMultCoil{..}    => 15,
            Self::WriteMultHolding{..} => 16,
        }
    }


    /// Gives the size of the command in bytes
    /// 
    pub fn size(&self) -> u16 {
        match self {
            Self::ReadCoils{..} => 5,
            Self::ReadDI{..} => 5,
            Self::ReadHolding{..} => 5,
            Self::ReadInput{..} => 5,
            Self::WriteCoil{..} => 5,
            Self::WriteHolding{..} => 5,
            Self::WriteMultCoil {vals, ..} => {
                // Function code + start + count + byte count
                let mut count = 6;

                if vals.len() % 8 == 0 {
                    count += vals.len() / 8;
                } else {
                    count +=  vals.len() / 8 + 1;
                }

                count as u16
            },
            Self::WriteMultHolding {vals, ..} => {
                // Function code + start + count + byte count
                let mut count = 6;
                count += vals.len() as u16 * 2;
                count
            }
        }
    }
}


impl Into<Vec<u8>> for &Command {
    fn into(self) -> Vec<u8> {
        let size = self.size() as usize;
        let mut v = Vec::with_capacity(size);
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
            Command::WriteCoil {coil, state} => {
                v.extend_from_slice(&coil.to_be_bytes());
                if *state {
                    v.push(0xFF);
                    v.push(0x00);
                } else {
                    v.push(0x00);
                    v.push(0x00);
                }
            }
            Command::WriteHolding {address, value} => {
                v.extend_from_slice(&address.to_be_bytes());
                v.extend_from_slice(&value.to_be_bytes());
            },
            Command::WriteMultCoil {start, count, vals} => {
                v.extend_from_slice(&start.to_be_bytes());
                v.extend_from_slice(&count.to_be_bytes());

                // Byte count
                if vals.len() % 8 == 0 {
                    v.push(vals.len() as u8 / 8)
                } else {
                    v.push(vals.len() as u8 / 8 + 1);
                }

                let vals_bytes = bools_to_bytes(&vals);
                v.extend_from_slice(&vals_bytes);
            },
            Command::WriteMultHolding {start, count, vals} => {
                v.extend_from_slice(&start.to_be_bytes());
                v.extend_from_slice(&count.to_be_bytes());

                let byte_count = vals.len() as u8 * 2;
                v.push(byte_count);
            }
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
#[derive(Clone, Debug)]
pub enum Response {
    /// Function code 0x01
    ReadCoils{
        status: Vec<bool>
    },

    /// Function code 0x02
    ReadDI{
        status: Vec<bool>
    },

    /// Function code 0x03
    ReadHolding{
        status: Vec<u16>
    },

    /// Function code 0x04
    ReadInput{
        status: Vec<u16>
    },

    /// Function code 0x05
    WriteCoil{
        coil: u16,
        state: bool
    },

    /// Function code 0x06
    WriteHolding{
        address: u16,
        value: u16
    },

    /// Function code 15
    WriteMultCoil{
        /// Starting address
        start: u16,

        /// Numbers of coils forced
        count: u16
    },

    /// Function code 16
    WriteMultHolding{
        /// Starting address
        start: u16,

        /// Number of registers written
        count: u16,
    }
}


impl Response {
    pub fn read_coils(coils: &[bool]) -> Self {
        let mut status = Vec::with_capacity(coils.len());
        status.extend_from_slice(coils);

        Self::ReadCoils { status }
    }

    pub fn read_di(di: &[bool]) -> Self {
        let mut status = Vec::with_capacity(di.len());
        status.extend_from_slice(di);
        
        Self::ReadDI { status }
    }

    pub fn read_holding(addresses: &[u16]) -> Self {
        let mut status = Vec::with_capacity(addresses.len());
        status.extend_from_slice(addresses);
        
        Self::ReadHolding { status }
    }

    pub fn read_input(addresses: &[u16]) -> Self {
        let mut status = Vec::with_capacity(addresses.len());
        status.extend_from_slice(addresses);
        
        Self::ReadInput { status }
    }

    pub fn write_coil(coil: u16, state: bool) -> Self {
        Self::WriteCoil { coil, state }
    }

    pub fn write_holding(address: u16, value: u16) -> Self {
        Self::WriteHolding { address, value }
    }

    pub fn write_mult_coil(start: u16, count: u16) -> Self {
        Self::WriteMultCoil { start, count }
    }

    pub fn write_mult_holding(start: u16, count: u16) -> Self {
        Self::WriteMultHolding { start, count }
    }


    /// Gives the size of the response in bytes
    /// 
    pub fn size(&self) -> u16 {
        match self {
            Response::ReadCoils{status} => {
                // Function code + byte count
                let mut size = 2;

                if status.len() % 8 == 0 {
                    size += status.len() as u16 / 8
                } else {
                    size += status.len() as u16 / 8 + 1
                }

                size
            },
            Response::ReadDI{status} => {
                // Function code + byte count
                let mut size = 2;

                if status.len() % 8 == 0 {
                    size += status.len() as u16 / 8
                } else {
                    size += status.len() as u16 / 8 + 1
                }

                size
            },
            Response::ReadHolding{ status } => {
                // Function code + byte count
                let mut size = 2;

                size += status.len() as u16 * 2;
                size
            },
            Response::ReadInput{ status } => {
                // Function code + byte count
                let mut size = 2;

                size += status.len() as u16 * 2;
                size
            },
            Response::WriteCoil{..} => 5, // Function code + addr + value
            Response::WriteHolding{..} => 5, // Function code + addr + value
            Response::WriteMultCoil{..} => 5, // Function code + start + count
            Response::WriteMultHolding{..} => 5 // Function code + start + count
        }
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
            Response::ReadCoils { status } => {
                let byte_count = if status.len() % 8 == 0 {
                    status.len() as u8 / 8
                } else {
                    status.len() as u8 / 8 + 1
                };

                let mut v = Vec::with_capacity(self.size() as usize);

                // Function code
                v.push(1);

                //Byte count
                v.push(byte_count);

                //Coils status
                v.extend_from_slice(&bools_to_bytes(&status));

                v
            },

            Response::ReadDI { status } => {
                let byte_count = if status.len() % 8 == 0 {
                    status.len() as u8 / 8
                } else {
                    status.len() as u8 / 8 + 1
                };

                let mut v = Vec::with_capacity(self.size() as usize);

                // Function code
                v.push(2);

                //Byte count
                v.push(byte_count);

                //Coils status
                v.extend_from_slice(&bools_to_bytes(&status));

                v
            },

            Response::ReadHolding { status } => {
                let byte_count = status.len() as u8 * 2;

                let mut v = Vec::with_capacity(self.size() as usize);

                // Function code
                v.push(3);

                //Byte count
                v.push(byte_count);

                //Coils status
                for word in status {
                    let bytes = word.to_be_bytes();
                    v.extend_from_slice(&bytes);
                }

                v
            },

            Response::ReadInput { status } => {
                let byte_count = status.len() as u8 * 2;

                let mut v = Vec::with_capacity(self.size() as usize);

                // Function code
                v.push(4);

                //Byte count
                v.push(byte_count);

                //Coils status
                for word in status {
                    let bytes = word.to_be_bytes();
                    v.extend_from_slice(&bytes);
                }

                v
            },

            Response::WriteCoil { coil, state } => {
                let mut v = Vec::with_capacity(self.size() as usize);

                // Function code
                v.push(5);

                v.extend_from_slice(&coil.to_be_bytes());

                if *state {
                    v.push(0xff);
                    v.push(0x00);
                } else {
                    v.push(0x00);
                    v.push(0x00);
                }

                v
            },

            Response::WriteHolding { address, value } => {
                let mut v = Vec::with_capacity(self.size() as usize);

                // Function code
                v.push(6);

                v.extend_from_slice(&address.to_be_bytes());
                v.extend_from_slice(&value.to_be_bytes());

                v
            },

            Response::WriteMultCoil { start, count } => {
                let mut v = Vec::with_capacity(self.size() as usize);

                // Function code
                v.push(15);

                v.extend_from_slice(&start.to_be_bytes());
                v.extend_from_slice(&count.to_be_bytes());

                v
            },

            Response::WriteMultHolding { start, count } => {
                let mut v = Vec::with_capacity(self.size() as usize);

                // Function code
                v.push(16);

                v.extend_from_slice(&start.to_be_bytes());
                v.extend_from_slice(&count.to_be_bytes());

                v
            }
        }
    }
}


impl ReadGet for Response {
    fn read_get(reader: &mut impl Read) -> Result<Self, Exception> where Self: Sized {
        let mut bfr = [0];
        read_bfr(reader, &mut bfr)?;

        let function_code = bfr[0];

        match function_code {
            //Read coils
            1 => {
                let mut bfr = [0];
                read_bfr(reader, &mut bfr)?;

                let byte_count = bfr[0];

                let mut bfr = vec![0; byte_count as usize];
                read_bfr(reader, &mut bfr)?;

                let status = bytes_to_bools(&bfr);
                
                let response = Self::ReadCoils { status };
                Ok(response)
            },

            //Read DI
            2 => {
                let mut bfr = [0];
                read_bfr(reader, &mut bfr)?;

                let byte_count = bfr[0];

                let mut bfr = vec![0; byte_count as usize];
                read_bfr(reader, &mut bfr)?;

                let status = bytes_to_bools(&bfr);
                
                let response = Self::ReadDI { status };
                Ok(response)
            },

            //Read Holding
            3 => {
                let mut bfr = [0];
                read_bfr(reader, &mut bfr)?;

                let byte_count = bfr[0];

                let mut bfr = vec![0; byte_count as usize * 2];
                read_bfr(reader, &mut bfr)?;

                let mut status = Vec::with_capacity(byte_count as usize / 2);
                for bytes in bfr.windows(2).step_by(2) {
                    let word = u16::from_be_bytes([bytes[0], bytes[1]]);
                    status.push(word);
                }
                
                let response = Self::ReadHolding { status };
                Ok(response)
            },

            //Read Input
            4 => {
                let mut bfr = [0];
                read_bfr(reader, &mut bfr)?;

                let byte_count = bfr[0];

                let mut bfr = vec![0; byte_count as usize * 2];
                read_bfr(reader, &mut bfr)?;

                let mut status = Vec::with_capacity(byte_count as usize / 2);
                for bytes in bfr.windows(2).step_by(2) {
                    let word = u16::from_be_bytes([bytes[0], bytes[1]]);
                    status.push(word);
                }
                
                let response = Self::ReadInput { status };
                Ok(response)
            },

            //Write coil
            5 => {
                let mut bfr = [0; 4];
                read_bfr(reader, &mut bfr)?;

                let coil = u16::from_be_bytes([bfr[0], bfr[1]]);
                let state = u16::from_be_bytes([bfr[2], bfr[3]]);

                let state = if state == 0xFF00 {
                    true
                } else if state == 0x0000 {
                    false
                } else {
                    return Err(Exception::FailedRead)
                };

                let cmd = Self::WriteCoil { coil, state };
                Ok(cmd)
            },

            //Write holding
            6 => {
                let mut bfr = [0; 4];
                read_bfr(reader, &mut bfr)?;

                let address = u16::from_be_bytes([bfr[0], bfr[1]]);
                let value = u16::from_be_bytes([bfr[2], bfr[3]]);

                let cmd = Self::WriteHolding { address, value };
                Ok(cmd)
            },

            //Write mult coils
            15 => {
                let mut bfr = [0; 4];
                read_bfr(reader, &mut bfr)?;

                let start = u16::from_be_bytes([bfr[0], bfr[1]]);
                let count = u16::from_be_bytes([bfr[2], bfr[3]]);

                let cmd = Self::WriteMultCoil { start, count };
                Ok(cmd)
            }

            //Write mult holding
            16 => {
                let mut bfr = [0; 4];
                read_bfr(reader, &mut bfr)?;

                let start = u16::from_be_bytes([bfr[0], bfr[1]]);
                let count = u16::from_be_bytes([bfr[2], bfr[3]]);

                let cmd = Self::WriteMultHolding { start, count };
                Ok(cmd)
            }

            _ => Err(Exception::FailedRead)
        }
    }
}