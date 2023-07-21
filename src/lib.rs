pub mod message;

use message::{DataType, InfoType, Message, MessageType, ResponseData};
use nom::{
    bytes::streaming::{is_a, tag, take},
    character::streaming::alpha1,
    combinator::verify,
    error::{context, VerboseError, VerboseErrorKind},
    number::streaming::{le_u16, u8},
    sequence::{preceded, terminated, tuple},
    Err::Failure,
    IResult,
};
use std::str;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

fn message_type(i: &[u8]) -> Res<&[u8], MessageType> {
    context("message_type", preceded(tag("jdcp-"), take(1u8)))(i)
        .map(|(i, result)| (i, result.into()))
}

fn character_name(i: &[u8]) -> Res<&[u8], &str> {
    context("character_name", terminated(alpha1, is_a(&b"\x00"[..])))(i).map(|(i, result)| {
        (
            i,
            str::from_utf8(result).expect("Error reading character name"),
        )
    })
}

fn info_type(i: &[u8]) -> Res<&[u8], InfoType> {
    context("info_type", take(1u8))(i).map(|(i, result)| (i, result.into()))
}

fn data_size(i: &[u8]) -> Res<&[u8], u16> {
    context("data_size", le_u16)(i)
}

fn data_stats(i: &[u8]) -> Res<&[u8], Option<DataType>> {
    context("Info Type Stats", take(6u8))(i)
        .map(|(i, result)| (i, Some(DataType::STATS(result.into()))))
}

fn data_age(i: &[u8]) -> Res<&[u8], Option<DataType>> {
    context("Info Type Age", le_u16)(i).map(|(i, result)| (i, Some(DataType::AGE(result.into()))))
}

fn data_class(i: &[u8]) -> Res<&[u8], Option<DataType>> {
    context("Info Type Class", take(1u8))(i)
        .map(|(i, result)| (i, Some(DataType::CLASS(result.into()))))
}
fn data_race(i: &[u8]) -> Res<&[u8], Option<DataType>> {
    context("Info Type Race", take(1u8))(i)
        .map(|(i, result)| (i, Some(DataType::RACE(result.into()))))
}
fn data_level(i: &[u8]) -> Res<&[u8], Option<DataType>> {
    context("Info Type Level", u8)(i).map(|(i, result)| (i, Some(DataType::LEVEL(result.into()))))
}

fn data_hp(i: &[u8]) -> Res<&[u8], Option<DataType>> {
    context("Info Type HP", take(2u8))(i).map(|(i, result)| (i, Some(DataType::HP(result.into()))))
}
fn data_lenth_inconsitant_error(
    i: &[u8],
) -> Result<(&[u8], Option<DataType>), nom::Err<VerboseError<&[u8]>>> {
    Err(Failure(VerboseError {
        errors: vec![(
            i,
            VerboseErrorKind::Context("Data length inconsistent with Data Type"),
        )],
    }))
}

fn data<'a, 'b, 'c>(
    i: &'a [u8],
    len: &'b u16,
    data_type: &'c InfoType,
) -> Res<&'a [u8], Option<DataType>> {
    if *len == 0 {
        Res::from(Ok((i, None)))
    } else {
        match *data_type {
            InfoType::STATS => {
                if *len == 6 {
                    data_stats(i) //len needs to be 6
                } else {
                    data_lenth_inconsitant_error(i)
                }
            }
            InfoType::AGE => {
                if *len == 2 {
                    data_age(i) //len needs to be 2
                } else {
                    data_lenth_inconsitant_error(i)
                }
            }
            InfoType::CLASS => {
                if *len == 1 {
                    data_class(i) //len needs to be 1
                } else {
                    data_lenth_inconsitant_error(i)
                }
            }
            InfoType::RACE => {
                if *len == 1 {
                    data_race(i) //len needs to be 1
                } else {
                    data_lenth_inconsitant_error(i)
                }
            }
            InfoType::LEVEL => {
                if *len == 1 {
                    data_level(i) //len needs to be 1
                } else {
                    data_lenth_inconsitant_error(i)
                }
            }
            InfoType::HP => {
                if *len == 2 {
                    data_hp(i) //len needs to be 2
                } else {
                    data_lenth_inconsitant_error(i)
                }
            }
        }
    }
}

fn parse_stats_response(input: &[u8]) -> Res<&[u8], ResponseData> {
    context(
        "Stats Response",
        tuple((
            verify(info_type, |info: &InfoType| *info == InfoType::STATS),
            verify(le_u16, |size: &u16| *size == 6),
            data_stats,
        )),
    )(input)
    .map(|(input, response)| {
        (
            input,
            ResponseData {
                info_type: response.0,
                data_size: response.1,
                data: response.2,
            },
        )
    })
}

pub fn decode_jdcp(input: &[u8]) -> Res<&[u8], Message> {
    let (input, message_type) = message_type(input)?;
    let (input, character_name) = character_name(input)?;
    let (input, info_type) = info_type(input)?;
    let (input, data_size) = data_size(input)?;
    let (input, data) = data(input, &data_size, &info_type)?;

    let i = 1;
    if i == 2 {
        let _ = parse_stats_response(input);
    }
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
    use crate::message::StatBlock;
    use nom::Err::{Failure, Incomplete};
    use nom::Needed;

    #[test]
    fn data_type_stats_parser_works_independantly() {
        let incoming_bytes = &b"\x01\x06\x00\x08\x0c\x13\x0e\x10\x09"[..];
        let expected_remainder = &b""[..];
        let expected_result = ResponseData {
            info_type: InfoType::STATS,
            data_size: 6u16,
            data: Some(DataType::STATS(StatBlock {
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
    fn todo_data_type_age_parser_works_independantly() {
        assert!(false)
    }
    #[test]
    fn todo_data_type_class_parser_works_independantly() {
        assert!(false)
    }
    #[test]
    fn todo_data_type_race_parser_works_independantly() {
        assert!(false)
    }
    #[test]
    fn todo_data_type_level_parser_works_independantly() {
        assert!(false)
    }
    #[test]
    fn todo_data_type_hp_parser_works_independantly() {
        assert!(false)
    }

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
    fn bytes_to_message_request_level_works() {
        assert_eq!(
            decode_jdcp(&b"jdcp-\xAABart\x00\x05\x00\x00"[..]),
            Ok((
                &b""[..],
                Message {
                    message_type: MessageType::REQUEST,
                    character_name: "Bart",
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
            decode_jdcp(&b"jdcp-\xAABart\x00\x01\x00\x00"[..]),
            Ok((
                &b""[..],
                Message {
                    message_type: MessageType::REQUEST,
                    character_name: "Bart",
                    info_type: InfoType::STATS,
                    data_size: 0,
                    data: None,
                }
            ))
        );
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
    fn bytes_to_message_response_stats_works() {
        assert_eq!(
            decode_jdcp(&b"jdcp-\xBBBart\x00\x01\x06\x00\x0C\x12\x12\x10\x0F\x0C"[..]),
            Ok((
                &b""[..],
                Message {
                    message_type: MessageType::RESPONSE,
                    character_name: "Bart",
                    info_type: InfoType::STATS,
                    data_size: 6,
                    data: Some(DataType::STATS(StatBlock::new(12, 18, 18, 16, 15, 12))),
                }
            ))
        );
    }

    #[test]
    fn bytes_to_message_response_stats_error_works() {
        assert_eq!(
            decode_jdcp(&b"jdcp-\xBBBart\x00\x01\x06\xA0\x0C\x12\x12\x10\x0F\x0C"[..]),
            Err(Failure(VerboseError {
                errors: vec![(
                    &b"\x0C\x12\x12\x10\x0F\x0C"[..],
                    VerboseErrorKind::Context("Data length inconsistent with Data Type")
                )]
            }))
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
                data: Some(DataType::STATS(StatBlock::new(12, 18, 18, 16, 15, 12))),
            }
            .encode_jdcp()
        );
    }

    #[test]
    fn bytes_to_message_response_level_works() {
        let expected_message = Message {
            message_type: MessageType::RESPONSE,
            character_name: "Bart",
            info_type: InfoType::LEVEL,
            data_size: 1,
            data: Some(DataType::LEVEL(10)),
        };
        assert_eq!(
            decode_jdcp(&b"jdcp-\xBBBart\x00\x05\x01\x00\x0A"[..]),
            Ok((&b""[..], expected_message))
        );
    }

    #[test]
    fn message_response_level_to_bytes_works() {
        let expected_message = Message {
            message_type: MessageType::RESPONSE,
            character_name: "Bart",
            info_type: InfoType::LEVEL,
            data_size: 1,
            data: Some(DataType::LEVEL(10)),
        };
        assert_eq!(
            &b"jdcp-\xBBBart\x00\x05\x01\x00\x0A"[..],
            expected_message.encode_jdcp()
        )
    }

    #[test]
    fn data_cond_works() {
        let result = data(&b"\x0A\x00\xAA"[..], &2u16, &InfoType::AGE);
        assert_eq!(result, Ok((&b"\xAA"[..], Some(DataType::AGE(10)))))
    }
    #[test]
    fn data_cond_shows_incomplete_when_not_enough_data_1() {
        let result = data(&b"\x0A"[..], &2u16, &InfoType::AGE);
        assert_eq!(result, Err(Incomplete(Needed::new(1))))
    }
    #[test]
    fn data_cond_shows_incomplete_when_not_enough_data_2() {
        let result = data(&b"\x0A"[..], &6u16, &InfoType::STATS);
        assert_eq!(result, Err(Incomplete(Needed::new(5))))
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
        let result = character_name(&b"\x42\x61\x72\x74\x00\x01"[..]);
        assert_eq!(result, Ok((&b"\x01"[..], "Bart")));
    }

    #[test]
    fn message_type_byte_returns_correct_type() {
        let request_message = message_type(&b"jdcp-\xAA\x12"[..]);
        let response_message = message_type(&b"jdcp-\xBB\x12"[..]);
        assert_eq!(request_message, Ok((&b"\x12"[..], MessageType::REQUEST)));
        assert_eq!(response_message, Ok((&b"\x12"[..], MessageType::RESPONSE)));
    }

    #[test]
    fn byte_message_errors_on_incorrect_message_type() {
        let result = decode_jdcp(&b"\xFF"[..]);
        assert!(result.is_err());
    }
}
