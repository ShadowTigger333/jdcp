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
