use nom::{error::context, number::streaming::le_u16};

use crate::Res;

pub fn parse_data_size(i: &[u8]) -> Res<&[u8], u16> {
    context("data_size", le_u16)(i)
}

#[cfg(test)]
mod josh_dnd_character_protocol_data_size_tests {
    use super::*;
    #[test]
    fn data_size_bytes_returns_correct_values() {
        let result = parse_data_size(&b"\x0A\x00\xAA"[..]);
        assert_eq!(result, Ok((&b"\xAA"[..], 10u16)))
    }
}
