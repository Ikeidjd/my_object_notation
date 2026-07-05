extern crate self as my_object_notation;

mod text;
mod mon_error;
mod token;
mod lexer;
mod mon_object;
mod parser;
mod serializable;

#[cfg(test)]
mod tests {
    use std::error::Error;

use proc_macros::Serializable;

    use crate::{mon_object::MonObject, serializable::Serializable};

    #[derive(Serializable)]
    struct Player {
        name: String,
        max_health: i32,
        health: i32,
        can_double_jump: bool,
        god_mode: bool,
        power_ups: Vec<PowerUp>,
        weapon: Option<String>,
        alt_weapon: Option<String>,
    }

    #[derive(Serializable)]
    struct PowerUp {
        level: u8,
        xp: u16,
        ttype: PowerUpType,
    }

    impl PowerUp {
        fn new(level: u8, xp: u16, ttype: PowerUpType) -> Self {
            Self {
                level,
                xp,
                ttype,
            }
        }
    }

    #[derive(Serializable)]
    enum PowerUpType {
        Fire { length: usize },
        Shadow(usize),
        Water,
    }

    #[test]
    fn serialize_test() -> Result<(), Box<dyn Error>> {
        let player = Player {
            name: "Jon".to_owned(),
            max_health: 10,
            health: 3,
            can_double_jump: true,
            god_mode: false,
            power_ups: vec![
                PowerUp::new(1, 100, PowerUpType::Fire { length: 10 }),
                PowerUp::new(2, 200, PowerUpType::Shadow(20)),
                PowerUp::new(3, 300, PowerUpType::Water),
            ],
            weapon: Some("CopperShortsword".to_owned()),
            alt_weapon: None,
        };

        assert_eq!(player.serialize(), MonObject::from_file("res/examples/player.mon")?);

        Ok(())
    }
}
