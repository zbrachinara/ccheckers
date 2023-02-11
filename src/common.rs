use board::Board;
use nannou::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use nannou_egui::Egui;
use player::{Mode, Turn};

mod board;
#[cfg(not(target_arch = "wasm32"))]
mod egui_defs;
mod player;
#[cfg(target_arch = "wasm32")]
mod js_comms;

const HEX_SIZE: f32 = 0.57;

fn viewport_size(app: &App) -> f32 {
    let window_bounds = app.main_window().rect();
    f32::min(window_bounds.w(), window_bounds.h()) / 2.
}

#[derive(Default)]
struct EguiData {
    mode: Mode,
}

#[cfg_attr(target_arch = "wasm32", derive(Default))]
pub struct Model {
    board: Board,
    turn: Turn,
    #[cfg(not(target_arch = "wasm32"))]
    egui: Egui,
    egui_data: EguiData,
    mode: Mode,
}

pub fn window_builder(app: &App) -> window::Builder<'_> {
    app.new_window()
        .view(window_handler)
        .closed(|_, _: &mut Model| std::process::exit(0))
        .raw_event(raw_window_event)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn model(app: &App) -> Model {
    app.set_exit_on_escape(false);

    let window_id = window_builder(app).build().unwrap();
    let window = app.window(window_id).unwrap();

    Model {
        board: Default::default(),
        turn: Default::default(),
        egui: Egui::from_window(&window),
        egui_data: Default::default(),
        mode: Default::default(),
    }
}
#[cfg(target_arch = "wasm32")]
pub async fn model(app: &App) -> Model {
    use nannou::wgpu::{DeviceDescriptor, Limits};
    let device_descriptor = DeviceDescriptor {
        limits: Limits {
            max_texture_dimension_2d: 8192,
            ..Limits::downlevel_webgl2_defaults()
        },
        ..Default::default()
    };
    let window_id = window_builder(app)
        .device_descriptor(device_descriptor)
        .build_async()
        .await
        .unwrap();

    let window = app.window(window_id).unwrap();
    Model::default()
}

fn window_handler(app: &App, m: &Model, f: Frame) {
    let viewport_size = viewport_size(app);

    f.clear(ANTIQUEWHITE);
    let draw = app.draw().scale_axes(Vec3::splat(viewport_size));
    m.board.draw(app, &draw);
    draw.to_frame(app, &f).unwrap();
    #[cfg(not(target_arch = "wasm32"))]
    m.egui.draw_to_frame(&f).unwrap();
}

pub fn update(_app: &App, model: &mut Model, update: Update) {
    #[cfg(not(target_arch = "wasm32"))]
    egui_defs::define_ui(model, &update);
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(mode) = js_comms::recieve_reset() {
            model.mode = mode;
            model.board.reset();
            model.turn = Turn::Player1;
        }
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    #[cfg(not(target_arch = "wasm32"))]
    model.egui.handle_raw_event(event);
}

pub fn events(app: &App, m: &mut Model, e: Event) {
    #[allow(clippy::single_match)]
    match e {
        Event::WindowEvent {
            simple: Some(WindowEvent::MousePressed(MouseButton::Left)),
            ..
        } => {
            if let Some(position) = m.board.position_of(&app.mouse, viewport_size(app)) {
                m.board.try_push_path(position, m.turn, m.mode);
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
