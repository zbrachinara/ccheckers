use nannou::prelude::*;
use strum_macros::{Display, EnumIter};

#[derive(Copy, Clone, PartialEq, Eq, Default, Debug, EnumIter, Display)]
pub enum Mode {
    #[default]
    Two,
    Three,
}

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum Player {
    #[default]
    None,
    Player1,
    Player2,
    Player3,
}

impl From<Player> for Rgb<u8> {
    fn from(value: Player) -> Self {
        match value {
            Player::None => DARKSLATEGRAY,
            Player::Player1 => DARKORCHID,
            Player::Player2 => FUCHSIA,
            Player::Player3 => HONEYDEW,
        }
    }
}

impl Mode {
    pub fn next_turn(&self, turn: Player) -> Player {
        match self {
            Mode::Two => match turn {
                Player::None => panic!("Wrong turn: Should not be possible to reach ingame"),
                Player::Player1 => Player::Player2,
                Player::Player2 => Player::Player1,
                _ => Player::Player1,
            }
            Mode::Three => match turn {
                Player::None => panic!("Wrong turn: Should not be possible to reach ingame"),
                Player::Player1 => Player::Player2,
                Player::Player2 => Player::Player3,
                Player::Player3 => Player::Player1,
                _ => Player::Player1,
            }
        }
    }
}
