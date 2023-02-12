use std::{
    fs::{self, File},
    io::Write,
};

use super::{
    board::Board,
    player::{Mode, Turn},
    Model,
};
use nannou::prelude::*;
use nannou_egui::egui;
use rfd::FileDialog;
use strum::IntoEnumIterator;

#[derive(Default)]
pub struct EguiData {
    mode: Mode,
}

fn load_game() -> Option<Board> {
    FileDialog::new()
        .pick_file()
        .and_then(|path| fs::read_to_string(path).ok())
        .and_then(|str| ron::from_str(&str).ok())
}

fn save_game(board: &Board) {
    if let Some(file_handle) = FileDialog::new()
        .set_file_name("unnamed_ccheckers_game.ron")
        .save_file()
    {
        if let Ok(mut f) = File::create(file_handle) {
            f.write_all(ron::to_string(board).unwrap().as_bytes());
        }
    }
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
        ui.horizontal(|ui| {
            if ui.button("Reset field").clicked() {
                model.board.reset(model.egui_data.mode);
            }

            if ui.button("Save game").clicked() {
                save_game(&model.board);
            }

            if ui.button("Load game").clicked() {
                if let Some(board) = load_game() {
                    model.board = board;
                }
            }
        });

        if model.board.turn != Turn::None {
            ui.label(format!("Currently {}'s turn", model.board.turn));
        }
    });
}
