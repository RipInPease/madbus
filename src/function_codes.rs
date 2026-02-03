use crate::ReadGet;
use std::io::prelude::*;

use crate::helpers::*;

/// A command sent from the client(Master) to the server(Slave)
/// 
#[derive(Clone, Debug)]
pub enum PduCommand {
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


impl ReadGet for PduCommand {
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
            4 => {
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

            // Write single coil
            5 => {
                let mut bfr = [0;4];
                match reader.read(&mut bfr) {
                    Ok(count) => if count < 4 { return None },
                    Err(_)    => return None
                }

                let coil = u16::from_be_bytes([bfr[0], bfr[1]]);
                let state = u16::from_be_bytes([bfr[2], bfr[3]]);

                let state = if state == 0xff00 {
                    true
                } else if state == 0x0000 {
                    false
                } else {
                    return None
                };

                let cmd = Self::WriteCoil { coil, state };
                Some(cmd)
            },

            // Write single holding
            6 => {
                let mut bfr = [0;4];
                match reader.read(&mut bfr) {
                    Ok(count) => if count < 4 { return None },
                    Err(_)    => return None
                }

                let address = u16::from_be_bytes([bfr[0], bfr[1]]);
                let value = u16::from_be_bytes([bfr[2], bfr[3]]);

                let cmd = Self::WriteHolding {address, value};
                Some(cmd)
            },

            // Write multiple coils
            15 => {
                let mut bfr = [0;5];
                match reader.read(&mut bfr) {
                    Ok(count) => if count < 5 { return None },
                    Err(_)    => return None
                }

                let start = u16::from_be_bytes([bfr[0], bfr[1]]);
                let count = u16::from_be_bytes([bfr[2], bfr[3]]);
                let byte_count = bfr[4] as usize;

                let mut bfr = vec![0;byte_count];
                match reader.read(&mut bfr) {
                    Ok(count) => if count < byte_count { return None },
                    Err(_)    => return None
                }

                let vals = bytes_to_bools(&bfr);

                let cmd = Self::WriteMultCoil{start, count, vals};

                Some(cmd)
            },

            // Write multiple holding
            16 => {
                let mut bfr = [0;5];
                match reader.read(&mut bfr) {
                    Ok(count) => if count < 5 { return None },
                    Err(_)    => return None
                }

                let start = u16::from_be_bytes([bfr[0], bfr[1]]);
                let count = u16::from_be_bytes([bfr[2], bfr[3]]);
                let byte_count = bfr[4] as usize;

                let mut bfr = vec![0;byte_count];
                match reader.read(&mut bfr) {
                    Ok(count) => if count < byte_count { return None },
                    Err(_)    => return None
                }

                let mut vals = Vec::with_capacity(byte_count as usize / 2);
                for bytes in bfr.windows(2).step_by(2) {
                    let val = u16::from_be_bytes([bytes[0], bytes[1]]);
                    vals.push(val)
                }

                let cmd = Self::WriteMultHolding { start, count, vals };

                Some(cmd)
            }

            _ => None
        }

    }
}


impl PduCommand {
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


impl Into<Vec<u8>> for &PduCommand {
    fn into(self) -> Vec<u8> {
        let size = self.size() as usize;
        let mut v = Vec::with_capacity(size);
        v.push(self.function_code());

        match self {
            PduCommand::ReadCoils{start, count}   => {
                v.extend_from_slice(&start.to_be_bytes());
                v.extend_from_slice(&count.to_be_bytes());
            },
            PduCommand::ReadDI{start, count}   => {
                v.extend_from_slice(&start.to_be_bytes());
                v.extend_from_slice(&count.to_be_bytes());
            },
            PduCommand::ReadHolding{start, count}   => {
                v.extend_from_slice(&start.to_be_bytes());
                v.extend_from_slice(&count.to_be_bytes());
            },
            PduCommand::ReadInput{start, count}   => {
                v.extend_from_slice(&start.to_be_bytes());
                v.extend_from_slice(&count.to_be_bytes());
            },
            PduCommand::WriteCoil {coil, state} => {
                v.extend_from_slice(&coil.to_be_bytes());
                if *state {
                    v.push(0xFF);
                    v.push(0x00);
                } else {
                    v.push(0x00);
                    v.push(0x00);
                }
            }
            PduCommand::WriteHolding {address, value} => {
                v.extend_from_slice(&address.to_be_bytes());
                v.extend_from_slice(&value.to_be_bytes());
            },
            PduCommand::WriteMultCoil {start, count, vals} => {
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
            PduCommand::WriteMultHolding {start, count, vals} => {
                v.extend_from_slice(&start.to_be_bytes());
                v.extend_from_slice(&count.to_be_bytes());

                let byte_count = vals.len() as u8 * 2;
                v.push(byte_count);
            }
        } 

        v
    }
}


impl Into<Vec<u8>> for PduCommand {
    fn into(self) -> Vec<u8> {
        (&self).into()
    }
}



/// Response a server(Slave) sends in response to a command
/// 
#[derive(Clone, Debug)]
pub enum PduResponse {
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
    }
}


impl PduResponse {
    pub fn read_coils(coils: &[bool]) -> Self {
        let mut status = Vec::with_capacity(coils.len());
        status.clone_from_slice(coils);

        Self::ReadCoils { status }
    }

    pub fn read_di(di: &[bool]) -> Self {
        let mut status = Vec::with_capacity(di.len());
        status.clone_from_slice(di);
        
        Self::ReadDI { status }
    }

    pub fn read_holding(addresses: &[u16]) -> Self {
        let mut status = Vec::with_capacity(addresses.len());
        status.clone_from_slice(addresses);
        
        Self::ReadHolding { status }
    }

    pub fn read_input(addresses: &[u16]) -> Self {
        let mut status = Vec::with_capacity(addresses.len());
        status.clone_from_slice(addresses);
        
        Self::ReadInput { status }
    }

    pub fn write_coil(coil: u16, state: bool) -> Self {
        Self::WriteCoil { coil, state }
    }

    pub fn write_holding(address: u16, value: u16) -> Self {
        Self::WriteHolding { address, value }
    }


    /// Gives the size of the response in bytes
    /// 
    pub fn size(&self) -> u16 {
        match self {
            PduResponse::ReadCoils{status} => {
                // Function code + byte count
                let mut size = 2;

                if status.len() % 8 == 0 {
                    size += status.len() as u16 / 8
                } else {
                    size += status.len() as u16 / 8 + 1
                }

                size
            },
            PduResponse::ReadDI{status} => {
                // Function code + byte count
                let mut size = 2;

                if status.len() % 8 == 0 {
                    size += status.len() as u16 / 8
                } else {
                    size += status.len() as u16 / 8 + 1
                }

                size
            },
            PduResponse::ReadHolding{ status } => {
                // Function code + byte count
                let mut size = 2;

                size += status.len() as u16 * 2;
                size
            },
            PduResponse::ReadInput{ status } => {
                // Function code + byte count
                let mut size = 2;

                size += status.len() as u16 * 2;
                size
            },
            PduResponse::WriteCoil{..} => 5, // Function code + addr + value
            PduResponse::WriteHolding{..} => 5 // Function code + addr + value
        }
    }
}


impl Into<Vec<u8>> for PduResponse {
    fn into(self) -> Vec<u8> {
        (&self).into()
    }
}


impl Into<Vec<u8>> for &PduResponse {
    fn into(self) -> Vec<u8> {
        match self {
            PduResponse::ReadCoils { status } => {
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

            PduResponse::ReadDI { status } => {
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

            PduResponse::ReadHolding { status } => {
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

            PduResponse::ReadInput { status } => {
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

            PduResponse::WriteCoil { coil, state } => {
                let mut v = Vec::with_capacity(self.size() as usize);

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

            PduResponse::WriteHolding { address, value } => {
                let mut v = Vec::with_capacity(self.size() as usize);

                v.extend_from_slice(&address.to_be_bytes());
                v.extend_from_slice(&value.to_be_bytes());

                v
            }
        }
    }
}


impl ReadGet for PduResponse {
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
                
                let response = Self::ReadCoils { status };
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
                
                let response = Self::ReadDI { status };
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
                
                let response = Self::ReadHolding { status };
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
                
                let response = Self::ReadInput { status };
                Some(response)
            },

            //Write coil
            5 => {
                let mut bfr = [0; 4];
                match reader.read(&mut bfr) {
                    Ok(count) => if count < 4 { return None },
                    Err(_)    => return None,
                }

                let coil = u16::from_be_bytes([bfr[0], bfr[1]]);
                let state = u16::from_be_bytes([bfr[2], bfr[3]]);

                let state = if state == 0xFF00 {
                    true
                } else if state == 0x0000 {
                    false
                } else {
                    return None
                };

                let cmd = Self::WriteCoil { coil, state };
                Some(cmd)
            },

            //Write holding
            6 => {
                let mut bfr = [0; 4];
                match reader.read(&mut bfr) {
                    Ok(count) => if count < 4 { return None },
                    Err(_)    => return None,
                }

                let address = u16::from_be_bytes([bfr[0], bfr[1]]);
                let value = u16::from_be_bytes([bfr[2], bfr[3]]);

                let cmd = Self::WriteHolding { address, value };
                Some(cmd)
            }

            _ => None
        }
    }
}