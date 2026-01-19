use std::io::Read;
use crate::ReadGet;


/// Responses sent by the slave
/// 
pub mod responses;


/// All function codes to send to a slave
/// 
#[derive(Clone, Debug)]
pub enum FunctionCode {
    ReadCoils{start: u16, count: u16},
    ReadDI{start: u16, count: u16},
    ReadHolding{start: u16, count: u16},
    ReadInput{start: u16, count: u16},
}


impl FunctionCode {
    /// Returns the function code of the FunctionCode
    /// 
    pub fn function_code(&self) -> u8 {
        match self {
            Self::ReadCoils{..}      => 1,
            Self::ReadDI{..}         => 2,
            Self::ReadHolding{..}    => 3,
            Self::ReadInput{..}      => 4,
        }
    }
}


impl ReadGet for FunctionCode {
    fn read_get(reader: &mut impl Read) -> Option<Self> 
    where Self: Sized 
    {
        let mut bfr = [0];

        match reader.read(&mut bfr) {
            Ok(count)   => if count < 1 { return None },
            Err(_)      => return None
        }

        match bfr[0] {
            1 => {
                let mut bfr = [0;4];
                match reader.read(&mut bfr) {
                    Ok(count)   => if count < 4 { return None },
                    Err(_)      => return None
                }

                let start = u16::from_be_bytes([bfr[0], bfr[1]]);
                let count = u16::from_be_bytes([bfr[2], bfr[3]]);

                Some(Self::ReadCoils { start, count })
            },
            
            2 => {
                let mut bfr = [0;4];
                match reader.read(&mut bfr) {
                    Ok(count)   => if count < 4 { return None },
                    Err(_)      => return None
                }

                let start = u16::from_be_bytes([bfr[0], bfr[1]]);
                let count = u16::from_be_bytes([bfr[2], bfr[3]]);

                Some(Self::ReadDI { start, count })
            },
            
            3 => {
                let mut bfr = [0;4];
                match reader.read(&mut bfr) {
                    Ok(count)   => if count < 4 { return None },
                    Err(_)      => return None
                }

                let start = u16::from_be_bytes([bfr[0], bfr[1]]);
                let count = u16::from_be_bytes([bfr[2], bfr[3]]);

                Some(Self::ReadHolding { start, count })
            },
            
            4 => {
                let mut bfr = [0;4];
                match reader.read(&mut bfr) {
                    Ok(count)   => if count < 4 { return None },
                    Err(_)      => return None
                }

                let start = u16::from_be_bytes([bfr[0], bfr[1]]);
                let count = u16::from_be_bytes([bfr[2], bfr[3]]);

                Some(Self::ReadInput { start, count })
            },
            _ => None,
        }
    }
}


impl Into<Vec<u8>> for FunctionCode {
    fn into(self) -> Vec<u8> {
        let mut v = Vec::new();
        v.push(self.function_code());

        match self {
            Self::ReadCoils { start, count }        => {
                v.extend_from_slice(&start.to_be_bytes());
                v.extend_from_slice(&count.to_be_bytes());
            },
            Self::ReadDI { start, count }           => {
                v.extend_from_slice(&start.to_be_bytes());
                v.extend_from_slice(&count.to_be_bytes());
            },
            Self::ReadHolding { start, count }      => {
                v.extend_from_slice(&start.to_be_bytes());
                v.extend_from_slice(&count.to_be_bytes());
            },
            Self::ReadInput { start, count }        => {
                v.extend_from_slice(&start.to_be_bytes());
                v.extend_from_slice(&count.to_be_bytes());
            },
        }

        v
    }
}