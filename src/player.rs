use nannou::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum Mode {
    #[default]
    None,
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
