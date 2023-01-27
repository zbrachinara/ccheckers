use std::collections::HashMap;

use itertools::Itertools;
use nannou::{color::Alpha, prelude::*, state::Mouse};

use crate::{
    player::{Mode, Player},
    HEX_SIZE,
};

/// "Divides" v1 by v2
fn divide(v1: IVec2, v2: IVec2) -> Option<i32> {
    v1.x.checked_div(v2.x)
        .or_else(|| v1.y.checked_div(v2.y))
        .and_then(|r| (r > 0 && v2.x * r == v1.x && v2.y * r == v1.y).then_some(r))
}

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
    path: Vec<IVec2>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            backing: {
                let center = (-4..5)
                    .cartesian_product(-4..5)
                    .map(|(a, b)| IVec2::new(a, b));

                center
                    .chain(Self::region_2())
                    .chain(Self::region_3())
                    .chain(Self::region_5())
                    .chain(Self::region_6())
                    .map(|v| (v, Player::None))
                    .collect()
            },
            path: Default::default(),
        }
    }
}

/// Iterators for each home region of the board
impl Board {
    fn region_1() -> impl Iterator<Item = IVec2> {
        (0..5).flat_map(|x| (5 - x..5).map(move |y| ivec2(x, y)))
    }
    fn region_2() -> impl Iterator<Item = IVec2> {
        (5..9).flat_map(|y| (-4..(5 - y)).map(move |x| ivec2(x, y)))
    }
    fn region_3() -> impl Iterator<Item = IVec2> {
        (5..9).flat_map(|x| (-4..(5 - x)).map(move |y| ivec2(x, y)))
    }

    fn region_4() -> impl Iterator<Item = IVec2> {
        Self::region_1().map(|v| -v)
    }
    fn region_5() -> impl Iterator<Item = IVec2> {
        Self::region_2().map(|v| -v)
    }
    fn region_6() -> impl Iterator<Item = IVec2> {
        Self::region_3().map(|v| -v)
    }
}

impl Board {
    const BASE_SPACING: f32 = 0.04;
    const WIDTH: f32 = (HEX_SIZE - Self::BASE_SPACING * 4.5) / 9.0;
    const SPACING: f32 = Self::BASE_SPACING + Self::WIDTH * 2.0;

    pub fn bases() -> (Vec2, Vec2) {
        let unit = Vec2::new(Self::SPACING, 0.0);
        (unit, unit.rotate(f32::FRAC_PI_3()))
    }

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

    pub fn cardinal_distance(v1: IVec2, v2: IVec2) -> Option<(IVec2, i32)> {
        let dv = v2 - v1;
        Self::cardinals()
            .into_iter()
            .find_map(|cardinal| divide(dv, cardinal).map(|div| (cardinal, div)))
    }

    fn fill_area(&mut self, positions: impl Iterator<Item = IVec2>, piece: Player) {
        for p in positions {
            *self.backing.get_mut(&p).unwrap() = piece;
        }
    }

    pub fn fill(&mut self, mode: Mode) {
        for v in self.backing.values_mut() {
            *v = Player::None;
        }
        match mode {
            Mode::Two => {
                self.fill_area(
                    Self::region_1()
                        .chain(Self::region_2())
                        .chain(Self::region_3()),
                    Player::Player1,
                );
                self.fill_area(
                    Self::region_4()
                        .chain(Self::region_5())
                        .chain(Self::region_6()),
                    Player::Player2,
                );
            }
            Mode::Three => {
                self.fill_area(Self::region_1().chain(Self::region_2()), Player::Player1);
                self.fill_area(Self::region_3().chain(Self::region_4()), Player::Player2);
                self.fill_area(Self::region_5().chain(Self::region_6()), Player::Player3);
            }
        }
    }

    pub fn move_piece(&mut self, from: &IVec2, to: &IVec2) {
        *self.backing.get_mut(to).unwrap() = std::mem::take(self.backing.get_mut(from).unwrap())
    }

    /// Converts a board position into a viewport position
    pub fn physical_position(point: &IVec2) -> Point2 {
        let (bx, by) = Self::bases();
        bx * point.x as f32 + by * point.y as f32
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

    /// Checks if moving from the first to the second position is a legal jump, taking into account
    /// the full path. Both positions given must be valid positions on the board.
    pub fn is_legal(&self, new: IVec2, turn: Player) -> bool {
        if let Some(&starts) = self.path.last() {
            self.backing.get(&new).unwrap() == &Player::None
                && match Self::cardinal_distance(starts, new) {
                    Some((_, x)) if x == 1 => self.path.len() == 1,
                    Some((cardinal, x)) if x == 2 => {
                        if self.path.len() > 1
                            && Self::cardinal_distance(
                                *self.path.get(0).unwrap(),
                                *self.path.get(1).unwrap(),
                            )
                            .unwrap()
                            .1 == 1
                        {
                            return false;
                        }
                        self.backing.get(&(starts + cardinal)).unwrap() != &Player::None
                    }
                    _ => false,
                }
        } else {
            self.get(&new).map(|p| p == turn).unwrap_or(false)
        }
    }

    pub fn try_push_path(&mut self, new: IVec2, turn: Player) -> bool {
        if self.is_legal(new, turn) {
            self.path.push(new);
            true
        } else {
            false
        }
    }

    pub fn pop_path(&mut self) {
        self.path.pop();
    }

    /// If the path is long enough to move, does the move and returns true. Otherwise does nothing
    /// and returns false.
    pub fn commit_path(&mut self) -> bool {
        if self.path.len() > 1 {
            let (first, last) = (*self.path.first().unwrap(), *self.path.last().unwrap());
            self.move_piece(&first, &last);
            self.path.clear();
            true
        } else {
            false
        }
    }
}

impl Board {
    pub fn draw(&self, draw: &Draw) {
        Self::draw_board_background(draw);
        self.draw_pieces(draw);
        self.draw_path(draw);
    }

    fn draw_board_background(draw: &Draw) {
        let hex_coords = (0..)
            .map(|i| f32::PI() * i as f32 / 3.0)
            .map(|rad| pt2(rad.cos(), rad.sin()) * HEX_SIZE);

        let hex = hex_coords.clone().take(6);
        draw.polygon().points(hex);

        hex_coords.tuple_windows().take(6).for_each(|(a, b)| {
            draw.polygon().color(STEELBLUE).points([a, b, a + b]);
        });
    }

    fn draw_pieces(&self, draw: &Draw) {
        for (pos, state) in &self.backing {
            draw.ellipse()
                .color(Rgb::<u8>::from(*state))
                .xy(Self::physical_position(pos))
                .radius(Self::WIDTH)
                .finish();
        }
    }

    fn draw_path(&self, draw: &Draw) {
        for point in &self.path {
            draw.ellipse()
                .color(Alpha::<Rgb<_>, _>::new(0.0, 0.0, 0.0, 0.5))
                .radius(Self::WIDTH + Self::BASE_SPACING / 5.0)
                .xy(Self::physical_position(point));
        }
        for (p1, p2) in self.path.iter().tuple_windows() {
            draw.line()
                .start(Board::physical_position(p1))
                .end(Board::physical_position(p2))
                .weight(0.01)
                .color(RED);
        }
    }
}
