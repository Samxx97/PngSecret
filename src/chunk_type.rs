use std::{error::Error, fmt, str::FromStr};
#[derive(Debug)]
pub struct ChunkType(u8, u8, u8, u8);

impl ChunkType {

    fn new(b1: u8, b2: u8, b3: u8, b4: u8) -> Result<Self, ChunkTypeError> {

        ChunkType(b1, b2, b3, b4).of_valid_byte_range()
    }

    pub fn bytes(&self) -> [u8; 4] {
        [self.0, self.1, self.2, self.3]
    }

    fn of_valid_reserved_bit(self) -> Result<Self, ChunkTypeError> {
        if self.is_reserved_bit_valid() {
            Ok(self)
        } else {
            Err(ChunkTypeError::ReservedBitInvalid)
        }
    }

    fn of_valid_byte_range(self) -> Result<Self, ChunkTypeError>{
        let valid_bytes = self.get_valid_bytes();
        let all_bytes_valid = valid_bytes.iter().fold(true, |result, x| *x && result);
        let result = match all_bytes_valid {
            true => Ok(self),
            false => Err(ChunkTypeError::InvalidByteRange("Some bytes are out of range!", valid_bytes)),
        };
        result
    }

    pub fn get_valid_bytes(&self) -> [bool; 4] {
        
        self.bytes().iter().map(|byte| { (65..91).contains(byte) || (97..123).contains(byte)})
            .collect::<Vec<bool>>()
            .try_into()
            .unwrap()
    }

    pub fn is_valid(&self) -> bool {
        let bytes_valid = self.get_valid_bytes();
        bytes_valid.iter()
        .fold(true, |result, x| *x && result)
         && self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        ( self.0 >> 5 & 1 ) as u8 == 0
    }

    pub fn is_public(&self) -> bool {
        ( self.1 >> 5 & 1 ) as u8 == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        ( self.2 >> 5 & 1 ) as u8 == 0
    }

    pub fn is_safe_to_copy(&self) -> bool {
        ( self.3 >> 5 & 1 ) as u8 == 1
    }
}

impl TryFrom<[u8; 4]> for ChunkType {

    type Error = ChunkTypeError;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {

        ChunkType::new(value[0], value[1], value[2], value[3])
    }

}

impl FromStr for ChunkType {

    type Err = ChunkTypeError;

     fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes_vec: Vec<u8> = s.bytes().collect();
        if bytes_vec.len() != 4 {
            Err(ChunkTypeError::SizeMismatch("Number of bytes expected is 4"))
        } else {
            ChunkType::new(bytes_vec[0], bytes_vec[1], bytes_vec[2], bytes_vec[3])
        }
    }
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 &&
        self.1 == other.1 &&
        self.2 == other.2 &&
        self.3 == other.3
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}{}", self.0 as char, self.1 as char, self.2 as char, self.3 as char)
    }
}

#[derive(Debug)]
pub enum ChunkTypeError {
    InvalidByteRange(&'static str, [bool; 4]),
    SizeMismatch(&'static str),
    ReservedBitInvalid,
}

impl Error for ChunkTypeError {}

impl fmt::Display for ChunkTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ChunkTypeError::InvalidByteRange(message, valid_bytes) => {
                write!(f, "{}, {}. {}. {}. {}",
                    message, 
                    valid_bytes[0],
                    valid_bytes[1],
                    valid_bytes[2],
                    valid_bytes[3]) },
            ChunkTypeError::SizeMismatch(message) => {
                write!(f, "{}", message)
            }, 
            ChunkTypeError::ReservedBitInvalid => {
                write!(f, "")
            }
        }

        
    }
}

mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }
    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }
}