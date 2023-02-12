use board::Board;
use nannou::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use nannou_egui::Egui;

mod board;
#[cfg(not(target_arch = "wasm32"))]
mod egui_defs;
#[cfg(target_arch = "wasm32")]
mod js_comms;
mod player;

const HEX_SIZE: f32 = 0.57;

fn viewport_size(app: &App) -> f32 {
    let window_bounds = app.main_window().rect();
    f32::min(window_bounds.w(), window_bounds.h()) / 2.
}

#[cfg_attr(target_arch = "wasm32", derive(Default))]
pub struct Model {
    board: Board,
    #[cfg(not(target_arch = "wasm32"))]
    egui: Egui,
    #[cfg(not(target_arch = "wasm32"))]
    egui_data: egui_defs::EguiData,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn model(app: &App) -> Model {
    app.set_exit_on_escape(false);

    let window_id = app
        .new_window()
        .view(window_handler)
        .closed(|_, _: &mut Model| std::process::exit(0))
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    Model {
        board: Default::default(),
        egui: Egui::from_window(&window),
        egui_data: Default::default(),
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
    app.new_window()
        .device_descriptor(device_descriptor)
        .view(window_handler)
        .build_async()
        .await
        .unwrap();
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

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    #[cfg(not(target_arch = "wasm32"))]
    egui_defs::define_ui(model, &_update);
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(mode) = js_comms::recieve_reset() {
            model.board.reset(mode);
        }
        js_comms::respond_load_request(model);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

pub fn events(app: &App, m: &mut Model, e: Event) {
    #[allow(clippy::single_match)]
    if let Event::WindowEvent {
        simple: Some(ev), ..
    } = e
    {
        match ev {
            WindowEvent::MousePressed(MouseButton::Left) => {
                if let Some(position) = m.board.position_of(&app.mouse, viewport_size(app)) {
                    m.board.try_push_path(position);
                }
            }
            WindowEvent::KeyPressed(Key::Return) => m.board.commit_path(),
            WindowEvent::KeyPressed(Key::Left) => m.board.pop_path(),
            _ => (),
        }
    }
}
