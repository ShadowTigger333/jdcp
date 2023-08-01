use self::{
    class_type::ClassType, health_points::HealthPoints, race_kind::RaceKind, stat_block::StatBlock,
};

pub mod class_type;
pub mod health_points;
pub mod race_kind;
pub mod stat_block;

#[derive(Debug, PartialEq)]
pub enum CharacterData {
    STATS(StatBlock),
    AGE(u16),
    CLASS(ClassType),
    RACE(RaceKind),
    LEVEL(u8),
    HP(HealthPoints),
}