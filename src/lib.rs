use nom::{
    bytes::streaming::{tag, take},
    combinator::map_res,
    IResult,
};

#[derive(Debug, PartialEq)]
enum MessageType {
    Request,
    Response,
}

fn check_type(input: &str) -> Result<MessageType, &'static str> {
    match input {
        "AA" => Ok(MessageType::Request),
        "BB" => Ok(MessageType::Response),
        _ => return Err("Invalid message type"),
    }
}

fn message_type(i: &str) -> IResult<&str, MessageType> {
    map_res(take(2u8), check_type)(i)
}

fn hex_message(input: &str) -> IResult<&str, (&str, MessageType)> {
    let (input, found_tag) = tag("jdcp-")(input)?;
    let (input, message_type) = message_type(input)?;
    Ok((input, (found_tag, message_type)))
    // let (input, (message_type, character, info_type, size_data, data)) =
    //     tuple((message_type, character, info_type, size_data, data))(input)?;

    // Ok((
    //     input,
    //     Message {
    //         message_type,
    //         character,
    //         info_type,
    //         size_data,
    //         data,
    //     },
    // ))
}

#[cfg(test)]
mod josh_dnd_character_protocol_message_tests {
    use super::*;

    #[test]
    fn test() {
        assert!(true)
    }
    
    #[test]
    fn hex_message_parses_message_type() {
        assert_eq!(
            hex_message("jdcp-AA426172740005000000"),
            Ok(("426172740005000000", ("jdcp-", MessageType::Request)))
        );
    }

    #[test]
    fn hex_message_errors_on_incorrect_message_type() {
        let result = hex_message("jdcp-FF426172740005000000");
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
    //                 hex_message("jdcp-AA426172740005000000"),
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
