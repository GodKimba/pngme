// mod chunk {
//     use std::convert::TryFrom;
//     use std::fmt;
//     use std::num::ParseIntError;
//     use std::{fmt::Display, str::FromStr};

//     struct ChunkType {
//         chunk: [u8; 4],
//     }

//     impl TryFrom<ChunkSize> for ChunkType {
//         type Error = &'static str;

//         fn try_from(value: ChunkSize) -> Result<Self, Self::Error> {
//             if value.len() <= 4 && value.len() >= 0 {
//                 Ok(ChunkType { chunk: value })
//             } else {
//                 Err("ChunkType only accepts values of size greater than 0 and smaller than 4.")
//             }
//         }
//     }
//     impl FromStr for ChunkType {
//         type Err = ParseIntError;

//         fn from_str(s: &str) -> Result<Self, Self::Err> {
//             let parsed_str = s.parse::<ChunkType>()?;
//             Ok(ChunkType { chunk: parsed_str })
//         }
//     }
//     }
//     impl PartialEq for ChunkType {}
//     impl Eq for ChunkType {}
// }

pub mod chunk {
    use std::convert::TryFrom;
    use std::fmt;
    use std::fmt::Display;
    use std::{num::ParseIntError, str::FromStr};

    pub struct ChunkType {
        chunk: [u8; 4],
    }

    impl TryFrom<[u8; 4]> for ChunkType {
        type Error = &'static str;
        fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
            if value.is_ascii() {
                Ok(ChunkType { chunk: value })
            } else {
                Err("Chunk type expects an array of type u8 with 4 values")
            }
        }
    }

    impl FromStr for ChunkType {
        type Err = &'static str;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let bytes = s.as_bytes();
            assert!(bytes.len() == 4);

            let valid_chars = bytes
                .iter()
                .all(|&b| (b >= b'a' && b <= b'z' || (b >= b'A' && b <= b'Z')));

            let sized: [u8; 4] = [bytes[0], bytes[1], bytes[2], bytes[3]];
            if valid_chars {
                Ok(ChunkType::try_from(sized)?)
            } else {
                Err("Chunk chars aren't valid")
            }
        }
    }
    impl Display for ChunkType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self.chunk)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::chunk_type::chunk::ChunkType;

    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    // #[test]
    // pub fn test_chunk_type_from_bytes() {
    //     let expected = [82, 117, 83, 116];
    //     let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

    //     assert_eq!(expected, actual.bytes());
    // }

    // #[test]
    // pub fn test_chunk_type_from_str() {
    //     let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
    //     let actual = ChunkType::from_str("RuSt").unwrap();
    //     assert_eq!(expected, actual);
    // }

    // #[test]
    // pub fn test_chunk_type_is_critical() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert!(chunk.is_critical());
    // }

    // #[test]
    // pub fn test_chunk_type_is_not_critical() {
    //     let chunk = ChunkType::from_str("ruSt").unwrap();
    //     assert!(!chunk.is_critical());
    // }

    // #[test]
    // pub fn test_chunk_type_is_public() {
    //     let chunk = ChunkType::from_str("RUSt").unwrap();
    //     assert!(chunk.is_public());
    // }

    // #[test]
    // pub fn test_chunk_type_is_not_public() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert!(!chunk.is_public());
    // }

    // #[test]
    // pub fn test_chunk_type_is_reserved_bit_valid() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert!(chunk.is_reserved_bit_valid());
    // }

    // #[test]
    // pub fn test_chunk_type_is_reserved_bit_invalid() {
    //     let chunk = ChunkType::from_str("Rust").unwrap();
    //     assert!(!chunk.is_reserved_bit_valid());
    // }

    // #[test]
    // pub fn test_chunk_type_is_safe_to_copy() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert!(chunk.is_safe_to_copy());
    // }

    // #[test]
    // pub fn test_chunk_type_is_unsafe_to_copy() {
    //     let chunk = ChunkType::from_str("RuST").unwrap();
    //     assert!(!chunk.is_safe_to_copy());
    // }

    // #[test]
    // pub fn test_valid_chunk_is_valid() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert!(chunk.is_valid());
    // }

    // #[test]
    // pub fn test_invalid_chunk_is_valid() {
    //     let chunk = ChunkType::from_str("Rust").unwrap();
    //     assert!(!chunk.is_valid());

    //     let chunk = ChunkType::from_str("Ru1t");
    //     assert!(chunk.is_err());
    // }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    // #[test]
    // pub fn test_chunk_type_trait_impls() {
    //     let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
    //     let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
    //     let _chunk_string = format!("{}", chunk_type_1);
    //     let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    // }
}
