pub mod function_codes;

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
    fn read_get(&mut self) -> Option<Self> where Self: Sized;
}


/// The header of any transmission
/// 
pub(crate) struct MBAPHeader {
    pub TransactionID: [u8; 2],
    pub ProtocolID   : [u8; 2],
    pub Length       : [u8; 2],
    pub UnitID       : u8,
}


/// It is a master unit, whoever though of calling it Client/Server instead of Master/Slave should burn in hell
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

/// It is a slave unit, whoever though of calling it Client/Server instead of Master/Slave should burn in hell
/// 
pub struct Server {

}


/// A command sent from the client(Master) to the server(Slave)
/// 
pub struct Command {
    header: MBAPHeader
}