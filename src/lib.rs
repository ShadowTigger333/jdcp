pub mod message;

use byteorder::{LittleEndian, ReadBytesExt};
use message::{
    ClassType, DataType, HealthPoints, InfoType, Message, MessageType, RaceKind, StatBlock,
};
use nom::{
    bytes::streaming::{is_a, tag, take},
    character::streaming::alpha1,
    combinator::map_res,
    number::streaming::{self, le_u16},
    IResult,
};
use std::str;

fn check_message_type(input: u8) -> Result<MessageType, &'static str> {
    match input {
        0xAA => Ok(MessageType::REQUEST),
        0xBB => Ok(MessageType::RESPONSE),
        _ => return Err("Invalid message type"),
    }
}

fn check_info_type(input: u8) -> Result<InfoType, &'static str> {
    match input {
        1 => Ok(InfoType::STATS),
        2 => Ok(InfoType::AGE),
        3 => Ok(InfoType::CLASS),
        4 => Ok(InfoType::RACE),
        5 => Ok(InfoType::LEVEL),
        6 => Ok(InfoType::HP),
        _ => return Err("Invalid info type"),
    }
}

fn message_type(i: &[u8]) -> IResult<&[u8], MessageType> {
    map_res(streaming::u8, check_message_type)(i)
}

fn character_name(i: &[u8]) -> IResult<&[u8], &[u8]> {
    alpha1(i)
}

fn info_type(i: &[u8]) -> IResult<&[u8], InfoType> {
    map_res(streaming::u8, check_info_type)(i)
}

fn data_size(i: &[u8]) -> IResult<&[u8], u16> {
    le_u16(i)
}

fn data_stats(i: &[u8]) -> StatBlock {
    StatBlock {
        strength: i[0],
        dexterity: i[1],
        constitution: i[2],
        intelligence: i[3],
        wisdom: i[4],
        charisma: i[5],
    }
}

fn data_age(i: &[u8]) -> u16 {
    let mut data = &i[0..2];
    data.read_u16::<LittleEndian>().unwrap()
}

fn data_class(i: &[u8]) -> Result<ClassType, &str> {
    match i[0] {
        1 => Ok(ClassType::ARTIFACER),
        2 => Ok(ClassType::BARBARIAN),
        3 => Ok(ClassType::BARD),
        4 => Ok(ClassType::BLOODHUNTER),
        5 => Ok(ClassType::CLERIC),
        6 => Ok(ClassType::DRUID),
        7 => Ok(ClassType::FIGHTER),
        8 => Ok(ClassType::MONK),
        9 => Ok(ClassType::PALADIN),
        10 => Ok(ClassType::RANGER),
        11 => Ok(ClassType::ROGUE),
        12 => Ok(ClassType::SORCERER),
        13 => Ok(ClassType::WARLOCK),
        14 => Ok(ClassType::WIZARD),
        _ => return Err("Invalid class type"),
    }
}

fn data_race(i: &[u8]) -> Result<RaceKind, &str> {
    match i[0] {
        1 => Ok(RaceKind::DWARF),
        2 => Ok(RaceKind::ELF),
        3 => Ok(RaceKind::GNOME),
        4 => Ok(RaceKind::HALFELF),
        5 => Ok(RaceKind::HALFLING),
        6 => Ok(RaceKind::HALFORK),
        7 => Ok(RaceKind::HUMAN),
        8 => Ok(RaceKind::ORC),
        9 => Ok(RaceKind::TIEFLING),
        _ => return Err("Invalid race kind"),
    }
}

fn data_level(i: &[u8]) -> u8 {
    i[0]
}

fn data_hp(i: &[u8]) -> HealthPoints {
    HealthPoints {
        current: i[0],
        max: i[1],
    }
}

fn parse_data<'a>(info_type: &InfoType, data: &'a [u8]) -> Option<DataType<'a>> {
    match info_type {
        InfoType::STATS => Some(DataType::STATS(data_stats(data))),
        InfoType::AGE => Some(DataType::AGE(data_age(data))),
        InfoType::CLASS => Some(DataType::CLASS(data_class(data))),
        InfoType::RACE => Some(DataType::RACE(data_race(data))),
        InfoType::LEVEL => Some(DataType::LEVEL(data_level(data))),
        InfoType::HP => Some(DataType::HP(data_hp(data))),
        _ => None,
    }
}

fn data(i: &[u8], len: u16) -> IResult<&[u8], &[u8]> {
    take(len)(i)
}

fn bytes_to_message(input: &[u8]) -> IResult<&[u8], Message> {
    let (input, _) = tag("jdcp-")(input)?;
    let (input, message_type) = message_type(input)?;
    let (input, character_name) = character_name(input)?;
    let (input, _) = is_a(&b"\x00"[..])(input)?;
    let (input, info_type) = info_type(input)?;
    let (input, data_size) = data_size(input)?;
    let (input, data) = data(input, data_size)?;

    let data: Option<DataType> = if data_size > 0 {
        parse_data(&info_type, data)
    } else {
        None
    };

    Ok((
        input,
        Message {
            message_type,
            character_name,
            info_type,
            data_size,
            data,
        },
    ))
}

#[cfg(test)]
mod josh_dnd_character_protocol_message_tests {
    use super::*;

    #[test]
    fn bytes_to_message_request_level_works() {
        assert_eq!(
            bytes_to_message(&b"jdcp-\xAABart\x00\x05\x00\x00"[..]),
            Ok((
                &b""[..],
                Message {
                    message_type: MessageType::REQUEST,
                    character_name: "Bart".as_bytes(),
                    info_type: InfoType::LEVEL,
                    data_size: 0,
                    data: None,
                }
            ))
        );
    }

    #[test]
    fn bytes_to_message_request_stats_works() {
        assert_eq!(
            bytes_to_message(&b"jdcp-\xAABart\x00\x01\x00\x00"[..]),
            Ok((
                &b""[..],
                Message {
                    message_type: MessageType::REQUEST,
                    character_name: "Bart".as_bytes(),
                    info_type: InfoType::STATS,
                    data_size: 0,
                    data: None,
                }
            ))
        );
    }

    #[test]
    fn bytes_to_message_response_stats_works() {
        assert_eq!(
            bytes_to_message(&b"jdcp-\xBBBart\x00\x01\x06\x00\x0C\x12\x12\x10\x0F\x0C"[..]),
            Ok((
                &b""[..],
                Message {
                    message_type: MessageType::RESPONSE,
                    character_name: "Bart".as_bytes(),
                    info_type: InfoType::STATS,
                    data_size: 6,
                    data: Some(DataType::STATS(StatBlock {
                        strength: 12,
                        dexterity: 18,
                        constitution: 18,
                        intelligence: 16,
                        wisdom: 15,
                        charisma: 12
                    })),
                }
            ))
        );
    }

    #[test]
    fn bytes_to_message_response_level_works() {
        let expected_message = Message {
            message_type: MessageType::RESPONSE,
            character_name: "Bart".as_bytes(),
            info_type: InfoType::LEVEL,
            data_size: 1,
            data: Some(DataType::LEVEL(10)),
        };
        assert_eq!(
            bytes_to_message(&b"jdcp-\xBBBart\x00\x05\x01\x00\x0A"[..]),
            Ok((&b""[..], expected_message))
        );
    }

    #[test]
    fn data_bytes_gets_0_bytes_if_0() {
        let result = data(&b"\xAA"[..], 0);
        assert_eq!(result, Ok((&b"\xAA"[..], &b""[..])))
    }

    #[test]
    fn data_size_bytes_returns_correct_values() {
        let result = data_size(&b"\x0A\x00\xAA"[..]);
        assert_eq!(result, Ok((&b"\xAA"[..], 10u16)))
    }

    #[test]
    fn info_type_bytes_returns_correct_types() {
        let stats = info_type(&b"\x01\xAA"[..]);
        let age = info_type(&b"\x02\xAA"[..]);
        let class = info_type(&b"\x03\xAA"[..]);
        let race = info_type(&b"\x04\xAA"[..]);
        let level = info_type(&b"\x05\xAA"[..]);
        let hp = info_type(&b"\x06\xAA"[..]);

        assert_eq!(stats, Ok((&b"\xAA"[..], InfoType::STATS)));
        assert_eq!(age, Ok((&b"\xAA"[..], InfoType::AGE)));
        assert_eq!(class, Ok((&b"\xAA"[..], InfoType::CLASS)));
        assert_eq!(race, Ok((&b"\xAA"[..], InfoType::RACE)));
        assert_eq!(level, Ok((&b"\xAA"[..], InfoType::LEVEL)));
        assert_eq!(hp, Ok((&b"\xAA"[..], InfoType::HP)));
    }

    #[test]
    fn character_name_bytes_returns_actual_name() {
        let result = character_name(&b"\x42\x61\x72\x74\x00"[..]);
        assert_eq!(result, Ok((&b"\x00"[..], "Bart".as_bytes())));
    }

    #[test]
    fn message_type_byte_returns_correct_type() {
        let request_message = message_type(&b"\xAA\x12"[..]);
        let response_message = message_type(&b"\xBB\x12"[..]);
        assert_eq!(request_message, Ok((&b"\x12"[..], MessageType::REQUEST)));
        assert_eq!(response_message, Ok((&b"\x12"[..], MessageType::RESPONSE)));
    }

    #[test]
    fn byte_message_errors_on_incorrect_message_type() {
        let result = bytes_to_message(&b"\xFF"[..]);
        assert!(result.is_err());
    }
}
