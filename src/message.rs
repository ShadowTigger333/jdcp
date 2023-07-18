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
            .expect("Could not write data_size");
        let data_bytes: Vec<u8> = match &self.data {
            Some(DataType::STATS(char_stats_block)) => [
                char_stats_block.strength,
                char_stats_block.dexterity,
                char_stats_block.constitution,
                char_stats_block.intelligence,
                char_stats_block.wisdom,
                char_stats_block.charisma,
            ]
            .to_vec(),
            Some(DataType::AGE(char_age)) => {
                let mut age = Vec::new();
                age.write_u16::<LittleEndian>(*char_age)
                    .expect("Age not written");
                return age;
            }
            Some(DataType::CLASS(char_class)) => [char_class.discriminant()].to_vec(),
            Some(DataType::RACE(char_race)) => [char_race.discriminant()].to_vec(),
            Some(DataType::LEVEL(char_level)) => [*char_level].to_vec(),
            Some(DataType::HP(char_hp)) => [char_hp.current, char_hp.max].to_vec(),
            None => Vec::new(),
        };

        [
            b"jdcp-",
            message_type_bytes,
            char_bytes,
            null_byte,
            info_type_bytes,
            &data_size_bytes,
            &data_bytes,
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
