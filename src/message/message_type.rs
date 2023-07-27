use nom::{
    bytes::streaming::{tag, take},
    error::context,
    sequence::preceded,
};

use crate::Res;

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum MessageType {
    REQUEST = 0xAA,
    RESPONSE = 0xBB,
}

impl MessageType {
    pub fn discriminant(&self) -> u8 {
        unsafe { *(self as *const Self as *const u8) }
    }
}

impl From<&[u8]> for MessageType {
    fn from(value: &[u8]) -> Self {
        match value.first() {
            Some(0xAA) => MessageType::REQUEST,
            Some(0xBB) => MessageType::RESPONSE,
            _ => unimplemented!("No other messages currently"),
        }
    }
}
pub fn message_type(i: &[u8]) -> Res<&[u8], MessageType> {
    context("message_type", preceded(tag("jdcp-"), take(1u8)))(i)
        .map(|(i, result)| (i, result.into()))
}

#[cfg(test)]
mod josh_dnd_character_protocol_message_type_tests {
    use super::*;
    #[test]
    fn message_type_byte_returns_correct_type() {
        let request_message = message_type(&b"jdcp-\xAA\x12"[..]);
        let response_message = message_type(&b"jdcp-\xBB\x12"[..]);
        assert_eq!(request_message, Ok((&b"\x12"[..], MessageType::REQUEST)));
        assert_eq!(response_message, Ok((&b"\x12"[..], MessageType::RESPONSE)));
    }
}
