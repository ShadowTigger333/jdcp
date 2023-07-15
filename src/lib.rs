use nom::{
    bytes::streaming::{is_a, tag, take},
    character::streaming::alpha1,
    combinator::map_res,
    number::streaming::{self, le_u16},
    IResult,
};
use std::str;

#[derive(Debug, PartialEq)]
enum MessageType {
    Request,
    Response,
}

#[derive(Debug, PartialEq)]
enum InfoType {
    Stats,
    Age,
    Class,
    Race,
    Level,
    HP,
}

#[derive(Debug, PartialEq)]
struct Message<'a> {
    message_type: MessageType,
    character_name: &'a [u8],
    info_type: InfoType,
    data_size: u16,
    data: &'a [u8],
}

fn check_message_type(input: u8) -> Result<MessageType, &'static str> {
    match input {
        0xAA => Ok(MessageType::Request),
        0xBB => Ok(MessageType::Response),
        _ => return Err("Invalid message type"),
    }
}

fn check_info_type(input: u8) -> Result<InfoType, &'static str> {
    match input {
        1 => Ok(InfoType::Stats),
        2 => Ok(InfoType::Age),
        3 => Ok(InfoType::Class),
        4 => Ok(InfoType::Race),
        5 => Ok(InfoType::Level),
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
                    message_type: MessageType::Request,
                    character_name: "Bart".as_bytes(),
                    info_type: InfoType::Level,
                    data_size: 0,
                    data: &b""[..],
                }
            ))
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

        assert_eq!(stats, Ok((&b"\xAA"[..], InfoType::Stats)));
        assert_eq!(age, Ok((&b"\xAA"[..], InfoType::Age)));
        assert_eq!(class, Ok((&b"\xAA"[..], InfoType::Class)));
        assert_eq!(race, Ok((&b"\xAA"[..], InfoType::Race)));
        assert_eq!(level, Ok((&b"\xAA"[..], InfoType::Level)));
        assert_eq!(hp, Ok((&b"\xAA"[..], InfoType::HP)));
    }

    #[test]
    fn character_name_bytes_returns_actual_name() {
        let result = character_name(&b"\x42\x61\x72\x74\x00"[..]);
        assert_eq!(result, Ok((&b"\x00"[..], "Bart".as_bytes())));
    }

    #[test]
    fn message_type_byte_returns_correct_type() {
        let result = message_type(&b"\xAA\x12"[..]);
        assert_eq!(result, Ok((&b"\x12"[..], MessageType::Request)))
    }

    #[test]
    fn byte_message_errors_on_incorrect_message_type() {
        let result = bytes_to_message(&b"\xFF"[..]);
        assert!(result.is_err());
    }
}
