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
    Six,
}

#[derive(Copy, Clone, PartialEq, Eq, Default, Display)]
pub enum Turn {
    #[default]
    None,
    Player1,
    Player2,
    Player3,
    Player4,
    Player5,
    Player6,
}

impl Turn {
    pub fn owns(&self, piece: Piece, mode: Mode) -> bool {
        match mode {
            Mode::Two => {
                *self
                    == match piece {
                        Piece::None => return false,
                        Piece::Player1 | Piece::Player2 | Piece::Player6 => Turn::Player1,
                        _ => Turn::Player2,
                    }
            }
            Mode::Three => {
                *self
                    == match piece {
                        Piece::None => return false,
                        Piece::Player1 | Piece::Player2 => Turn::Player1,
                        Piece::Player3 | Piece::Player4 => Turn::Player2,
                        Piece::Player5 | Piece::Player6 => Turn::Player3,
                    }
            }
            Mode::Six => {
                *self
                    == match piece {
                        Piece::None => return false,
                        Piece::Player1 => Turn::Player1,
                        Piece::Player2 => Turn::Player2,
                        Piece::Player3 => Turn::Player3,
                        Piece::Player4 => Turn::Player4,
                        Piece::Player5 => Turn::Player5,
                        Piece::Player6 => Turn::Player6,
                    }
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Default, Display, EnumIter)]
pub enum Piece {
    #[default]
    None,
    Player1,
    Player2,
    Player3,
    Player4,
    Player5,
    Player6,
}

impl From<Piece> for rgb::Rgb<Linear<srgb::Srgb>> {
    fn from(value: Piece) -> Self {
        match value {
            Piece::None => DARKSLATEGRAY,
            Piece::Player1 => DARKORCHID,
            Piece::Player2 => FUCHSIA,
            Piece::Player3 => YELLOWGREEN,
            Piece::Player4 => MAROON,
            Piece::Player5 => MIDNIGHTBLUE,
            Piece::Player6 => OLIVE,
        }
        .into_format::<f32>()
        .into_linear()
    }
}

impl Piece {
    pub fn is_none(&self) -> bool {
        matches!(self, Piece::None)
    }
}

impl Mode {
    pub fn next_turn(&self, turn: Turn) -> Turn {
        match self {
            Mode::Two => match turn {
                Turn::None => panic!("Wrong turn: Should not be possible to reach ingame"),
                Turn::Player1 => Turn::Player2,
                Turn::Player2 => Turn::Player1,
                _ => Turn::Player1,
            },
            Mode::Three => match turn {
                Turn::None => panic!("Wrong turn: Should not be possible to reach ingame"),
                Turn::Player1 => Turn::Player2,
                Turn::Player2 => Turn::Player3,
                Turn::Player3 => Turn::Player1,
                _ => Turn::Player1,
            },
            Mode::Six => match turn {
                Turn::None => panic!("Wrong turn: Should not be possible to reach ingame"),
                Turn::Player1 => Turn::Player2,
                Turn::Player2 => Turn::Player3,
                Turn::Player3 => Turn::Player4,
                Turn::Player4 => Turn::Player5,
                Turn::Player5 => Turn::Player6,
                Turn::Player6 => Turn::Player1,
            },
        }
    }
}
