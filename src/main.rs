use nannou::prelude::*;

struct Model {

}

fn main() {
    // nannou::sketch().simple_window(window_handler).run()
    nannou::sketch(window_handler).run();
}

fn model(app: &App) -> Model {
    Model {}
}

fn window_handler(app: &App, f: Frame) {

    f.clear(ANTIQUEWHITE);

    let draw = app.draw();
    // draw.tri().color(STEELBLUE).points(pt2(10., 10.), pt2(-10., 10.), pt2(0., -10.)).finish();
    draw_board(&draw);

    draw.to_frame(app, &f).unwrap();

}

fn draw_board(draw: &Draw) {
    let hex_size = 100.0;
    let hex = (0..6).map(|i| f32::PI()  * i as f32 / 3.0).map(|rad| pt2(rad.cos() * hex_size, rad.sin() * hex_size));
    draw.polygon().points(hex);
}