use std::io::Read;
use crate::ReadGet;


/// The response for ReadCoil command
/// 
#[derive(Clone, Debug)]
pub struct ReadCoilsResp {
    bytes: u8,
    coils: Vec<bool>
}


impl ReadCoilsResp {
    /// Creates a new instance with given coils
    /// 
    pub fn new(coils: &[bool]) -> Self {
        let bytes = if coils.len() % 8 != 0 {
            (coils.len() / 8 + 1) as u8
        } else {
            coils.len() as u8 / 8
        };

        let mut v = Vec::with_capacity(coils.len());
        v.extend_from_slice(coils);


        Self{ bytes, coils: v }
    }
}


impl ReadGet for ReadCoilsResp {
    fn read_get(reader: &mut impl Read) -> Option<Self> 
    where Self: Sized 
    {
        let mut bfr = [0];
        match reader.read(&mut bfr) {
            Ok(count)   => if count < 1 { return None },
            Err(_)      => return None
        }

        let bytes = bfr[0];
        if bytes % 2 != 0 { return None }

        let mut bfr = vec![0; bytes as usize];
        if reader.read_exact(&mut bfr).is_err() {
            return None
        }

        let mut coils: Vec<bool> = Vec::with_capacity(bytes as usize / 2);

        for mut bytes in bfr {
            for _ in 0..8 {
                coils.push(bytes & 1 == 1);
                bytes >>= 1;
            }
        }

        Some(Self{ bytes, coils })
    }
}


impl Into<Vec<u8>> for ReadCoilsResp {
    fn into(self) -> Vec<u8> {
        let mut v = Vec::with_capacity(self.bytes as usize + 1);
        v.push(self.bytes);

        let mut byte = 0;
        for (i, coil) in self.coils.iter().enumerate() {
            byte |= (*coil as u8) << (i % 8);

            if i % 8 == 7 {
                v.push(byte);
                byte = 0;
            }
        }

        if self.coils.len() % 8 != 0 {
            v.push(byte)
        }

        v
    }
}


/// The response for ReadDI command
/// 
#[derive(Clone, Debug)]
pub struct ReadDIResp {
    bytes: u8,
    coils: Vec<bool>
}


impl ReadDIResp {
    /// Creates a new instance with given coils
    /// 
    pub fn new(coils: &[bool]) -> Self {
        let bytes = if coils.len() % 8 != 0 {
            (coils.len() / 8 + 1) as u8
        } else {
            coils.len() as u8 / 8
        };

        let mut v = Vec::with_capacity(coils.len());
        v.extend_from_slice(coils);


        Self{ bytes, coils: v }
    }
}


impl Into<Vec<u8>> for ReadDIResp {
    fn into(self) -> Vec<u8> {
        let mut v = Vec::with_capacity(self.bytes as usize + 1);
        v.push(self.bytes);

        let mut byte = 0;
        for (i, coil) in self.coils.iter().enumerate() {
            byte |= (*coil as u8) << (i % 8);

            if i % 8 == 7 {
                v.push(byte);
                byte = 0;
            }
        }

        if self.coils.len() % 8 != 0 {
            v.push(byte)
        }

        v
    }
}



/// The response for ReadHolding command
/// 
#[derive(Clone, Debug)]
pub struct ReadHoldingResp {
    bytes: u8,
    data : Vec<u16>
}


impl ReadHoldingResp {
    /// Creates a new instance with given data
    /// 
    pub fn new(data: &[u16]) -> Self {
        let bytes = data.len() as u8 * 2;

        let mut v = Vec::with_capacity(bytes as usize);
        v.extend_from_slice(data);

        Self{ bytes, data: v }
    }
}


impl ReadGet for ReadHoldingResp {
    fn read_get(reader: &mut impl Read) -> Option<Self> 
    where Self: Sized 
    {
        let mut bfr = [0];
        match reader.read(&mut bfr) {
            Ok(count)   => if count < 1 { return None },
            Err(_)      => return None
        }

        let bytes = bfr[0];
        if bytes % 2 != 0 { return None }

        let mut bfr = vec![0; bytes as usize];
        if reader.read_exact(&mut bfr).is_err() {
            return None
        }

        let mut data: Vec<u16> = Vec::with_capacity(bytes as usize / 2);

        for bytes in bfr.windows(2).step_by(2) {
            let bytes = [bytes[0], bytes[1]];
            let word = u16::from_be_bytes(bytes);
            data.push(word)
        }

        Some(Self{ bytes, data })
    }
}


impl Into<Vec<u8>> for ReadHoldingResp {
    fn into(self) -> Vec<u8> {
        let mut v = Vec::with_capacity(self.bytes as usize + 1);
        v.push(self.bytes);

        for word in self.data {
            v.extend_from_slice(&word.to_be_bytes());
        }
        v
    }
}


/// The response for ReadInput command
/// 
#[derive(Clone, Debug)]
pub struct ReadInputResp {
    bytes: u8,
    data : Vec<u16>
}


impl ReadInputResp {
    /// Creates a new instance with given data
    /// 
    pub fn new(data: &[u16]) -> Self {
        let bytes = data.len() as u8 * 2;

        let mut v = Vec::with_capacity(bytes as usize);
        v.extend_from_slice(data);

        Self{ bytes, data: v }
    }
}


impl ReadGet for ReadInputResp {
    fn read_get(reader: &mut impl Read) -> Option<Self> 
    where Self: Sized 
    {
        let mut bfr = [0];
        match reader.read(&mut bfr) {
            Ok(count)   => if count < 1 { return None },
            Err(_)      => return None
        }

        let bytes = bfr[0];
        if bytes % 2 != 0 { return None }

        let mut bfr = vec![0; bytes as usize];
        if reader.read_exact(&mut bfr).is_err() {
            return None
        }

        let mut data: Vec<u16> = Vec::with_capacity(bytes as usize / 2);

        for bytes in bfr.windows(2).step_by(2) {
            let bytes = [bytes[0], bytes[1]];
            let word = u16::from_be_bytes(bytes);
            data.push(word)
        }

        Some(Self{ bytes, data })
    }
}


impl Into<Vec<u8>> for ReadInputResp {
    fn into(self) -> Vec<u8> {
        let mut v = Vec::with_capacity(self.bytes as usize + 1);
        v.push(self.bytes);

        for word in self.data {
            v.extend_from_slice(&word.to_be_bytes());
        }
        v
    }
}