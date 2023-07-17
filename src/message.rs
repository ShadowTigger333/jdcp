mod data;

use byteorder::{LittleEndian, WriteBytesExt};
pub use data::*;

#[derive(Debug, PartialEq)]
pub struct Message<'a> {
    pub message_type: MessageType,
    pub character_name: &'a [u8],
    pub info_type: InfoType,
    pub data_size: u16,
    pub data: Option<DataType>,
}

impl Message<'_> {
    

    pub fn encode_jdcp(self: &Self) -> Vec<u8> {
        let message_type_bytes: &[u8] = &[self.message_type.discriminant()];
        let char_bytes: &[u8] = self.character_name;
        let null_byte: &[u8] = b"\x00";
        let info_type_bytes: &[u8] = &[self.info_type.discriminant()];
        let mut data_size_bytes = Vec::new();
        data_size_bytes
            .write_u16::<LittleEndian>(self.data_size)
            .unwrap();

        //TODO: Data needs to be encoded

        [
            b"jdcp-",
            message_type_bytes,
            char_bytes,
            null_byte,
            info_type_bytes,
            &data_size_bytes,
        ]
        .concat()
    }
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum MessageType {
    REQUEST = 0xAA,
    RESPONSE = 0xBB,
}

impl MessageType {
    fn discriminant(&self) -> u8 {
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
    fn discriminant(&self) -> u8 {
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
