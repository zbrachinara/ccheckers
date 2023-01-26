use std::collections::HashMap;

use itertools::Itertools;
use nannou::{prelude::*, state::Mouse};

use crate::{player::Player, HEX_SIZE};

/// Board is a 2d representation of the hexagonal grid, where the horizontal component remains as is
/// and the "vertical" component is the rightward shearing line. The leftward shearing line can be
/// represented using left diagonal lines drawn through the "gridlines". Right diagnonal lines
/// should *not* be used for any purpose regarding "local" (regarding neighboring cells) behavior.
///
/// The center of the coordinate system refers to the cell in the direct center of the board. From
/// there, the point (0, 4) is in the top right corner within the central hexagon, while (0, -4) is
/// in the bottom left corner of the hexagon. Similarly, the point at (4, 0) is at the right, while
/// (-4, 0) is in the left. By this, the top left is at (-4, 4) and the bottom right is at (4, -4)
pub struct Board {
    backing: HashMap<IVec2, Player>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            backing: {
                let center = (-4..5)
                    .cartesian_product(-4..5)
                    .map(|(a, b)| IVec2::new(a, b));
                let following_y = (5..9)
                    .flat_map(|y| (-4..(5 - y)).map(move |x| IVec2::new(x, y)))
                    .flat_map(|v| [v, -v]);
                let following_x = (5..9)
                    .flat_map(|x| (-4..(5 - x)).map(move |y| IVec2::new(x, y)))
                    .flat_map(|v| [v, -v]);

                center
                    .chain(following_y)
                    .chain(following_x)
                    .map(|v| (v, Player::None))
                    .collect()
            },
        }
    }
}

impl Board {
    const BASE_SPACING: f32 = 0.04;
    const WIDTH: f32 = (HEX_SIZE - Self::BASE_SPACING * 4.5) / 9.0;
    const SPACING: f32 = Self::BASE_SPACING + Self::WIDTH * 2.0;

    fn bases() -> (Vec2, Vec2) {
        let unit = Vec2::new(Self::SPACING, 0.0);
        (unit, unit.rotate(f32::FRAC_PI_3()))
    }

    /// Returns all cardinals in
    fn cardinals() -> [IVec2; 6] {
        [
            IVec2::X,
            IVec2::Y,
            IVec2::new(-1, 1),
            -IVec2::X,
            -IVec2::Y,
            IVec2::new(1, -1),
        ]
    }

    pub fn draw(&self, draw: &Draw) {
        let (bx, by) = Self::bases();

        for (pos, state) in &self.backing {
            let physical_position = bx * pos.x as f32 + by * pos.y as f32;
            draw.ellipse()
                .color(Rgb::<u8>::from(*state))
                .x_y(physical_position.x, physical_position.y)
                .radius(Self::WIDTH)
                .finish();
        }
    }

    /// Converts the screen position (say, of the cursor) into a position on the board, if the
    /// position is within the board's bounds.
    pub fn position_of(&self, mouse: &Mouse, scale: f32) -> Option<IVec2> {
        let (bx, by) = Self::bases();
        let inverter = mat2(bx, by).inverse();
        let predicted_f32 = inverter * (mouse.position() / scale) + Point2::ONE / 2.;
        let predicted = predicted_f32.floor().as_i32();

        self.backing.contains_key(&predicted).then_some(predicted)
    }

    pub fn get(&self, position: &IVec2) -> Option<Player> {
        self.backing.get(position).copied()
    }

    /// Checks if moving from the first to the second position is a legal jump (does not calculate a
    /// "series" of jumps). Both positions given must
    /// be valid positions on the board.
    pub fn is_legal(&self, starts: IVec2, ends: IVec2) -> bool {
        Self::cardinals().into_iter().find_map(|cardinal| {
            if starts + cardinal == ends {
                Some(true)
            } else if starts + 2 * cardinal == ends {
                Some(self.backing.get(&(starts + cardinal)).unwrap() != &Player::None)
            } else {
                None
            }
        }).unwrap_or(false)
    }
}
