use egui_grid::{Grid, GridBuilder};
use egui_extras::Size;

use crate::gameboy::GameBoy;

const DISPLAY_WIDTH: usize = 160;
const DISPLAY_HEIGHT: usize = 144;

pub struct GuiState {
    gameboy: GameBoy,
    display: Display,
    step_manually: bool,
}

impl GuiState {
    pub fn new() -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            gameboy: GameBoy::new(),
            display: Display { texture: None },
            step_manually: true,
        }
    }
}

fn format_u8_binary(value: u8) -> String {
    let upper = value >> 4;
    let lower = value & 0xf;
    format!("{:04b} {:04b}", upper, lower)
}

/// Returns the binary formatted string of the given value.
fn format_u16_binary(value: u16) -> String {
    let upper = value >> 8;
    let lower = value & 0xff;
    format_u8_binary(upper as u8) + " " + &format_u8_binary(lower as u8)
}

/// Displays a separator with appropriate margins.
fn separator_ui(ui: &mut egui::Ui) {
    ui.add_space(5.0);
    ui.separator();
    ui.add_space(5.0);
}

/// Displays the flags (the relevant values of the `F` register).
fn flags_ui(state: &mut GuiState, ui: &mut egui::Ui) {
    ui.add_space(5.0);

    GridBuilder::new()
        .new_row(Size::exact(15.0))
        .cells(Size::remainder(), 2)
        .new_row(Size::exact(15.0))
        .cells(Size::remainder(), 2)
        .show(ui, |mut grid| {
            grid.cell(|ui| {
                ui.vertical_centered(|ui| {
                    ui.monospace(format!("Z: {}", state.gameboy.cpu.borrow().get_flag_z()));
                });
            });
            grid.cell(|ui| {
                ui.vertical_centered(|ui| {
                    ui.monospace(format!("N: {}", state.gameboy.cpu.borrow().get_flag_n()));
                });
            });
            grid.cell(|ui| {
                ui.vertical_centered(|ui| {
                    ui.monospace(format!("H: {}", state.gameboy.cpu.borrow().get_flag_h()));
                });
            });
            grid.cell(|ui| {
                ui.vertical_centered(|ui| {
                    ui.monospace(format!("C: {}", state.gameboy.cpu.borrow().get_flag_c()));
                });
            });
        });
}

/// Displays a grid cell with the given label and value.
fn register16_ui(ui: &mut egui::Ui, label: &str, value: u16) {
    ui.vertical_centered(|ui| {
        ui.monospace(format!("{label}: {value:#06x}"));
        ui.monospace(format_u16_binary(value));
    });
}

/// Displays a grid cell with the given label and value.
fn register8_grid_cell_ui(grid: &mut Grid, label: &str, value: u8) {
    grid.cell(|ui| {
        ui.vertical_centered(|ui| {
            ui.monospace(format!("{label}: {value:#04x}"));
            ui.monospace(format_u8_binary(value));
        });
    });
}

/// Displays the 16-bit (special) registers.
fn special_registers_ui(state: &mut GuiState, ui: &mut egui::Ui) {
    register16_ui(ui, "PC", state.gameboy.cpu.borrow().pc);
    ui.add_space(5.0);
    register16_ui(ui, "SP", state.gameboy.cpu.borrow().sp);
}

/// Displays the 8-bit registers in a grid.
fn registers_ui(state: &mut GuiState, ui: &mut egui::Ui) {
    GridBuilder::new()
        .new_row(Size::exact(40.0))
        .cells(Size::remainder(), 2)
        .new_row(Size::exact(40.0))
        .cells(Size::remainder(), 2)
        .new_row(Size::exact(40.0))
        .cells(Size::remainder(), 2)
        .new_row(Size::exact(40.0))
        .cells(Size::remainder(), 2)
        .show(ui, |mut grid| {
            register8_grid_cell_ui(&mut grid, "A", state.gameboy.cpu.borrow().regs.a);
            register8_grid_cell_ui(&mut grid, "F", state.gameboy.cpu.borrow().regs.f);
            register8_grid_cell_ui(&mut grid, "B", state.gameboy.cpu.borrow().regs.b);
            register8_grid_cell_ui(&mut grid, "C", state.gameboy.cpu.borrow().regs.c);
            register8_grid_cell_ui(&mut grid, "D", state.gameboy.cpu.borrow().regs.d);
            register8_grid_cell_ui(&mut grid, "E", state.gameboy.cpu.borrow().regs.e);
            register8_grid_cell_ui(&mut grid, "H", state.gameboy.cpu.borrow().regs.h);
            register8_grid_cell_ui(&mut grid, "L", state.gameboy.cpu.borrow().regs.l);
        });
}

/// Displays the special flags IME and HALT.
fn special_flags_ui(state: &mut GuiState, ui: &mut egui::Ui) {
    let ime = if state.gameboy.cpu.borrow().ime { 1 } else { 0 };
    let halt = if state.gameboy.cpu.borrow().halt { 1 } else { 0 };

    GridBuilder::new()
        .new_row(Size::exact(15.0))
        .cells(Size::remainder(), 2)
        .show(ui, |mut grid| {
            grid.cell(|ui| {
                ui.vertical_centered(|ui| {
                    // let label = egui::RichText::new("IME")
                    //     .monospace()
                    //     .color(egui::Color32::from_rgb(255, 0, 0));
                    //
                    // let value = egui::RichText::new(format!("{}", ime))
                    //     .monospace()
                    //     .color(egui::Color32::from_rgb(0, 255, 0));
                    //
                    // ui.label(label);
                    // ui.label(value);
                    ui.monospace(format!("IME: {ime}"));
                });
            });
            grid.cell(|ui| {
                ui.vertical_centered(|ui| {
                    ui.monospace(format!("HALT: {halt}"));
                });
            });
        });
}

impl eframe::App for GuiState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Processor")
            .fixed_size(egui::vec2(175.0, 175.0))
            .show(ctx, |ui| {
                flags_ui(self, ui);
                separator_ui(ui);
                special_registers_ui(self, ui);
                separator_ui(ui);
                registers_ui(self, ui);
                separator_ui(ui);
                special_flags_ui(self, ui);
            });

        egui::Window::new("Display")
            .fixed_size(egui::vec2(175.0, 175.0))
            .show(ctx, |ui| {
                self.display.ui(ui);
            });

        egui::Window::new("IO Map")
            .fixed_size(egui::vec2(175.0, 175.0))
            .show(ctx, |ui| {});

        egui::Window::new("Memory Editor")
            .fixed_size(egui::vec2(175.0, 175.0))
            .show(ctx, |ui| {});

        egui::Window::new("VRAM Viewer")
            .fixed_size(egui::vec2(175.0, 175.0))
            .show(ctx, |ui| {});

        egui::Window::new("Disassembler")
            .fixed_size(egui::vec2(175.0, 175.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Step manually");
                    ui.checkbox(&mut self.step_manually, "");
                });
            });

        egui::Window::new("Sound Registers")
            .fixed_size(egui::vec2(175.0, 175.0))
            .show(ctx, |ui| {});

        // Step the CPU if the user has enabled manual stepping
        if self.step_manually {
            ctx.input(|i| {
                if i.key_down(egui::Key::Space) {
                    self.gameboy.cpu.borrow_mut().step();
                }
            });
        } else {
            self.gameboy.cpu.borrow_mut().step();
        }

        // request call to this update function
        ctx.request_repaint();
    }
}

struct Display {
    texture: Option<egui::TextureHandle>,
}

impl Display {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
            // Create pixel buffer
            let pixels = {
                let mut pixels = [0; 160 * 144 * 3];
                for i in 0..160 * 144 {
                    pixels[i * 3] = 255;
                    pixels[i * 3 + 1] = 82;
                    pixels[i * 3 + 2] = 82;
                }
                pixels
            };

            // Load the texture only once
            ui.ctx().load_texture(
                "lcd_display",
                egui::ColorImage::from_rgb([DISPLAY_WIDTH, DISPLAY_HEIGHT], &pixels),
                egui::TextureOptions::default()
            )
        });

        ui.image(texture, texture.size_vec2());
    }
}
