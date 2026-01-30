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
        count: u8,
        status: Vec<bool>
    },

    /// Function code 0x02
    ReadDI{
        count: u8,
        status: Vec<bool>
    },

    /// Function code 0x03
    ReadHolding{
        count: u8,
        status: Vec<u16>
    },

    /// Function code 0x04
    ReadInput{
        count: u8,
        status: Vec<u16>
    },
}


impl Response {
    pub fn read_coils(coils: &[bool]) -> Self {
        let count = if coils.len() % 8 > 0 {
            coils.len() as u8 / 8 + 1
        } else {
            coils.len() as u8 / 8
        };

        let mut status = Vec::with_capacity(coils.len());
        status.clone_from_slice(coils);

        Self::ReadCoils { count, status }
    }

    pub fn read_di(di: &[bool]) -> Self {
        let count = if di.len() % 8 > 0 {
            di.len() as u8 / 8 + 1
        } else {
            di.len() as u8 / 8
        };

        let mut status = Vec::with_capacity(di.len());
        status.clone_from_slice(di);
        
        Self::ReadDI { count, status }
    }

    pub fn read_holding(addresses: &[u16]) -> Self {
        let count = addresses.len() as u8 * 2;

        let mut status = Vec::with_capacity(addresses.len());
        status.clone_from_slice(addresses);
        
        Self::ReadHolding { count, status }
    }

    pub fn read_input(addresses: &[u16]) -> Self {
        let count = addresses.len() as u8 * 2;

        let mut status = Vec::with_capacity(addresses.len());
        status.clone_from_slice(addresses);
        
        Self::ReadInput { count, status }
    }
}


impl Into<Vec<u8>> for &Response {
    fn into(self) -> Vec<u8> {
        match self {
            Response::ReadCoils { count, status } => {
                //The byte count + the byte count itself + function code
                let mut v = Vec::with_capacity(*count as usize + 2);

                // Function code
                v.push(1);

                //Byte count
                v.push(*count);

                //Coils status
                v.extend_from_slice(&bools_to_bytes(&status));

                v
            },

            Response::ReadDI { count, status } => {
                //The byte count + the byte count itself + function code
                let mut v = Vec::with_capacity(*count as usize + 2);

                // Function code
                v.push(2);

                //Byte count
                v.push(*count);

                //Coils status
                v.extend_from_slice(&bools_to_bytes(&status));

                v
            },

            Response::ReadHolding { count, status } => {
                //The byte count + the byte count itself + function code
                let mut v = Vec::with_capacity(*count as usize + 2);

                // Function code
                v.push(3);

                //Byte count
                v.push(*count);

                //Coils status
                for word in status {
                    let bytes = word.to_be_bytes();
                    v.extend_from_slice(&bytes);
                }

                v
            },

            Response::ReadInput { count, status } => {
                //The byte count + the byte count itself + function code
                let mut v = Vec::with_capacity(*count as usize + 2);

                // Function code
                v.push(3);

                //Byte count
                v.push(*count);

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



/// Turns a slice of bools into a vec of bytes
/// 
fn bools_to_bytes(bools: &[bool]) -> Vec<u8> {
    let mut v = Vec::with_capacity(bools.len() / 8 + 1);

    for bools in bools.windows(8).step_by(8) {
        let mut byte = 0x0;
        for bool in bools.iter().rev() {
            byte <<= 1;
            byte |= *bool as u8;
        }

        v.push(byte)
    }

    let remain = bools.len() % 8;
    let mut byte = 0x0;
    for bool in bools[bools.len() - remain..].iter().rev() {
        byte <<= 1;
        byte |= *bool as u8;
    }
    v.push(byte);

    v
}


#[cfg(test)]
mod test {
    use super::bools_to_bytes;

    #[test]
    fn test_bools_to_bytes1() {
        let bools = vec![true;8];
        let bytes = bools_to_bytes(&bools);
        
        assert_eq!(bytes[0], 255);
    }

    #[test]
    fn test_bools_to_bytes2() {
        let bools = vec![true;9];
        let bytes = bools_to_bytes(&bools);
        
        assert_eq!(bytes[0], 255);
        assert_eq!(bytes[1], 1);
    }

    #[test]
    fn test_bools_to_bytes3() {
        let bools = vec![true, false, true];
        let bytes = bools_to_bytes(&bools);
        
        assert_eq!(bytes[0], 5);
    }
}