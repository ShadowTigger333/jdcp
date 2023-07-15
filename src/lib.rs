use byteorder::{LittleEndian, ReadBytesExt};
use hex::FromHexError;
use nom::{
    bytes::streaming::{tag, take, take_until},
    combinator::map_res,
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

fn check_message_type(input: &str) -> Result<MessageType, &'static str> {
    match input {
        "AA" => Ok(MessageType::Request),
        "BB" => Ok(MessageType::Response),
        _ => return Err("Invalid message type"),
    }
}

fn check_info_type(input: &str) -> Result<InfoType, &'static str> {
    match input {
        "01" => Ok(InfoType::Stats),
        "02" => Ok(InfoType::Age),
        "03" => Ok(InfoType::Class),
        "04" => Ok(InfoType::Race),
        "05" => Ok(InfoType::Level),
        "06" => Ok(InfoType::HP),
        _ => return Err("Invalid info type"),
    }
}

fn message_type(i: &str) -> IResult<&str, MessageType> {
    map_res(take(2u8), check_message_type)(i)
}

fn to_str(input: &str) -> Result<Vec<u8>, FromHexError> {
    hex::decode(input)
}

fn until_nul(i: &str) -> IResult<&str, &str> {
    take_until("00")(i)
}

fn character_name(i: &str) -> IResult<&str, Vec<u8>> {
    map_res(until_nul, to_str)(i)
}

fn info_type(i: &str) -> IResult<&str, InfoType> {
    map_res(take(2u8), check_info_type)(i)
}

fn to_u16(input: &str) -> Result<u16, std::io::Error> {
    let data: Vec<u8> = hex::decode(input).unwrap();
    let mut current = &data[..];

    current.read_u16::<LittleEndian>()
}

fn data_size(i: &str) -> IResult<&str, u16> {
    map_res(take(4u8), to_u16)(i)
}

fn data(i: &str, len: u16) -> IResult<&str, &str> {
    take(len)(i)
}

fn hex_message(input: &str) -> IResult<&str, (&str, MessageType, Vec<u8>, InfoType, u16, &str)> {
    let (input, found_tag) = tag("jdcp-")(input)?;
    let (input, message_type) = message_type(input)?;
    let (input, character_name) = character_name(input)?;
    let (input, _) = tag("00")(input)?;
    let (input, info_type) = info_type(input)?;
    let (input, data_size) = data_size(input)?;
    let (input, data) = data(input, data_size)?;
    Ok((
        input,
        (
            found_tag,
            message_type,
            character_name,
            info_type,
            data_size,
            data
        ),
    ))
    // let (input, (message_type, character, info_type, data_size, data)) =
    //     tuple((message_type, character, info_type, data_size, data))(input)?;

    // Ok((
    //     input,
    //     Message {
    //         message_type,
    //         character,
    //         info_type,
    //         data_size,
    //         data,
    //     },
    // ))
}

#[cfg(test)]
mod josh_dnd_character_protocol_message_tests {
    use super::*;

    #[test]
    fn hex_message_parses_data_size() {
        assert_eq!(
            hex_message("jdcp-AA4261727400050000"),
            Ok((
                "",
                (
                    "jdcp-",
                    MessageType::Request,
                    "Bart".to_owned().into_bytes(),
                    InfoType::Level,
                    0u16,
                    "",
                )
            ))
        );
    }

    #[test]
    fn data_gets_0_bytes_if_0() {
        let result = data("remain", 0);
        assert_eq!(result, Ok(("remain", "")))
    }

    // #[test]
    // fn hex_message_parses_data_size() {
    //     assert_eq!(
    //         hex_message("jdcp-AA4261727400050000"),
    //         Ok((
    //             "",
    //             (
    //                 "jdcp-",
    //                 MessageType::Request,
    //                 "Bart".to_owned().into_bytes(),
    //                 InfoType::Level,
    //                 0u16
    //             )
    //         ))
    //     );
    // }

    #[test]
    fn data_size_returns_correct_values() {
        let result = data_size("0A00remainder");
        assert_eq!(result, Ok(("remainder", 10u16)))
    }

    // #[test]
    // fn hex_message_parses_info_type() {
    //     assert_eq!(
    //         hex_message("jdcp-AA426172740005000000"),
    //         Ok((
    //             "000000",
    //             (
    //                 "jdcp-",
    //                 MessageType::Request,
    //                 "Bart".to_owned().into_bytes(),
    //                 InfoType::Level
    //             )
    //         ))
    //     );
    // }

    #[test]
    fn info_type_returns_correct_types() {
        let stats = info_type("01remainder1");
        let age = info_type("02remainder2");
        let class = info_type("03remainder3");
        let race = info_type("04remainder4");
        let level = info_type("05remainder5");
        let hp = info_type("06remainder6");

        assert_eq!(stats, Ok(("remainder1", InfoType::Stats)));
        assert_eq!(age, Ok(("remainder2", InfoType::Age)));
        assert_eq!(class, Ok(("remainder3", InfoType::Class)));
        assert_eq!(race, Ok(("remainder4", InfoType::Race)));
        assert_eq!(level, Ok(("remainder5", InfoType::Level)));
        assert_eq!(hp, Ok(("remainder6", InfoType::HP)));
    }

    // #[test]
    // fn hex_message_parses_character_name() {
    //     assert_eq!(
    //         hex_message("jdcp-AA426172740005000000"),
    //         Ok((
    //             "0005000000",
    //             ("jdcp-", MessageType::Request, "Bart".to_owned().into_bytes())
    //         ))
    //     );
    // }

    #[test]
    fn character_name_returns_actual_name() {
        let result = character_name("4261727400");
        assert_eq!(result, Ok(("00", "Bart".to_owned().into_bytes())));
    }

    // #[test]
    // fn hex_message_parses_message_type() {
    //     assert_eq!(
    //         hex_message("jdcp-AA426172740005000000"),
    //         Ok(("426172740005000000", ("jdcp-", MessageType::Request)))
    //     );
    // }

    #[test]
    fn message_type_returns_correct_type() {
        let result = message_type("AAremainder");
        assert_eq!(result, Ok(("remainder", MessageType::Request)))
    }

    #[test]
    fn hex_message_errors_on_incorrect_message_type() {
        let result = hex_message("jdcp-FF426172740005000000");
        assert!(result.is_err());
    }

    #[test]
    fn hex_message_errors_on_incorrect_tag() {
        let result = hex_message("abcd-FF426172740005000000");
        assert!(result.is_err());
    }

    // First passing test
    // #[test]
    // fn hex_message_parses_tag() {
    //     assert_eq!(
    //         hex_message("jdcp-AA426172740005000000"),
    //         Ok(("426172740005000000", "jdcp-"))
    //     );
    // }

    // #[test]
    // #[ignore = "final test"]
    // fn hex_message_request_level_works() {
    //     assert_eq!(
    //                 hex_message("jdcp-AA4261727400050000"),
    //                 Ok(("", Message {
    //                             message_type: MessageType::Request,
    //                             character: "Bart",
    //                             info_type: InfoType::Level,
    //                             size_data: 0
    //                             data: None
    //                 }))
    //     );
    // }
}
