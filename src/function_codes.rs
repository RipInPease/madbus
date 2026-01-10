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