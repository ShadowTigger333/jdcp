#[derive(Debug, PartialEq)]
pub enum RaceKind {
    DWARF = 1,
    ELF = 2,
    GNOME = 3,
    HALFELF = 4,
    HALFLING = 5,
    HALFORC = 6,
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
            Some(6) => RaceKind::HALFORC,
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
