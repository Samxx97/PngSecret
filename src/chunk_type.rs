use std::{error::Error, fmt};
pub struct ChunkType(u8, u8, u8, u8);

const MASK: u8 = 0b00010000 as u8;

impl ChunkType {
    
    pub fn bytes(&self) -> [u8; 4] {
        [self.0, self.1, self.2, self.3]
    }

    pub fn bytes_validity(&self) -> [bool; 4] {

        let mut bytes_validity: [bool; 4] = [false; 4];
        let chunk_type_array: [u8; 4] = self.bytes();
        for i in 0..4 {
            if !(65..91).contains(&chunk_type_array[i]) &&
               !(97..123).contains(&chunk_type_array[i]) {
                bytes_validity[i] = false;
            } else {
                bytes_validity[i] = true;
            }
        }
        bytes_validity
    }

    pub fn is_valid(&self) -> bool {
        let bytes_valid = self.bytes_validity();
        bytes_valid.iter().fold(true, |result, x| *x && result)
    }
    pub fn print(&self, index : usize) {
        let chunk_array = self.bytes();
        print!("value of {} byte is {}", index, chunk_array[index])
    }
}

impl TryFrom<[u8; 4]> for ChunkType {

    type Error = ChunkTypeError;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let chunk_type = ChunkType(value[0], value[1], value[2], value[3]);
        let bytes_valid = chunk_type.bytes_validity();
        match chunk_type.is_valid() {
            true => Ok(chunk_type),
            false => Err(ChunkTypeError("bytes out of range!", bytes_valid))
        }
        
    }

}

#[derive(Debug)]
pub struct ChunkTypeError(&'static str, [bool; 4]);

impl Error for ChunkTypeError {}

impl fmt::Display for ChunkTypeError {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid Chunk Type {}, bytes validity: {}. {}. {}. {}",
         self.0, 
         self.1[0],
         self.1[1],
         self.1[2],
         self.1[3])
    }
}