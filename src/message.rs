mod data;

pub use data::*;

#[derive(Debug, PartialEq)]
pub struct Message<'a> {
    pub message_type: MessageType,
    pub character_name: &'a [u8],
    pub info_type: InfoType,
    pub data_size: u16,
    pub data: Option<DataType>,
}

#[derive(Debug, PartialEq)]
pub enum MessageType {
    REQUEST,
    RESPONSE,
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
pub enum InfoType {
    STATS,
    AGE,
    CLASS,
    RACE,
    LEVEL,
    HP,
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
