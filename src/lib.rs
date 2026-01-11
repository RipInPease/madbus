/// Basic handling of creating and using a serial port
/// 
pub(crate) mod serial_port;


/// Horror lies beyond these walls
/// 
pub mod function_codes;


/// The master unit in the modbus line
/// 
mod master;
pub use master::Master;


/// A slave unit in the modbus line
/// 
mod slave;
pub use slave::Slave;


/// A command to send from the master to a slave device
/// 
pub struct Command {
    /// Which slave the command is to be sent to
    addr    : u8,

    /// The command to be sent to the slave
    cmd     : function_codes::FunctionCode, 
}


impl Into<Vec<u8>> for Command {
    fn into(self) -> Vec<u8> {
        let cmd: Vec<u8> = self.cmd.into();
        
        let mut data = Vec::with_capacity(cmd.len() + 3);
        data.push(self.addr);
        data.extend_from_slice(&cmd);

        let crc = crc_gen(&data);
        data.extend_from_slice(&crc);
        
        data
    }
}


/// Create a CRC based on input bytes
/// 
/// https://www.modbus.org/file/secure/modbusoverserial.pdf page 39
/// for reference
/// 
pub fn crc_gen(bytes: &[u8]) -> [u8; 2] {
    let mut crc = u16::MAX;

    for b in bytes {
        crc ^= *b as u16;

        for _ in 0..8 {
            if crc & 1 != 0 {
                crc >>= 1;
                crc ^= 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }
    
    crc.to_le_bytes()
}


/// Checks if data is uncorrputed.
/// Returns true if data is uncorrupted, otherwise false
/// 
pub fn crc_check(data: &[u8]) -> bool {
    // Not enough data (crc is two bytes, need at least one more)
    if data.len() < 3 {
        return false
    }

    let original_crc = &data[data.len()-3..];
    let calculated_crc = crc_gen(&data[..data.len()-2]);

    original_crc == calculated_crc
}


// Crc checks
#[cfg(test)]
mod tests {
    use super::crc_gen;

    #[test]
    fn test_crc_empty() {
        let data: [u8; 0] = [];
        assert_eq!(crc_gen(&data), [0xFF, 0xFF]);
    }

    #[test]
    fn crc_test1() {
        let data = vec![0];
        let crc = crc_gen(&data);
        assert_eq!(crc, [0xBF, 0x40])
    }

    #[test]
    fn crc_test2() {
        let data = vec![0x01, 0x30, 0xFF];
        let crc = crc_gen(&data);
        assert_eq!(crc, [0x74, 0x40])
    }

    #[test]
    fn crc_test3() {
        let data = vec![0x90, 0xFA, 0xBD, 0x01, 0x80, 0x4B, 0x7C];
        let crc = crc_gen(&data);
        assert_eq!(crc, [0xC4, 0x4D])
    }
}