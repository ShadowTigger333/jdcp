#[derive(Debug, PartialEq)]
pub enum ClassType {
    ARTIFICER = 1,
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
            Some(1) => ClassType::ARTIFICER,
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
