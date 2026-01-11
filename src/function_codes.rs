use std::io::Read;
use crate::ReadGet;


macro_rules! new_function {
    ( $t:ty, $( $field:ident : $ty:ty ),* ) => {

        impl $t {
            /// Creates a new instance of this type with given inputs
            pub fn new( $( $field: $ty ),* ) -> Self {
                Self {
                    $( $field ),*
                }
            }
        }

    };
}


/// Enum containing all function codes
/// 
#[derive(Clone, Debug)]
pub enum FunctionCode {
    ReadCoils(ReadCoils),
    ReadDi(ReadDI),
    ReadHolding(ReadHolding),
    ReadInput(ReadInput),
}


impl FunctionCode {
    /// Returns the function code of the FunctionCode
    /// 
    pub fn function_code(&self) -> u8 {
        match self {
            Self::ReadCoils(_)      => 1,
            Self::ReadDi(_)         => 2,
            Self::ReadHolding(_)    => 3,
            Self::ReadInput(_)      => 4,
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
            1 => Some(Self::ReadCoils(ReadCoils::read_get(reader)?)),
            2 => Some(Self::ReadDi(ReadDI::read_get(reader)?)),
            3 => Some(Self::ReadHolding(ReadHolding::read_get(reader)?)),
            4 => Some(Self::ReadInput(ReadInput::read_get(reader)?)),
            _ => None,
        }
    }
}


impl Into<Vec<u8>> for FunctionCode {
    fn into(self) -> Vec<u8> {
        let data: Vec<u8> = match self {
            Self::ReadCoils(code)       => code.into(),
            Self::ReadDi(code)              => code.into(),
            Self::ReadHolding(code)     => code.into(),
            Self::ReadInput(code)       => code.into(),
        };

        let mut v = Vec::with_capacity(data.len());
        v.extend_from_slice(&data);
        v
    }
}


/// Function code 01
/// 
/// Read Coils
/// 
#[derive(Clone, Debug)]
pub struct ReadCoils {
    // Which addr to start reading at
    start   : u16,

    // How many coils to read
    count   : u16
}

new_function!(
    ReadCoils,
    start: u16,
    count: u16
);


impl ReadGet for ReadCoils {
    fn read_get(reader: &mut impl Read) -> Option<Self> 
    where Self: Sized 
    {
        let mut bfr = [0; 4];

        match reader.read(&mut bfr) {
            Ok(count)   => if count < 4 { return None },
            Err(_)      => return None
        }

        let start_bytes = [bfr[0], bfr[1]];
        let start = u16::from_be_bytes(start_bytes);

        let count_bytes = [bfr[2], bfr[3]];
        let count = u16::from_be_bytes(count_bytes);

        let res = Self { start, count };

        Some(res)
    }
}


impl Into<Vec<u8>> for ReadCoils {
    fn into(self) -> Vec<u8> {
        let mut v = vec![1];

        v.extend_from_slice(&self.start.to_be_bytes());
        v.extend_from_slice(&self.count.to_be_bytes());

        v
    }
}


/// Function code 02
/// 
/// Read Discrete Inputs
/// 
#[derive(Clone, Debug)]
pub struct ReadDI {
    // Which addr to start reading at
    start   : u16,
    
    // How many inputs to read
    count   : u16,
}

new_function!(
    ReadDI,
    start: u16,
    count: u16
);


impl ReadGet for ReadDI {
    fn read_get(reader: &mut impl Read) -> Option<Self> 
    where Self: Sized 
    {
        let mut bfr = [0; 4];

        match reader.read(&mut bfr) {
            Ok(count)   => if count < 4 { return None },
            Err(_)      => return None
        }

        let start_bytes = [bfr[0], bfr[1]];
        let start = u16::from_be_bytes(start_bytes);

        let count_bytes = [bfr[2], bfr[3]];
        let count = u16::from_be_bytes(count_bytes);

        let res = Self { start, count };

        Some(res)
    }
}


impl Into<Vec<u8>> for ReadDI {
    fn into(self) -> Vec<u8> {
        let mut v = vec![2];

        v.extend_from_slice(&self.start.to_be_bytes());
        v.extend_from_slice(&self.count.to_be_bytes());

        v
    }
}


/// Function code 03
/// 
/// Read Holding Register
/// 
#[derive(Clone, Debug)]
pub struct ReadHolding {
    // Which addr to start reading at
    start   : u16,

    // How many registers to read
    count   : u16,
}

new_function!(
    ReadHolding,
    start: u16,
    count: u16
);


impl ReadGet for ReadHolding {
    fn read_get(reader: &mut impl Read) -> Option<Self> 
    where Self: Sized 
    {
        let mut bfr = [0; 4];

        match reader.read(&mut bfr) {
            Ok(count)   => if count < 4 { return None },
            Err(_)      => return None
        }

        let start_bytes = [bfr[0], bfr[1]];
        let start = u16::from_be_bytes(start_bytes);

        let count_bytes = [bfr[2], bfr[3]];
        let count = u16::from_be_bytes(count_bytes);

        let res = Self { start, count };

        Some(res)
    }
}


impl Into<Vec<u8>> for ReadHolding {
    fn into(self) -> Vec<u8> {
        let mut v = vec![3];

        v.extend_from_slice(&self.start.to_be_bytes());
        v.extend_from_slice(&self.count.to_be_bytes());

        v
    }
}


/// Function code 04
/// 
/// Read Input Registers
/// 
#[derive(Clone, Debug)]
pub struct ReadInput {
    // The addr to start reading at
    start   : u16,

    // How many registers to read
    count   : u16,
}

new_function!(
    ReadInput,
    start: u16,
    count: u16
);


impl ReadGet for ReadInput {
    fn read_get(reader: &mut impl Read) -> Option<Self> 
    where Self: Sized 
    {
        let mut bfr = [0; 4];

        match reader.read(&mut bfr) {
            Ok(count)   => if count < 4 { return None },
            Err(_)      => return None
        }

        let start_bytes = [bfr[0], bfr[1]];
        let start = u16::from_be_bytes(start_bytes);

        let count_bytes = [bfr[2], bfr[3]];
        let count = u16::from_be_bytes(count_bytes);

        let res = Self { start, count };

        Some(res)
    }
}


impl Into<Vec<u8>> for ReadInput {
    fn into(self) -> Vec<u8> {
        let mut v = vec![4];

        v.extend_from_slice(&self.start.to_be_bytes());
        v.extend_from_slice(&self.count.to_be_bytes());

        v
    }
}