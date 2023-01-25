use std::collections::HashMap;

use itertools::Itertools;
use nannou::prelude::*;

const HEX_SIZE: f32 = 0.4;

enum Player {
    None,
    Player1,
    Player2,
    Player3,
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
            backing: Default::default(),
        }
    }
}

impl Board {
    pub fn draw(&self, draw: &Draw) {}
}

#[derive(Default)]
struct Model {
    board: Board,
}

fn main() {
    nannou::app(model).simple_window(window_handler).run()
}

fn model(_: &App) -> Model {
    Model::default()
}

fn window_handler(app: &App, _: &Model, f: Frame) {
    let window_bounds = app.main_window().rect();
    let viewport_size = f32::min(window_bounds.w(), window_bounds.h()) / 2.;

    f.clear(ANTIQUEWHITE);
    let draw = app.draw().scale_axes(Vec3::splat(viewport_size));
    draw_board(&draw);
    draw.to_frame(app, &f).unwrap();
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
