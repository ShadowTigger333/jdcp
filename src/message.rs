mod data;

pub use data::*;

#[derive(Debug, PartialEq)]
pub struct Message<'a> {
    pub message_type: MessageType,
    pub character_name: &'a [u8],
    pub info_type: InfoType,
    pub data_size: u16,
    pub data: Option<DataType<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum MessageType {
    REQUEST,
    RESPONSE,
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
