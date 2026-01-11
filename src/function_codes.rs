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


impl Into<Vec<u8>> for FunctionCode {
    fn into(self) -> Vec<u8> {
        let code = self.function_code();
        
        let data: Vec<u8> = match self {
            Self::ReadCoils(code)       => code.into(),
            Self::ReadDi(code)              => code.into(),
            Self::ReadHolding(code)     => code.into(),
            Self::ReadInput(code)       => code.into(),
        };

        let mut v = Vec::with_capacity(data.len()+1);
        v.push(code);
        v.extend_from_slice(&data);
        v
    }
}


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


impl Into<Vec<u8>> for ReadDI {
    fn into(self) -> Vec<u8> {
        let mut v = vec![1];

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


impl Into<Vec<u8>> for ReadHolding {
    fn into(self) -> Vec<u8> {
        let mut v = vec![1];

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


impl Into<Vec<u8>> for ReadInput {
    fn into(self) -> Vec<u8> {
        let mut v = vec![1];

        v.extend_from_slice(&self.start.to_be_bytes());
        v.extend_from_slice(&self.count.to_be_bytes());

        v
    }
}