use itertools::Itertools;
use nannou::prelude::*;

struct Model {}

fn main() {
    nannou::app(model).simple_window(window_handler).run()
    // nannou::sketch(window_handler).run();
}

fn model(_: &App) -> Model {
    Model {}
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
    let hex_size = 0.4;
    let hex_coords = (0..)
        .map(|i| f32::PI() * i as f32 / 3.0)
        .map(|rad| pt2(rad.cos(), rad.sin()) * hex_size);
        

    let hex = hex_coords.clone().take(6);
    draw.polygon().points(hex);

    hex_coords.tuple_windows().take(6).for_each(|(a, b)| {
        draw.polygon().color(STEELBLUE).points([a, b, a + b]);
    });
}
