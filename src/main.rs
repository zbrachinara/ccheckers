use board::Board;
use nannou::prelude::*;
use nannou_egui::{egui, Egui};
use player::{Mode, Player};
use strum::IntoEnumIterator;

mod board;
mod player;

const HEX_SIZE: f32 = 0.57;

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
    m.board.draw(&draw);
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
                m.board.try_push_path(position, m.turn);
            }
        }
        Event::WindowEvent {
            simple: Some(WindowEvent::KeyPressed(Key::Return)),
            ..
        } => {
            if m.board.commit_path() {
                m.turn = m.mode.next_turn(m.turn)
            }
        }
        Event::WindowEvent {
            simple: Some(WindowEvent::KeyPressed(Key::Left)),
            ..
        } => {
            m.board.pop_path();
        }
        _ => (),
    }
}
