mod character_name;
mod data;
mod message_type;

use byteorder::{LittleEndian, WriteBytesExt};
pub use character_name::*;
pub use data::*;
pub use info_type::*;
pub use message_type::*;

use crate::character::character_data::CharacterData;

use self::data::info_type::InfoType;

#[derive(Debug, PartialEq)]
pub struct Message<'a> {
    pub message_type: MessageType,
    pub character_name: &'a str,
    pub info_type: InfoType,
    pub data_size: u16,
    pub data: Option<CharacterData>,
}

impl Message<'_> {
    pub fn encode_jdcp(self: &Self) -> Vec<u8> {
        let message_type_bytes: &[u8] = &[self.message_type.discriminant()];
        let char_bytes: &str = self.character_name;
        let null_byte: &[u8] = b"\x00";
        let info_type_bytes: &[u8] = &[self.info_type.discriminant()];
        let mut data_size_bytes = Vec::new();
        data_size_bytes
            .write_u16::<LittleEndian>(self.data_size)
            .expect("Could not write data_size");
        let data_bytes: Vec<u8> = match &self.data {
            Some(CharacterData::STATS(char_stats_block)) => [
                char_stats_block.strength,
                char_stats_block.dexterity,
                char_stats_block.constitution,
                char_stats_block.intelligence,
                char_stats_block.wisdom,
                char_stats_block.charisma,
            ]
            .to_vec(),
            Some(CharacterData::AGE(char_age)) => {
                let mut age = Vec::new();
                age.write_u16::<LittleEndian>(*char_age)
                    .expect("Age not written");
                return age;
            }
            Some(CharacterData::CLASS(char_class)) => [char_class.discriminant()].to_vec(),
            Some(CharacterData::RACE(char_race)) => [char_race.discriminant()].to_vec(),
            Some(CharacterData::LEVEL(char_level)) => [*char_level].to_vec(),
            Some(CharacterData::HP(char_hp)) => [char_hp.current, char_hp.max].to_vec(),
            None => Vec::new(),
        };

        [
            b"jdcp-",
            message_type_bytes,
            char_bytes.as_bytes(),
            null_byte,
            info_type_bytes,
            &data_size_bytes,
            &data_bytes,
        ]
        .concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        character::character_data::{health_points::HealthPoints, stat_block::StatBlock},
        decode_jdcp,
    };
    use nom::AsBytes;

    #[test]
    fn message_request_level_to_bytes_works() {
        assert_eq!(
            &b"jdcp-\xAABart\x00\x05\x00\x00"[..],
            Message {
                message_type: MessageType::REQUEST,
                character_name: "Bart",
                info_type: InfoType::LEVEL,
                data_size: 0,
                data: None,
            }
            .encode_jdcp()
        )
    }

    #[test]
    fn message_request_stats_to_bytes_works() {
        assert_eq!(
            &b"jdcp-\xAABart\x00\x01\x00\x00"[..],
            Message {
                message_type: MessageType::REQUEST,
                character_name: "Bart",
                info_type: InfoType::STATS,
                data_size: 0,
                data: None,
            }
            .encode_jdcp()
        )
    }

    #[test]
    fn message_response_stats_to_bytes_works() {
        assert_eq!(
            &b"jdcp-\xBBBart\x00\x01\x06\x00\x0C\x12\x12\x10\x0F\x0C"[..],
            Message {
                message_type: MessageType::RESPONSE,
                character_name: "Bart",
                info_type: InfoType::STATS,
                data_size: 6,
                data: Some(CharacterData::STATS(StatBlock::new(12, 18, 18, 16, 15, 12))),
            }
            .encode_jdcp()
        );
    }

    #[test]
    fn message_response_level_to_bytes_works() {
        let expected_message = Message {
            message_type: MessageType::RESPONSE,
            character_name: "Bart",
            info_type: InfoType::LEVEL,
            data_size: 1,
            data: Some(CharacterData::LEVEL(10)),
        };
        assert_eq!(
            &b"jdcp-\xBBBart\x00\x05\x01\x00\x0A"[..],
            expected_message.encode_jdcp()
        )
    }

    #[test]
    fn back_and_forth_conversion() {
        let first_message = Message {
            message_type: MessageType::RESPONSE,
            character_name: "Bart",
            info_type: InfoType::HP,
            data_size: 2,
            data: Some(CharacterData::HP(HealthPoints {
                current: 34,
                max: 42,
            })),
        };
        let msg_vec = first_message.encode_jdcp();
        let buff = msg_vec.as_bytes();
        let result_message = decode_jdcp(&buff).unwrap().1;

        assert_eq!(first_message, result_message)
    }
}
