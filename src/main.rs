use std::thread;

use discogb::gui::GuiState;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1080.0, 720.0)),
        ..Default::default()
    };

    let state = GuiState::new();
    let cpu = state.gameboy.cpu.clone();
    let should_step_manually = state.step_manually.clone();

    thread::Builder::new()
        .name("GameBoy Run-Loop".to_string())
        .spawn(move || loop {
            if !should_step_manually.read().unwrap().to_owned() {
                cpu.write().unwrap().step();
            }
        })
        .unwrap();

    eframe::run_native("DiscoGB", options, Box::new(|_cc| Box::new(state))).unwrap();
}
