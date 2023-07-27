pub mod message;

use message::{parse_character_name, parse_data, parse_message_type, Message};
use nom::{error::VerboseError, IResult};

type Res<T, U> = IResult<T, U, VerboseError<T>>;

//TODO: Not sure how to cleanly throw custom errors when something doesn't parse correctly (Ex: when the alt tag fails in the data parser)
// fn data_lenth_inconsitant_error(
//     i: &[u8],
// ) -> Result<(&[u8], Option<DataType>), nom::Err<VerboseError<&[u8]>>> {
//     Err(Failure(VerboseError {
//         errors: vec![(
//             i,
//             VerboseErrorKind::Context("Data length inconsistent with Data Type"),
//         )],
//     }))
// }

pub fn decode_jdcp(input: &[u8]) -> Res<&[u8], Message> {
    let (input, message_type) = parse_message_type(input)?;
    let (input, character_name) = parse_character_name(input)?;
    let (input, message_data) = parse_data(input, &message_type)?;
    Ok((
        input,
        Message {
            message_type,
            character_name,
            info_type: message_data.info_type,
            data_size: message_data.data_size,
            data: message_data.data,
        },
    ))
}

#[cfg(test)]
mod josh_dnd_character_protocol_message_tests {
    use crate::message::character_data::stat_block::StatBlock;
    use crate::message::character_data::CharacterData;
    use crate::message::info_type::InfoType;
    use crate::message::MessageType;

    use super::*;
    use nom::error::ErrorKind;
    use nom::error::VerboseErrorKind::{Context, Nom};
    use nom::Err::Error;

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
                    data: Some(CharacterData::STATS(StatBlock::new(12, 18, 18, 16, 15, 12))),
                }
            ))
        );
    }

    #[test]
    fn bytes_to_message_response_stats_error_works() {
        assert_eq!(
            decode_jdcp(&b"jdcp-\xBBBart\x00\x01\x06\xA0\x0C\x12\x12\x10\x0F\x0C"[..]),
            Err(Error(VerboseError {
                errors: vec![
                    (
                        &b"\x01\x06\xA0\x0C\x12\x12\x10\x0F\x0C"[..],
                        Nom(ErrorKind::Verify)
                    ),
                    (
                        &b"\x01\x06\xA0\x0C\x12\x12\x10\x0F\x0C"[..],
                        Context("HP Response")
                    ),
                    (
                        &b"\x01\x06\xA0\x0C\x12\x12\x10\x0F\x0C"[..],
                        Nom(ErrorKind::Alt)
                    ),
                    (
                        &b"\x01\x06\xA0\x0C\x12\x12\x10\x0F\x0C"[..],
                        Context("Parse Response")
                    )
                ]
            }))
        )
    }

    #[test]
    fn bytes_to_message_response_level_works() {
        let expected_message = Message {
            message_type: MessageType::RESPONSE,
            character_name: "Bart",
            info_type: InfoType::LEVEL,
            data_size: 1,
            data: Some(CharacterData::LEVEL(10)),
        };
        assert_eq!(
            decode_jdcp(&b"jdcp-\xBBBart\x00\x05\x01\x00\x0A"[..]),
            Ok((&b""[..], expected_message))
        );
    }

    #[test]
    fn byte_message_errors_on_incorrect_message_type() {
        let result = decode_jdcp(&b"\xFF"[..]);
        assert!(result.is_err());
    }
}
