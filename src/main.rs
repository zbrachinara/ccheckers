use board::Board;
use itertools::Itertools;
use nannou::prelude::*;
use nannou_egui::{egui, Egui};
use player::{Mode, Player};
use strum::IntoEnumIterator;

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
struct EguiData {
    mode: Mode,
}

struct Model {
    board: Board,
    path: Vec<IVec2>,
    turn: Player,
    egui: Egui,
    egui_data: EguiData,
    mode: Mode,
}

fn main() {
    nannou::app(model).event(events).update(update).run()
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(window_handler)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    Model {
        board: Default::default(),
        path: Default::default(),
        turn: Default::default(),
        egui: Egui::from_window(&window),
        egui_data: Default::default(),
        mode: Default::default(),
    }
}

fn window_handler(app: &App, m: &Model, f: Frame) {
    let viewport_size = viewport_size(app);

    f.clear(ANTIQUEWHITE);
    let draw = app.draw().scale_axes(Vec3::splat(viewport_size));
    draw_board_back(&draw);
    m.board.draw(&draw);
    draw_path(m, &draw);
    draw.to_frame(app, &f).unwrap();
    m.egui.draw_to_frame(&f).unwrap();
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let Model { ref mut egui, .. } = *model;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();
    // define ui
    egui::Window::new("ChuFEUNieSE CHEikcERsS????").show(&ctx, |ui| {
        egui::ComboBox::from_label("#Players")
            .selected_text(format!("{}", model.egui_data.mode))
            .show_ui(ui, |ui| {
                for mode in Mode::iter() {
                    ui.selectable_value(&mut model.egui_data.mode, mode, format!("{mode}"));
                }
            });
        if ui.button("Reset field").clicked() {
            model.mode = model.egui_data.mode;
            model.board.fill(model.mode);
            model.turn = Player::Player1;
        }

        if model.turn != Player::None {
            ui.label(format!("Currently {}'s turn", model.turn));
        }
    });
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn events(app: &App, m: &mut Model, e: Event) {
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
        Event::WindowEvent {
            simple: Some(WindowEvent::KeyPressed(Key::Return)),
            ..
        } => {
            if m.path.len() > 1 {
                m.board
                    .move_piece(m.path.first().unwrap(), m.path.last().unwrap());
                m.path.clear();
                m.turn = m.mode.next_turn(m.turn)
            }
        }
        _ => (),
    }
}

fn draw_path(model: &Model, draw: &Draw) {
    for (p1, p2) in model.path.iter().tuple_windows() {
        draw.line()
            .start(Board::physical_position(p1))
            .end(Board::physical_position(p2))
            .weight(0.01)
            .color(RED);
    }
}

fn draw_board_back(draw: &Draw) {
    let hex_coords = (0..)
        .map(|i| f32::PI() * i as f32 / 3.0)
        .map(|rad| pt2(rad.cos(), rad.sin()) * HEX_SIZE);

    let hex = hex_coords.clone().take(6);
    draw.polygon().points(hex);

    hex_coords.tuple_windows().take(6).for_each(|(a, b)| {
        draw.polygon().color(STEELBLUE).points([a, b, a + b]);
    });
}
