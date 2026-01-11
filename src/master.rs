use std::io::{Read, Write, Result as IOResult};
use crate::serial_port::SerialPort;


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
}


impl Read for Master {
    fn read(&mut self, bfr: &mut[u8]) -> IOResult<usize> {
        self.port.read(bfr)
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