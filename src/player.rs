use nannou::prelude::*;
use strum_macros::{EnumIter, Display};

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
