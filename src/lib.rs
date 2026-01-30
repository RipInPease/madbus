/// Helper tools and functions to be used within the crate
/// 
pub(crate) mod helpers;

/// Holds commands and respoonses
/// 
pub mod function_codes;
use function_codes::{Command, Response};

use std::io::prelude::*;
use std::net::{
    TcpListener,
    Ipv4Addr,
    SocketAddrV4,
};


/// A type can be constructed from bytes read from a reader.
/// 
/// Return NONE if any error occured, else returns Self
/// 
pub trait ReadGet {
    fn read_get(reader: &mut impl Read) -> Option<Self> where Self: Sized;
}


/// The header of any transmission
/// 
pub(crate) struct MBAPHeader {
    pub transaction_id: u16,
    pub protocol_id   : u16,
    pub length        : u16,
    pub unit_id       : u8,
}


impl ReadGet for MBAPHeader {
    fn read_get(reader: &mut impl Read) -> Option<Self> where Self: Sized {
        let mut bfr = [0;7];

        match reader.read(&mut bfr) {
            Ok(count) => if count < 7 { return None },
            Err(_)    => return None
        }
        
        let transaction_id = u16::from_be_bytes([bfr[0], bfr[1]]);
        let protocol_id = u16::from_be_bytes([bfr[2], bfr[3]]);
        let length = u16::from_be_bytes([bfr[4], bfr[5]]);
        let unit_id = bfr[6];

        let header = Self{ transaction_id, protocol_id, length, unit_id };
        Some(header)
    }
}


impl Into<Vec<u8>> for MBAPHeader {
    fn into(self) -> Vec<u8> {
        (&self).into()
    }
}


impl Into<Vec<u8>> for &MBAPHeader {
    fn into(self) -> Vec<u8> {
        //Header is 7 bytes
        let mut v = Vec::with_capacity(7);

        v.extend_from_slice(&self.transaction_id.to_be_bytes());
        v.extend_from_slice(&self.protocol_id.to_be_bytes());
        v.extend_from_slice(&self.length.to_be_bytes());
        v.push(self.unit_id);

        v
    }
}

/// It is a master unit, whoever thought of calling it Client/Server instead of Master/Slave should burn in hell
/// 
pub struct Client {
    ip: SocketAddrV4,
}


impl Client {
    pub fn new<T: Into<Ipv4Addr>>(ip: T) -> Self {
        let ip = ip.into();
        let ip = SocketAddrV4::new(ip, 502);
        Self{ ip }
    }
}

/// It is a slave unit, whoever thought of calling it Client/Server instead of Master/Slave should burn in hell
/// 
pub struct Server {

}


/// A command sent from the client(Master) to the server(Slave)
/// 
pub struct Request {
    header  : MBAPHeader,
    command : Command
}