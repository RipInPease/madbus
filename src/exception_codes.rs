/// Exception codes explaining possible causes of failure
/// 
#[derive(Clone, Debug)]
pub enum Exception {
    /// The function code is unknown by the server
    IllegalCode = 0x01,

    /// Dependant on the request
    IllegalAddress = 0x02,

    /// Dependant on the request
    IllegalDataValue = 0x03,

    /// The server failed during the exception
    ServerFailure = 0x04,

    /// The server accepted the request, but requires a long time to execute
    Acknowledge = 0x05,

    /// The server was unable to accept request
    ServerBusy = 0x06,

    /// Gateway paths not available
    GatewayUnavail = 0x0A,

    /// The targeted device failed to respond
    BadDevice = 0x0B
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
        }
    }
}