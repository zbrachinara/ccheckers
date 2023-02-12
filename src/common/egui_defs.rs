use super::{
    player::{Mode, Turn},
    Model,
};
use nannou::prelude::*;
use nannou_egui::egui;
use strum::IntoEnumIterator;

#[derive(Default)]
pub struct EguiData {
    mode: Mode,
}

pub fn define_ui(model: &mut Model, update: &Update) {
    let Model { ref mut egui, .. } = *model;
    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("ChuFEUNieSE CHEikcERsS????").show(&ctx, |ui| {
        ui.label("Controls:");
        ui.label("Click positions to begin a move");
        ui.label("Left arrow to undo part of a move");
        ui.label("Press enter to finish a move");
        egui::ComboBox::from_label("#Players")
            .selected_text(format!("{}", model.egui_data.mode))
            .show_ui(ui, |ui| {
                for mode in Mode::iter() {
                    ui.selectable_value(&mut model.egui_data.mode, mode, format!("{mode}"));
                }
            });
        if ui.button("Reset field").clicked() {
            model.board.reset(model.egui_data.mode);
        }

        if model.board.turn != Turn::None {
            ui.label(format!("Currently {}'s turn", model.board.turn));
        }
    });
}
