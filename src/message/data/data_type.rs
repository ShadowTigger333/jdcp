use nom::{
    bytes::streaming::take,
    error::context,
    number::streaming::{le_u16, u8},
};

use crate::Res;

use self::{
    class_type::ClassType, health_points::HealthPoints, race_kind::RaceKind, stat_block::StatBlock,
};

pub mod class_type;
pub mod health_points;
pub mod race_kind;
pub mod stat_block;

#[derive(Debug, PartialEq)]
pub enum DataType {
    STATS(StatBlock),
    AGE(u16),
    CLASS(ClassType),
    RACE(RaceKind),
    LEVEL(u8),
    HP(HealthPoints),
}

pub fn parse_stats(i: &[u8]) -> Res<&[u8], DataType> {
    context("Info Type Stats", take(6u8))(i).map(|(i, result)| (i, DataType::STATS(result.into())))
}

pub fn parse_age(i: &[u8]) -> Res<&[u8], DataType> {
    context("Info Type Age", le_u16)(i).map(|(i, result)| (i, DataType::AGE(result.into())))
}

pub fn parse_class(i: &[u8]) -> Res<&[u8], DataType> {
    context("Info Type Class", take(1u8))(i).map(|(i, result)| (i, DataType::CLASS(result.into())))
}
pub fn parse_race(i: &[u8]) -> Res<&[u8], DataType> {
    context("Info Type Race", take(1u8))(i).map(|(i, result)| (i, DataType::RACE(result.into())))
}
pub fn parse_level(i: &[u8]) -> Res<&[u8], DataType> {
    context("Info Type Level", u8)(i).map(|(i, result)| (i, DataType::LEVEL(result.into())))
}

pub fn parse_hp(i: &[u8]) -> Res<&[u8], DataType> {
    context("Info Type HP", take(2u8))(i).map(|(i, result)| (i, DataType::HP(result.into())))
}
