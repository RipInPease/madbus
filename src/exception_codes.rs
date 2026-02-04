/// Exception codes explaining possible causes of failure
/// 
#[derive(Debug)]
pub enum Exception {
    /// The function code is unknown by the server
    IllegalCode,

    /// Dependant on the request
    IllegalAddress,

    /// Dependant on the request
    IllegalDataValue,

    /// The server failed during the exception
    ServerFailure,

    /// The server accepted the request, but requires a long time to execute
    Acknowledge,

    /// The server was unable to accept request
    ServerBusy,

    /// Gateway paths not available
    GatewayUnavail,

    /// The targeted device failed to respond
    BadDevice,

    /// std::io::Error
    IOError(std::io::Error),
}


impl Exception {
    /// Gives the associated exception code
    pub fn code(&self) -> u8 {
        match self {
            Self::IllegalCode       => 0x01,
            Self::IllegalAddress    => 0x02,
            Self::IllegalDataValue  => 0x03,
            Self::ServerFailure     => 0x04,
            Self::Acknowledge       => 0x05,
            Self::ServerBusy        => 0x06,
            Self::GatewayUnavail    => 0x0A,
            Self::BadDevice         => 0x0B,
            Self::IOError(_)        => 0x00,
        }
    }
}


impl Into<u8> for &Exception {
    fn into(self) -> u8 {
        self.code()
    }
}


impl Into<u8> for Exception {
    fn into(self) -> u8 {
        self.code()
    }
}


impl TryFrom<u8> for Exception {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::IllegalCode),
            0x02 => Ok(Self::IllegalAddress),
            0x03 => Ok(Self::IllegalDataValue),
            0x04 => Ok(Self::ServerFailure),
            0x05 => Ok(Self::Acknowledge),
            0x06 => Ok(Self::ServerBusy),
            0x0A => Ok(Self::GatewayUnavail),
            0x0B => Ok(Self::BadDevice),
            _    => Err(())
        }
    }
}