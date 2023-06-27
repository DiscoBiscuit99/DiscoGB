use discogb::gui::GuiState;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1080.0, 720.0)),
        ..Default::default()
    };

    eframe::run_native("DiscoGB", options, Box::new(|_cc| Box::new(GuiState::new()))).unwrap();
}
