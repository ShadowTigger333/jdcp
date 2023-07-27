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
