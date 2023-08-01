use nom::{
    bytes::streaming::take,
    error::context,
    number::streaming::{le_u16, u8},
};

use crate::{Res, character::character_data::CharacterData};

pub fn parse_stats(i: &[u8]) -> Res<&[u8], CharacterData> {
    context("Info Type Stats", take(6u8))(i)
        .map(|(i, result)| (i, CharacterData::STATS(result.into())))
}

pub fn parse_age(i: &[u8]) -> Res<&[u8], CharacterData> {
    context("Info Type Age", le_u16)(i).map(|(i, result)| (i, CharacterData::AGE(result.into())))
}

pub fn parse_class(i: &[u8]) -> Res<&[u8], CharacterData> {
    context("Info Type Class", take(1u8))(i)
        .map(|(i, result)| (i, CharacterData::CLASS(result.into())))
}
pub fn parse_race(i: &[u8]) -> Res<&[u8], CharacterData> {
    context("Info Type Race", take(1u8))(i)
        .map(|(i, result)| (i, CharacterData::RACE(result.into())))
}
pub fn parse_level(i: &[u8]) -> Res<&[u8], CharacterData> {
    context("Info Type Level", u8)(i).map(|(i, result)| (i, CharacterData::LEVEL(result.into())))
}

pub fn parse_hp(i: &[u8]) -> Res<&[u8], CharacterData> {
    context("Info Type HP", take(2u8))(i).map(|(i, result)| (i, CharacterData::HP(result.into())))
}
