use std::collections::HashMap;

use itertools::Itertools;
use nannou::{prelude::*, glam::XY};

const HEX_SIZE: f32 = 0.57;

#[derive(Copy, Clone)]
enum Player {
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

/// Finds out if the given position is in an "outer region" of the playing board, and if so, which
/// one it is in. An outer region is a region which contains positions that don't fall on the
/// central hexagon. These outer regions are indexed roughly corresponding to which edge of the
/// hexagon the region is adjacent to, counting counterclockwise in the usual way. For example, if a
/// point lies directly above the top edge of the hexagon, its region is given the index `1`, since
/// the previous edge, the first one starting from the x axis, was given the index `0`.
///
/// The outer regions do not contain every point outside of the hexagon. Instead, they are defined
/// so that every point that is within a player's home is part of an outer region. This means that
/// if the position that you pass in is not a valid playing position, you may get a nonsensical
/// answer.
fn outer_region_of(position: IVec2) -> Option<u8> {
    let (x, y) = (position.x, position.y);

    const BOARD_SIZE: i32 = 4; // the minimum amount of positions to walk until you reach the edge of the board
    if x > 0 && y > 0 && x + y > BOARD_SIZE {
        Some(0)
    } else if -BOARD_SIZE < x && x < 0 && y > BOARD_SIZE {
        Some(1)
    } else if todo!() {
        Some(2)
    } else if todo!() {
        Some(3)
    } else if todo!() {
        Some(4)
    } else if todo!() {
        Some(5)
    } else {
        None
    }
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
struct Board {
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
    
    pub fn bases() -> (Vec2, Vec2) {
        let unit = Vec2::new(Self::SPACING, 0.0);
        (unit,unit.rotate(f32::FRAC_PI_3()))
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

    /// Reverses the screen position (say, of the cursor) into a position on the board, if the
    /// position is within bounds.
    pub fn position_of(point: Point2) -> Option<IVec2> {
        // let predicted_point = 
        todo!()
    }
}

#[derive(Default)]
struct Model {
    board: Board,
    path: Vec<IVec2>,
}

fn main() {
    nannou::app(model)
        .simple_window(window_handler)
        .update(update)
        .run()
}

fn model(_: &App) -> Model {
    Model::default()
}

fn window_handler(app: &App, m: &Model, f: Frame) {
    let window_bounds = app.main_window().rect();
    let viewport_size = f32::min(window_bounds.w(), window_bounds.h()) / 2.;

    f.clear(ANTIQUEWHITE);
    let draw = app.draw().scale_axes(Vec3::splat(viewport_size));
    draw_board(&draw);
    m.board.draw(&draw);
    draw.to_frame(app, &f).unwrap();
}

fn update(app: &App, _: &mut Model, _: Update) {
    dbg!(&app.mouse);
}

fn draw_board(draw: &Draw) {
    let hex_coords = (0..)
        .map(|i| f32::PI() * i as f32 / 3.0)
        .map(|rad| pt2(rad.cos(), rad.sin()) * HEX_SIZE);

    let hex = hex_coords.clone().take(6);
    draw.polygon().points(hex);

    hex_coords.tuple_windows().take(6).for_each(|(a, b)| {
        draw.polygon().color(STEELBLUE).points([a, b, a + b]);
    });
}
