use self::character_data::{
    class_type::ClassType, health_points::HealthPoints, race_kind::RaceKind, stat_block::StatBlock,
};

pub mod character_data;

#[derive(PartialEq, Debug)]
struct Character {
    name: String,
    race: RaceKind,
    class: ClassType,
    level: u8,
    stats: StatBlock,
    hp: HealthPoints,
    age: u16,
    description: String,
}

impl Character {
    fn new(
        name: String,
        race: RaceKind,
        class: ClassType,
        level: u8,
        stats: StatBlock,
        hp: HealthPoints,
        age: u16,
        description: String,
    ) -> Character {
        Character {
            name,
            race,
            class,
            level,
            stats,
            hp,
            age,
            description,
        }
    }
}

#[cfg(test)]
mod character_tests {
    use super::*;

    #[test]
    fn character_new_functions() {
        let character = Character::new(
            "Sally".to_string(),
            RaceKind::GNOME,
            ClassType::ROGUE,
            6u8,
            StatBlock {
                strength: 13,
                dexterity: 17,
                constitution: 15,
                intelligence: 14,
                wisdom: 12,
                charisma: 10,
            },
            HealthPoints {
                current: 56,
                max: 60,
            },
            2420u16,
            "Some weird description".to_string(),
        );
        assert_eq!(
            character,
            Character {
                name: "Sally".to_string(),
                race: RaceKind::GNOME,
                class: ClassType::ROGUE,
                level: 6u8,
                stats: StatBlock {
                    strength: 13,
                    dexterity: 17,
                    constitution: 15,
                    intelligence: 14,
                    wisdom: 12,
                    charisma: 10
                },
                hp: HealthPoints {
                    current: 56,
                    max: 60
                },
                age: 2420u16,
                description: "Some weird description".to_string()
            }
        )
    }
}
