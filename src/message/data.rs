pub mod character_data;
pub mod data_size;
pub mod info_type;

use self::{
    character_data::{parse_age, parse_class, parse_hp, parse_level, parse_race, parse_stats},
    data_size::parse_data_size,
    info_type::{parse_info_type, InfoType},
};
use super::MessageType;
use crate::{character::character_data::CharacterData, Res};
use nom::{branch::alt, combinator::verify, error::context, sequence::tuple};

#[derive(Debug, PartialEq)]
pub struct MessageData {
    pub info_type: InfoType,
    pub data_size: u16,
    pub data: Option<CharacterData>,
}

#[derive(Debug, PartialEq)]
pub struct RequestData {
    pub info_type: InfoType,
}
pub fn parse_data<'a, 'b>(
    i: &'a [u8],
    message_type: &'b MessageType,
) -> Res<&'a [u8], MessageData> {
    if *message_type == MessageType::REQUEST {
        parse_request(i)
    } else {
        context(
            "Parse Response",
            alt((
                parse_stats_response,
                parse_age_response,
                parse_class_response,
                parse_race_response,
                parse_level_response,
                parse_hp_response,
            )),
        )(i)
    }
}
pub fn parse_stats_response(input: &[u8]) -> Res<&[u8], MessageData> {
    context(
        "Stats Response",
        tuple((
            verify(parse_info_type, |info: &InfoType| *info == InfoType::STATS),
            verify(parse_data_size, |size: &u16| *size == 6),
            parse_stats,
        )),
    )(input)
    .map(|(input, response)| {
        (
            input,
            MessageData {
                info_type: response.0,
                data_size: response.1,
                data: Some(response.2),
            },
        )
    })
}

pub fn parse_age_response(input: &[u8]) -> Res<&[u8], MessageData> {
    context(
        "Age Response",
        tuple((
            verify(parse_info_type, |info: &InfoType| *info == InfoType::AGE),
            verify(parse_data_size, |size: &u16| *size == 2),
            parse_age,
        )),
    )(input)
    .map(|(input, response)| {
        (
            input,
            MessageData {
                info_type: response.0,
                data_size: response.1,
                data: Some(response.2),
            },
        )
    })
}

pub fn parse_class_response(input: &[u8]) -> Res<&[u8], MessageData> {
    context(
        "Class Response",
        tuple((
            verify(parse_info_type, |info: &InfoType| *info == InfoType::CLASS),
            verify(parse_data_size, |size: &u16| *size == 1),
            parse_class,
        )),
    )(input)
    .map(|(input, response)| {
        (
            input,
            MessageData {
                info_type: response.0,
                data_size: response.1,
                data: Some(response.2),
            },
        )
    })
}

pub fn parse_race_response(input: &[u8]) -> Res<&[u8], MessageData> {
    context(
        "Race Response",
        tuple((
            verify(parse_info_type, |info: &InfoType| *info == InfoType::RACE),
            verify(parse_data_size, |size: &u16| *size == 1),
            parse_race,
        )),
    )(input)
    .map(|(input, response)| {
        (
            input,
            MessageData {
                info_type: response.0,
                data_size: response.1,
                data: Some(response.2),
            },
        )
    })
}

pub fn parse_level_response(input: &[u8]) -> Res<&[u8], MessageData> {
    context(
        "Level Response",
        tuple((
            verify(parse_info_type, |info: &InfoType| *info == InfoType::LEVEL),
            verify(parse_data_size, |size: &u16| *size == 1),
            parse_level,
        )),
    )(input)
    .map(|(input, response)| {
        (
            input,
            MessageData {
                info_type: response.0,
                data_size: response.1,
                data: Some(response.2),
            },
        )
    })
}
fn parse_hp_response(input: &[u8]) -> Res<&[u8], MessageData> {
    context(
        "HP Response",
        tuple((
            verify(parse_info_type, |info: &InfoType| *info == InfoType::HP),
            verify(parse_data_size, |size: &u16| *size == 2),
            parse_hp,
        )),
    )(input)
    .map(|(input, response)| {
        (
            input,
            MessageData {
                info_type: response.0,
                data_size: response.1,
                data: Some(response.2),
            },
        )
    })
}

pub fn parse_request(input: &[u8]) -> Res<&[u8], MessageData> {
    context(
        "Request",
        tuple((
            parse_info_type,
            verify(parse_data_size, |size: &u16| *size == 0),
        )),
    )(input)
    .map(|(input, response)| {
        (
            input,
            MessageData {
                info_type: response.0,
                data_size: response.1,
                data: None,
            },
        )
    })
}

#[cfg(test)]
mod josh_dnd_character_protocol_data_tests {
    use crate::character::character_data::{
        class_type::ClassType, health_points::HealthPoints, race_kind::RaceKind,
        stat_block::StatBlock,
    };
    use nom::error::VerboseErrorKind::{Context, Nom};
    use nom::error::{ErrorKind, VerboseError};
    use nom::Err::Error;

    use super::*;

    #[test]
    fn data_type_stats_parser_works_independantly() {
        let incoming_bytes = &b"\x01\x06\x00\x08\x0c\x13\x0e\x10\x09"[..];
        let expected_remainder = &b""[..];
        let expected_result = MessageData {
            info_type: InfoType::STATS,
            data_size: 6u16,
            data: Some(CharacterData::STATS(StatBlock {
                strength: 0x08,
                dexterity: 0x0c,
                constitution: 0x13,
                intelligence: 0x0e,
                wisdom: 0x10,
                charisma: 0x09,
            })),
        };

        assert_eq!(
            parse_stats_response(incoming_bytes),
            Ok((expected_remainder, expected_result))
        );
    }
    #[test]
    fn data_type_age_parser_works_independantly() {
        let incoming_bytes = &b"\x02\x02\x00\x00\xA0"[..];
        let expected_remainder = &b""[..];
        let expected_result = MessageData {
            info_type: InfoType::AGE,
            data_size: 2u16,
            data: Some(CharacterData::AGE(0xA000)),
        };

        assert_eq!(
            parse_age_response(incoming_bytes),
            Ok((expected_remainder, expected_result))
        );
    }
    #[test]
    fn data_type_class_parser_works_independantly() {
        let incoming_bytes = &b"\x03\x01\x00\x03"[..];
        let expected_remainder = &b""[..];
        let expected_result = MessageData {
            info_type: InfoType::CLASS,
            data_size: 1u16,
            data: Some(CharacterData::CLASS(ClassType::BARD)),
        };

        assert_eq!(
            parse_class_response(incoming_bytes),
            Ok((expected_remainder, expected_result))
        );
    }
    #[test]
    fn data_type_race_parser_works_independantly() {
        let incoming_bytes = &b"\x04\x01\x00\x04"[..];
        let expected_remainder = &b""[..];
        let expected_result = MessageData {
            info_type: InfoType::RACE,
            data_size: 1u16,
            data: Some(CharacterData::RACE(RaceKind::HALFELF)),
        };

        assert_eq!(
            parse_race_response(incoming_bytes),
            Ok((expected_remainder, expected_result))
        );
    }
    #[test]
    fn data_type_level_parser_works_independantly() {
        let incoming_bytes = &b"\x05\x01\x00\x12"[..];
        let expected_remainder = &b""[..];
        let expected_result = MessageData {
            info_type: InfoType::LEVEL,
            data_size: 1u16,
            data: Some(CharacterData::LEVEL(0x12)),
        };

        assert_eq!(
            parse_level_response(incoming_bytes),
            Ok((expected_remainder, expected_result))
        );
    }
    #[test]
    fn data_type_hp_parser_works_independantly() {
        let incoming_bytes = &b"\x06\x02\x00\x22\x25"[..];
        let expected_remainder = &b""[..];
        let expected_result = MessageData {
            info_type: InfoType::HP,
            data_size: 2u16,
            data: Some(CharacterData::HP(HealthPoints {
                current: 0x22,
                max: 0x25,
            })),
        };

        assert_eq!(
            parse_hp_response(incoming_bytes),
            Ok((expected_remainder, expected_result))
        );
    }
    #[test]
    fn data_request_works() {
        let expected_result = MessageData {
            info_type: InfoType::AGE,
            data_size: 0,
            data: None,
        };
        let result = parse_data(&b"\x02\x00\x00\xAA"[..], &MessageType::REQUEST);
        assert_eq!(result, Ok((&b"\xAA"[..], expected_result)))
    }
    #[test]
    fn data_shows_error_when_corrupted() {
        let result = parse_data(&b"\x02\x50\x11\x12\x12"[..], &MessageType::REQUEST);
        assert_eq!(
            result,
            Err(Error(VerboseError {
                errors: vec![
                    (&b"\x50\x11\x12\x12"[..], Nom(ErrorKind::Verify)),
                    (&b"\x02\x50\x11\x12\x12"[..], Context("Request"))
                ]
            }))
        )
    }
}
