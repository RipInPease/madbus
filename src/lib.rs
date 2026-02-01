/// Helper tools and functions to be used within the crate
/// 
pub(crate) mod helpers;

/// Holds commands and respoonses
/// 
pub mod function_codes;
use function_codes::{PduCommand, PduResponse};

use std::io::prelude::*;
use std::io::Error as IOError;
use std::net::{
    TcpStream,
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
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct Client;

impl Client {
    /// Send a Request through a TcpStream to the Server(Slave)
    /// 
    pub fn send_request(stream: &mut TcpStream, command: PduCommand, unit_id: u8) -> Result<(), IOError> {
        let header = 
            MBAPHeader{ 
                transaction_id:1, 
                protocol_id: 0,  
                length: {command.size() + 1},
                unit_id
            };

        let request = AduRequest{ header, command };
        let data: Vec<u8> = request.into();

        stream.write(&data)?;

        Ok(())
    }


    /// Reads a response over the TcpStream.
    /// 
    /// Returns None if failed
    /// 
    pub fn read_response(stream: &mut TcpStream) -> Option<AduResposne> {
        AduResposne::read_get(stream)
    }
}


/// It is a slave unit, whoever thought of calling it Client/Server instead of Master/Slave should burn in hell
/// 
#[derive(Clone, Debug)]
pub struct Server;

impl Server {
    /// Reads a Request from the a client(Master) through a TcpStream. 
    /// 
    /// Returns None if failed
    /// 
    pub fn read_request(stream: &mut TcpStream) -> Option<AduRequest> {
        AduRequest::read_get(stream)
    }


    /// Send a response over a TcpStream to the Client(Master)
    /// 
    pub fn send_resp(stream: &mut TcpStream, response: AduResposne) -> Result<(), IOError> { 
        let data: Vec<u8> = response.into();

        stream.write(&data)?;

        Ok(())
    }
}


/// A command sent from the client(Master) to the server(Slave)
/// 
#[derive(Clone, Debug)]
pub struct AduRequest {
    header  : MBAPHeader,
    command : PduCommand
}


impl ReadGet for AduRequest {
    fn read_get(reader: &mut impl Read) -> Option<Self> where Self: Sized {
        let header = MBAPHeader::read_get(reader)?;
        let command = PduCommand::read_get(reader)?;

        Some(Self{ header, command })
    }
}


impl Into<Vec<u8>> for &AduRequest {
    fn into(self) -> Vec<u8> {
        let mut v: Vec<u8> = (&self.header).into();

        let command: Vec<u8> = (&self.command).into();
        v.extend_from_slice(&command);

        v
    }
}


impl Into<Vec<u8>> for AduRequest {
    fn into(self) -> Vec<u8> {
        (&self).into()
    }
}


/// A response sent from the Server(Slace) to the Client(Master)
/// 
#[derive(Clone, Debug)]
pub struct AduResposne {
    header: MBAPHeader,
    response: PduResponse,
}


impl ReadGet for AduResposne {
    fn read_get(reader: &mut impl Read) -> Option<Self> where Self: Sized {
        let header = MBAPHeader::read_get(reader)?;
        let response = PduResponse::read_get(reader)?;

        Some(Self{ header, response })
    }
}


impl Into<Vec<u8>> for &AduResposne {
    fn into(self) -> Vec<u8> {
        let mut v: Vec<u8> = (&self.header).into();

        let response: Vec<u8> = (&self.response).into();
        v.extend_from_slice(&response);

        v
    }
}


impl Into<Vec<u8>> for AduResposne {
    fn into(self) -> Vec<u8> {
        (&self).into()
    }
}