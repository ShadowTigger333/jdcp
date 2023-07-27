use nom::{
    bytes::streaming::is_a, character::streaming::alpha1, error::context, sequence::terminated,
};
use std::str;

use crate::Res;

pub fn parse_character_name(i: &[u8]) -> Res<&[u8], &str> {
    context("character_name", terminated(alpha1, is_a(&b"\x00"[..])))(i).map(|(i, result)| {
        (
            i,
            str::from_utf8(result).expect("Error reading character name"),
        )
    })
}

#[cfg(test)]
mod josh_dnd_character_protocol_character_name_tests {
    use super::*;

    #[test]
    fn character_name_bytes_returns_actual_name() {
        let result = parse_character_name(&b"\x42\x61\x72\x74\x00\x01"[..]);
        assert_eq!(result, Ok((&b"\x01"[..], "Bart")));
    }
}
