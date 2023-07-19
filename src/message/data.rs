#[derive(Debug, PartialEq)]
pub enum DataType {
    STATS(StatBlock),
    AGE(u16),
    CLASS(ClassType),
    RACE(RaceKind),
    LEVEL(u8),
    HP(HealthPoints),
}

#[derive(Debug, PartialEq)]
pub struct StatBlock {
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,
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
    ARTIFACER = 1,
    BARBARIAN = 2,
    BARD = 3,
    BLOODHUNTER = 4,
    CLERIC = 5,
    DRUID = 6,
    FIGHTER = 7,
    MONK = 8,
    PALADIN = 9,
    RANGER = 10,
    ROGUE = 11,
    SORCERER = 12,
    WARLOCK = 13,
    WIZARD = 14,
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

impl ClassType {
    pub fn discriminant(&self) -> u8 {
        unsafe { *(self as *const Self as *const u8) }
    }
}

#[derive(Debug, PartialEq)]
pub enum RaceKind {
    DWARF = 1,
    ELF = 2,
    GNOME = 3,
    HALFELF = 4,
    HALFLING = 5,
    HALFORK = 6,
    HUMAN = 7,
    ORC = 8,
    TIEFLING = 9,
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

impl RaceKind {
    pub fn discriminant(&self) -> u8 {
        unsafe { *(self as *const Self as *const u8) }
    }
}

#[derive(Debug, PartialEq)]
pub struct HealthPoints {
    pub current: u8,
    pub max: u8,
}

impl From<&[u8]> for HealthPoints {
    fn from(value: &[u8]) -> Self {
        HealthPoints {
            current: value[0],
            max: value[1],
        }
    }
}
