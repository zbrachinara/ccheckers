use board::Board;
use itertools::Itertools;
use nannou::{prelude::*, state::Mouse};
use player::Player;

mod board;
mod player;

const HEX_SIZE: f32 = 0.57;

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

fn viewport_size(app: &App) -> f32 {
    let window_bounds = app.main_window().rect();
    f32::min(window_bounds.w(), window_bounds.h()) / 2.
}

#[derive(Default)]
struct Model {
    board: Board,
    path: Vec<IVec2>,
    turn: Player,
}

fn main() {
    nannou::app(model)
        .simple_window(window_handler)
        .event(update)
        .run()
}

fn model(_: &App) -> Model {
    Model::default()
}

fn window_handler(app: &App, m: &Model, f: Frame) {
    let viewport_size = viewport_size(app);

    f.clear(ANTIQUEWHITE);
    let draw = app.draw().scale_axes(Vec3::splat(viewport_size));
    draw_board(&draw);
    m.board.draw(&draw);
    draw.to_frame(app, &f).unwrap();
}

fn update(app: &App, m: &mut Model, e: Event) {
    #[allow(clippy::single_match)]
    match e {
        Event::WindowEvent {
            simple: Some(WindowEvent::MousePressed(MouseButton::Left)),
            ..
        } => {
            if let Some(position) = m.board.position_of(&app.mouse, viewport_size(app)) {
                let legal = if let Some(&recent) = m.path.last() {
                    m.board.is_legal(recent, position)
                } else {
                    m.board.get(&position).map(|p| p == m.turn).unwrap_or(false)
                };

                if legal {
                    m.path.push(position);
                }
            }
        }
        _ => (),
    }
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
