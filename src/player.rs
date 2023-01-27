use nannou::{
    color::encoding::{srgb, Linear},
    prelude::*,
};
use strum_macros::{Display, EnumIter};

#[derive(Copy, Clone, PartialEq, Eq, Default, Debug, EnumIter, Display)]
pub enum Mode {
    #[default]
    Two,
    Three,
}

#[derive(Copy, Clone, PartialEq, Eq, Default, Display, EnumIter)]
pub enum Player {
    #[default]
    None,
    Player1,
    Player2,
    Player3,
    Player4,
    Player5,
    Player6,
}

impl From<Player> for rgb::Rgb<Linear<srgb::Srgb>> {
    fn from(value: Player) -> Self {
        match value {
            Player::None => DARKSLATEGRAY,
            Player::Player1 => DARKORCHID,
            Player::Player2 => FUCHSIA,
            Player::Player3 => YELLOWGREEN,
            Player::Player4 => MAROON,
            Player::Player5 => MIDNIGHTBLUE,
            Player::Player6 => OLIVE,
        }
        .into_format::<f32>()
        .into_linear()
    }
}

impl Player {
    pub fn is_none(&self) -> bool {
        matches!(self, Player::None)
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
            },
            Mode::Three => match turn {
                Player::None => panic!("Wrong turn: Should not be possible to reach ingame"),
                Player::Player1 => Player::Player2,
                Player::Player2 => Player::Player3,
                Player::Player3 => Player::Player1,
                _ => Player::Player1,
            },
        }
    }
}
