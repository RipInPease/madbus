/// Turns a slice of bools into a vec of bytes
/// 
pub fn bools_to_bytes(bools: &[bool]) -> Vec<u8> {
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


/// Turns a slice of bytes to a vec of bools
/// 
pub fn bytes_to_bools(bytes: &[u8]) -> Vec<bool> {
    let mut v = Vec::with_capacity(bytes.len() / 8 + 1);

    for byte in bytes {
        for i in 0..8 {
            let bool = (*byte >> i) & 1 == 1;
            v.push(bool)
        }
    }

    v
}


#[cfg(test)]
mod test {
    use super::bools_to_bytes;
    use super::bytes_to_bools;

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

    #[test]
    fn test_bools_to_bytes4() {
        let bools = vec![true, false, true, true];
        let bytes = bools_to_bytes(&bools);
        
        assert_eq!(bytes[0], 13);
    }


    #[test]
    fn test_bytes_to_bools1() {
        let bytes: Vec<u8> = vec![1];
        let bools = bytes_to_bools(&bytes);

        assert_eq!(bools[0], true);
        assert_eq!(bools[1], false);
    }

    #[test]
    fn test_bytes_to_bools2() {
        let bytes: Vec<u8> = vec![1, 1];
        let bools = bytes_to_bools(&bytes);

        assert_eq!(&bools, &[true, false, false, false, false, false, false, false,
                             true, false, false, false, false, false, false, false]);
    }

    #[test]
    fn test_bytes_to_bools3() {
        let bytes: Vec<u8> = vec![136, 99];
        let bools = bytes_to_bools(&bytes);

        assert_eq!(&bools, &[false, false, false, true, false, false, false, true,
                             true, true, false, false, false, true, true, false]);
    }
}