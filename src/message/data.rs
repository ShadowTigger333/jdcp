use byteorder::{LittleEndian, ReadBytesExt};

use super::InfoType;

#[derive(Debug, PartialEq)]
pub enum DataType {
    STATS(StatBlock),
    AGE(u16),
    CLASS(ClassType),
    RACE(RaceKind),
    LEVEL(u8),
    HP(HealthPoints),
}

impl DataType {
    pub fn parse(value: &[u8], info_type: &InfoType) -> Self {
        fn data_age(i: &[u8]) -> u16 {
            let mut data = &i[0..2];
            data.read_u16::<LittleEndian>().expect("Age not read")
        }
        match info_type {
            InfoType::STATS => DataType::STATS(value.into()),
            InfoType::AGE => DataType::AGE(data_age(value)),
            InfoType::CLASS => DataType::CLASS(value.into()),
            InfoType::RACE => DataType::RACE(value.into()),
            InfoType::LEVEL => DataType::LEVEL(value[0]),
            InfoType::HP => DataType::HP(value.into()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct StatBlock {
    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8,
}

impl From<&[u8]> for StatBlock {
    fn from(value: &[u8]) -> Self {
        StatBlock {
            strength: value[0],
            dexterity: value[1],
            constitution: value[2],
            intelligence: value[3],
            wisdom: value[4],
            charisma: value[5],
        }
    }
}

impl StatBlock {
    pub fn new(str: u8, dex: u8, con: u8, int: u8, wis: u8, chr: u8) -> Self {
        StatBlock {
            strength: str,
            dexterity: dex,
            constitution: con,
            intelligence: int,
            wisdom: wis,
            charisma: chr,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ClassType {
    ARTIFACER,
    BARBARIAN,
    BARD,
    BLOODHUNTER,
    CLERIC,
    DRUID,
    FIGHTER,
    MONK,
    PALADIN,
    RANGER,
    ROGUE,
    SORCERER,
    WARLOCK,
    WIZARD,
}

impl From<&[u8]> for ClassType {
    fn from(value: &[u8]) -> Self {
        match value.first() {
            Some(1) => ClassType::ARTIFACER,
            Some(2) => ClassType::BARBARIAN,
            Some(3) => ClassType::BARD,
            Some(4) => ClassType::BLOODHUNTER,
            Some(5) => ClassType::CLERIC,
            Some(6) => ClassType::DRUID,
            Some(7) => ClassType::FIGHTER,
            Some(8) => ClassType::MONK,
            Some(9) => ClassType::PALADIN,
            Some(10) => ClassType::RANGER,
            Some(11) => ClassType::ROGUE,
            Some(12) => ClassType::SORCERER,
            Some(13) => ClassType::WARLOCK,
            Some(14) => ClassType::WIZARD,
            _ => unimplemented!("No other classes currently"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RaceKind {
    DWARF,
    ELF,
    GNOME,
    HALFELF,
    HALFLING,
    HALFORK,
    HUMAN,
    ORC,
    TIEFLING,
}

impl From<&[u8]> for RaceKind {
    fn from(value: &[u8]) -> Self {
        match value.first() {
            Some(1) => RaceKind::DWARF,
            Some(2) => RaceKind::ELF,
            Some(3) => RaceKind::GNOME,
            Some(4) => RaceKind::HALFELF,
            Some(5) => RaceKind::HALFLING,
            Some(6) => RaceKind::HALFORK,
            Some(7) => RaceKind::HUMAN,
            Some(8) => RaceKind::ORC,
            Some(9) => RaceKind::TIEFLING,
            _ => unimplemented!("No other races currently"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct HealthPoints {
    current: u8,
    max: u8,
}

impl From<&[u8]> for HealthPoints {
    fn from(value: &[u8]) -> Self {
        HealthPoints {
            current: value[0],
            max: value[1],
        }
    }
}
