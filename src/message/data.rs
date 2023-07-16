#[derive(Debug, PartialEq)]
pub enum DataType<'a> {
    STATS(StatBlock),
    AGE(u16),
    CLASS(Result<ClassType, &'a str>),
    RACE(Result<RaceKind, &'a str>),
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

#[derive(Debug, PartialEq)]
pub struct HealthPoints {
    pub current: u8,
    pub max: u8,
}
