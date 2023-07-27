use nom::{bytes::streaming::take, error::context};

use crate::Res;

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum InfoType {
    STATS = 1,
    AGE = 2,
    CLASS = 3,
    RACE = 4,
    LEVEL = 5,
    HP = 6,
}

impl InfoType {
    pub fn discriminant(&self) -> u8 {
        unsafe { *(self as *const Self as *const u8) }
    }
}

impl From<&[u8]> for InfoType {
    fn from(value: &[u8]) -> Self {
        match value.first() {
            Some(1) => InfoType::STATS,
            Some(2) => InfoType::AGE,
            Some(3) => InfoType::CLASS,
            Some(4) => InfoType::RACE,
            Some(5) => InfoType::LEVEL,
            Some(6) => InfoType::HP,
            _ => unimplemented!("No other info_types currently"),
        }
    }
}

pub fn parse_info_type(i: &[u8]) -> Res<&[u8], InfoType> {
    context("info_type", take(1u8))(i).map(|(i, result)| (i, result.into()))
}

#[cfg(test)]
mod josh_dnd_character_protocol_info_type_tests {
    use super::*;

    #[test]
    fn info_type_bytes_returns_correct_types() {
        let stats = parse_info_type(&b"\x01\xAA"[..]);
        let age = parse_info_type(&b"\x02\xAA"[..]);
        let class = parse_info_type(&b"\x03\xAA"[..]);
        let race = parse_info_type(&b"\x04\xAA"[..]);
        let level = parse_info_type(&b"\x05\xAA"[..]);
        let hp = parse_info_type(&b"\x06\xAA"[..]);

        assert_eq!(stats, Ok((&b"\xAA"[..], InfoType::STATS)));
        assert_eq!(age, Ok((&b"\xAA"[..], InfoType::AGE)));
        assert_eq!(class, Ok((&b"\xAA"[..], InfoType::CLASS)));
        assert_eq!(race, Ok((&b"\xAA"[..], InfoType::RACE)));
        assert_eq!(level, Ok((&b"\xAA"[..], InfoType::LEVEL)));
        assert_eq!(hp, Ok((&b"\xAA"[..], InfoType::HP)));
    }
}
