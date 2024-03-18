use std::fmt::Display;
use std::fs::write;
use std::io::Read;
use std::str::FromStr;
use std::{convert::TryInto, io};

use anyhow::{anyhow, bail};

use crate::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Type {
    Ancillary,
    Private,
    Reserved,
    SafeToCopy,
}

impl Type {
    fn valid_bit(&self) -> u8 {
        match self {
            Type::Ancillary => 0,
            Type::Private => 1,
            Type::Reserved => 0,
            Type::SafeToCopy => 1,
        }
    }

    fn position(&self) -> u8 {
        match self {
            Type::Ancillary => 0,
            Type::Private => 1,
            Type::Reserved => 2,
            Type::SafeToCopy => 3,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    chunk: String,
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        let binding = self.chunk.as_bytes();
        let bytes = binding.bytes();
        let mut res: [u8; 4] = [0; 4];
        for (i, byte) in bytes.enumerate() {
            res[i] = byte.unwrap();
        }
        res
    }

    pub fn is_safe_to_copy(&self) -> bool {
        let byte_data = self.bytes();
        let mask = 1 << 5;
        byte_data[3] & mask != 0
    }

    /// Valid bytes are represented by the characters A-Z or a-z
    pub fn is_valid_byte(byte: u8) -> bool {
        byte.is_ascii_alphabetic()
    }

    pub fn is_valid(&self) -> bool {
        let byte_data = self.bytes();
        println!("{:?}", byte_data);

        if !self.chunk.chars().all(|c| c.is_ascii_alphabetic()) {
            println!("Result: false");
            return false;
        }

        let the_char = self.chunk.chars().nth(2).unwrap();
        println!("Reserved: {:?}", the_char);
        if !self.chunk.chars().nth(2).unwrap().is_ascii_uppercase() {
            println!("Result: false");
            return false;
        }
        let mut res = false;

        // Checking 5th bit validity in each byte
        for (j, byte) in byte_data.iter().enumerate() {
            let mask = 1 << 5; // Mask to isolate the 5th bit
            println!("Byte in binary: {:08b}", byte);
            let bit_is_set = byte & mask != 0;
            println!("Byte: {:?}, Bit is set: {}", byte, bit_is_set);

            if j == 2 {
                res = !bit_is_set;
                println!("Reserved bit is valid: {}", res);
            }
        }

        println!("Result: {}", res);
        res
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let data = s.to_string();
        let chunk = ChunkType { chunk: data };
        if chunk.bytes().into_iter().all(ChunkType::is_valid_byte) {
            Ok(chunk)
        } else {
            Err(anyhow!("Couldn't convert from string").into())
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.chunk)
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    fn try_from(value: [u8; 4]) -> Result<Self> {
        let data_vec = value.to_vec();
        let data = String::from_utf8(data_vec);
        let res = ChunkType {
            chunk: data.unwrap(),
        };

        Ok(res)
    }
    type Error = Error;
}

#[cfg(test)]
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

    // #[test]
    // pub fn test_chunk_type_is_critical() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert!(chunk.is_critical());
    // }
    //
    // #[test]
    // pub fn test_chunk_type_is_not_critical() {
    //     let chunk = ChunkType::from_str("ruSt").unwrap();
    //     assert!(!chunk.is_critical());
    // }
    //
    // #[test]
    // pub fn test_chunk_type_is_public() {
    //     let chunk = ChunkType::from_str("RUSt").unwrap();
    //     assert!(chunk.is_public());
    // }
    //
    // #[test]
    // pub fn test_chunk_type_is_not_public() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert!(!chunk.is_public());
    // }
    //
    // #[test]
    // pub fn test_chunk_type_is_reserved_bit_valid() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert!(chunk.is_reserved_bit_valid());
    // }
    //
    // #[test]
    // pub fn test_chunk_type_is_reserved_bit_invalid() {
    //     let chunk = ChunkType::from_str("Rust").unwrap();
    //     assert!(!chunk.is_reserved_bit_valid());
    // }

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

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
