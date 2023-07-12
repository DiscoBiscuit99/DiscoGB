use std::sync::{Arc, RwLock};
use std::time::Duration;

use egui_extras::Size;
use egui_grid::{Grid, GridBuilder};

use crate::gameboy::memory::Memory;
use crate::gameboy::GameBoy;

const DISPLAY_WIDTH: usize = 160;
const DISPLAY_HEIGHT: usize = 144;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MemoryView {
    ROM0,
    ROM1,
    VRAM,
    ERAM,
    WRAM0,
    WRAM1,
    OAM,
    IO,
    HRAM,
}

pub struct GuiState {
    pub gameboy: GameBoy,
    display: Display,
    pub step_manually: Arc<RwLock<bool>>,
    selected_memory_view: MemoryView,
}

impl GuiState {
    pub fn new() -> Self {
        let gameboy = GameBoy::new();
        Self {
            display: Display {
                texture: None,
                memory: gameboy.memory.clone(),
            },
            gameboy,
            step_manually: Arc::new(RwLock::new(true)),
            selected_memory_view: MemoryView::ROM0,
        }
    }

    pub fn run(&mut self) {
        // Step the CPU if the user has enabled manual stepping
        if !(self.step_manually.read().unwrap().to_owned()) {
            self.gameboy.cpu.write().unwrap().step();
        }
    }
}

struct Display {
    memory: Arc<RwLock<Memory>>,
    texture: Option<egui::TextureHandle>,
}

impl Display {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
            let mut addr = 0x8000;
            let first_sprite = std::iter::repeat_with(|| {
                let tmp = addr;
                addr += 1;
                tmp
            })
            .take(16)
            .map(|addr| self.memory.read().unwrap().read_byte(addr))
            .collect::<Vec<_>>();

            // Create pixel buffer
            let pixels = {
                let mut pixels = [0; 160 * 144 * 3];
                for i in 0..160 * 144 {
                    pixels[i * 3] = 255;
                    pixels[i * 3 + 1] = 82;
                    pixels[i * 3 + 2] = 82;
                }
                pixels[0] = first_sprite[0];
                pixels
            };

            // Load the texture only once
            ui.ctx().load_texture(
                "lcd_display",
                egui::ColorImage::from_rgb([DISPLAY_WIDTH, DISPLAY_HEIGHT], &pixels),
                egui::TextureOptions::default(),
            )
        });

        ui.image(texture, texture.size_vec2());
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
                    ui.monospace(format!(
                        "Z: {}",
                        state.gameboy.cpu.read().unwrap().get_flag_z()
                    ));
                });
            });
            grid.cell(|ui| {
                ui.vertical_centered(|ui| {
                    ui.monospace(format!(
                        "N: {}",
                        state.gameboy.cpu.read().unwrap().get_flag_n()
                    ));
                });
            });
            grid.cell(|ui| {
                ui.vertical_centered(|ui| {
                    ui.monospace(format!(
                        "H: {}",
                        state.gameboy.cpu.read().unwrap().get_flag_h()
                    ));
                });
            });
            grid.cell(|ui| {
                ui.vertical_centered(|ui| {
                    ui.monospace(format!(
                        "C: {}",
                        state.gameboy.cpu.read().unwrap().get_flag_c()
                    ));
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
    register16_ui(ui, "PC", state.gameboy.cpu.read().unwrap().pc);
    ui.add_space(5.0);
    register16_ui(ui, "SP", state.gameboy.cpu.read().unwrap().sp);
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
        .new_row(Size::exact(32.5))
        .cells(Size::remainder(), 2)
        .show(ui, |mut grid| {
            register8_grid_cell_ui(&mut grid, "A", state.gameboy.cpu.read().unwrap().regs.a);
            register8_grid_cell_ui(&mut grid, "F", state.gameboy.cpu.read().unwrap().regs.f);
            register8_grid_cell_ui(&mut grid, "B", state.gameboy.cpu.read().unwrap().regs.b);
            register8_grid_cell_ui(&mut grid, "C", state.gameboy.cpu.read().unwrap().regs.c);
            register8_grid_cell_ui(&mut grid, "D", state.gameboy.cpu.read().unwrap().regs.d);
            register8_grid_cell_ui(&mut grid, "E", state.gameboy.cpu.read().unwrap().regs.e);
            register8_grid_cell_ui(&mut grid, "H", state.gameboy.cpu.read().unwrap().regs.h);
            register8_grid_cell_ui(&mut grid, "L", state.gameboy.cpu.read().unwrap().regs.l);
        });
}

/// Displays the special flags IME and HALT.
fn special_flags_ui(state: &mut GuiState, ui: &mut egui::Ui) {
    let ime = if state.gameboy.cpu.read().unwrap().ime {
        1
    } else {
        0
    };
    let halt = if state.gameboy.cpu.read().unwrap().halt {
        1
    } else {
        0
    };

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

fn checkbox_step_manually_ui(state: &mut GuiState, ui: &mut egui::Ui) {
    ui.add_space(5.0);
    ui.vertical(|ui| {
        ui.checkbox(&mut *state.step_manually.write().unwrap(), "Step manually");
    });
}

fn memory_view_selectable_ui(state: &mut GuiState, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.selectable_value(&mut state.selected_memory_view, MemoryView::ROM0, "ROM 0");
        ui.selectable_value(&mut state.selected_memory_view, MemoryView::ROM1, "ROM 1");
        ui.selectable_value(&mut state.selected_memory_view, MemoryView::VRAM, "VRAM");
        ui.selectable_value(&mut state.selected_memory_view, MemoryView::ERAM, "ERAM");
        ui.selectable_value(&mut state.selected_memory_view, MemoryView::WRAM0, "WRAM 0");
        ui.selectable_value(&mut state.selected_memory_view, MemoryView::WRAM1, "WRAM 1");
        ui.selectable_value(&mut state.selected_memory_view, MemoryView::OAM, "OAM");
        ui.selectable_value(&mut state.selected_memory_view, MemoryView::HRAM, "HRAM");
        ui.selectable_value(&mut state.selected_memory_view, MemoryView::IO, "IO");
    });
}

impl eframe::App for GuiState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Processor")
            .fixed_size(egui::vec2(175.0, 175.0))
            .show(ctx, |ui| {
                ui.group(|ui| {
                    flags_ui(self, ui);
                });

                ui.group(|ui| {
                    special_registers_ui(self, ui);
                });

                ui.group(|ui| {
                    registers_ui(self, ui);
                });

                ui.group(|ui| {
                    special_flags_ui(self, ui);
                });

                checkbox_step_manually_ui(self, ui);
            });

        egui::Window::new("Display")
            .fixed_size(egui::vec2(175.0, 175.0))
            .show(ctx, |ui| {
                self.display.ui(ui);
            });

        egui::Window::new("IO Map")
            .fixed_size(egui::vec2(175.0, 175.0))
            .show(ctx, |_ui| {});

        egui::Window::new("Memory")
            .default_size(egui::vec2(300.0, 250.0))
            .resizable(true)
            .show(ctx, |ui| {
                ui.group(|ui| {
                    memory_view_selectable_ui(self, ui);
                });

                let mem = self.gameboy.memory.read().unwrap();

                let (memory_view, total_rows) = match self.selected_memory_view {
                    MemoryView::ROM0 => (mem.rom.bank0.as_slice(), mem.rom.bank0.len() / 8),
                    MemoryView::ROM1 => (mem.rom.bankn.as_slice(), mem.rom.bankn.len() / 8),
                    MemoryView::VRAM => (mem.vram.as_slice(), mem.vram.len() / 8),
                    MemoryView::ERAM => (mem.eram.as_slice(), mem.eram.len() / 8),
                    MemoryView::WRAM0 => (mem.wram.bank0.as_slice(), mem.wram.bank0.len() / 8),
                    MemoryView::WRAM1 => (mem.wram.bankn.as_slice(), mem.wram.bankn.len() / 8),
                    MemoryView::OAM => (mem.oam.as_slice(), mem.oam.len() / 8),
                    MemoryView::IO => (mem.io.as_slice(), mem.io.len() / 8),
                    MemoryView::HRAM => (mem.hram.as_slice(), mem.hram.len() / 8),
                };

                let text_style = egui::TextStyle::Body;
                let row_height = ui.text_style_height(&text_style);

                ui.group(|ui| {
                    egui::ScrollArea::vertical().show_rows(
                        ui,
                        row_height,
                        total_rows,
                        |ui, row_range| {
                            for row in row_range {
                                ui.horizontal(|ui| {
                                    let offset = match self.selected_memory_view {
                                        MemoryView::ROM0 => 0x0000,
                                        MemoryView::ROM1 => 0x4000,
                                        MemoryView::VRAM => 0x8000,
                                        MemoryView::ERAM => 0xa000,
                                        MemoryView::WRAM0 => 0xc000,
                                        MemoryView::WRAM1 => 0xd000,
                                        MemoryView::OAM => 0xfe00,
                                        MemoryView::IO => 0xff00,
                                        MemoryView::HRAM => 0xff80,
                                    };

                                    if row * 8 < memory_view.len() - 1 {
                                        let row_addr = egui::RichText::new(format!(
                                            "{:04x}",
                                            row * 8 + offset
                                        ))
                                        .monospace()
                                        .color(egui::Color32::from_rgb(255, 82, 82));

                                        ui.label(row_addr);
                                        ui.separator();

                                        let memory_view_range =
                                            row * 8..std::cmp::min(row * 8 + 8, memory_view.len());

                                        for byte in memory_view[memory_view_range.clone()].iter() {
                                            let byte_color = if *byte > 0x00 {
                                                egui::Color32::from_rgb(230, 230, 230)
                                            } else {
                                                egui::Color32::from_rgb(150, 150, 150)
                                            };

                                            let byte_label =
                                                egui::RichText::new(format!("{:02x}", byte))
                                                    .monospace()
                                                    .color(byte_color);

                                            ui.monospace(byte_label);
                                        }

                                        ui.separator();

                                        for byte in memory_view[memory_view_range].iter() {
                                            let c = *byte as char;
                                            if c.is_ascii_alphanumeric() || c.is_ascii_punctuation()
                                            {
                                                let byte_label =
                                                    egui::RichText::new(format!("{c}"))
                                                        .monospace()
                                                        .color(egui::Color32::from_rgb(
                                                            200, 200, 200,
                                                        ));

                                                ui.monospace(byte_label);
                                            } else {
                                                ui.monospace(".");
                                            }
                                        }

                                        ui.separator();
                                    }
                                });
                            }
                        },
                    );
                });

                // for chunk in self.gameboy.memory.borrow().rom.bank0.chunks(16) {
                //     ui.horizontal(|ui| {
                //         for byte in chunk {
                //             ui.monospace(format!("{:02x} ", byte));
                //         }
                //     });
                // }
            });

        egui::Window::new("VRAM Viewer")
            .fixed_size(egui::vec2(175.0, 175.0))
            .show(ctx, |_ui| {});

        egui::Window::new("Disassembler")
            .fixed_size(egui::vec2(175.0, 175.0))
            .show(ctx, |_ui| {});

        egui::Window::new("Sound Registers")
            .fixed_size(egui::vec2(175.0, 175.0))
            .show(ctx, |_ui| {});

        // Step the CPU if the user has enabled manual stepping
        if self.step_manually.read().unwrap().to_owned() {
            ctx.input(|i| {
                if i.key_down(egui::Key::Space) || i.key_pressed(egui::Key::Enter) {
                    self.gameboy.cpu.write().unwrap().step();
                }
            });
        }

        // request call to this update function (60 FPS)
        ctx.request_repaint_after(Duration::from_secs_f64(1.0 / 60.0));
    }
}
