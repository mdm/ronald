use eframe::egui;
use serde::{Deserialize, Serialize};

use ronald_core::debug::view::MemoryDebugView;

#[derive(Default, Deserialize, Serialize)]
pub struct MemoryDebugWindow {
    pub show: bool,
    scroll_to_address: Option<usize>,
    address_input: String,
}

impl MemoryDebugWindow {
    pub fn ui(&mut self, ctx: &egui::Context, data: Option<&MemoryDebugView>) {
        if !self.show {
            return;
        }

        let mut open = self.show;
        egui::Window::new("Memory Debug")
            .open(&mut open)
            .default_size([600.0, 500.0])
            .show(ctx, |ui| {
                if let Some(data) = data {
                    self.render_memory_controls(ui);
                    ui.separator();
                    self.render_memory_status(ui, data);
                    ui.separator();
                    self.render_memory_hex_dump(ui, data);
                } else {
                    ui.label("No debug data available - emulator must be paused");
                }
            });
        self.show = open;
    }

    fn render_memory_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Jump to address:");
            ui.text_edit_singleline(&mut self.address_input);
            if ui.button("Go").clicked() {
                if let Ok(addr) =
                    usize::from_str_radix(&self.address_input.trim_start_matches("0x"), 16)
                {
                    self.scroll_to_address = Some(addr);
                }
            }
        });
    }

    fn render_memory_status(&self, ui: &mut egui::Ui, data: &MemoryDebugView) {
        ui.horizontal(|ui| {
            ui.label("Lower ROM:");
            ui.colored_label(
                if data.lower_rom_enabled {
                    egui::Color32::GREEN
                } else {
                    egui::Color32::RED
                },
                if data.lower_rom_enabled {
                    "ENABLED"
                } else {
                    "DISABLED"
                },
            );
            ui.separator();
            ui.label("Upper ROM:");
            ui.colored_label(
                if data.upper_rom_enabled {
                    egui::Color32::GREEN
                } else {
                    egui::Color32::RED
                },
                if data.upper_rom_enabled {
                    "ENABLED"
                } else {
                    "DISABLED"
                },
            );
            if data.upper_rom_enabled {
                ui.label(format!("(#{:02X})", data.selected_upper_rom));
            }
        });
    }

    fn render_memory_hex_dump(&mut self, ui: &mut egui::Ui, data: &MemoryDebugView) {
        egui::ScrollArea::vertical()
            .max_height(400.0)
            .show(ui, |ui| {
                ui.style_mut().override_font_id = Some(egui::FontId::monospace(12.0));

                // Show RAM section
                ui.colored_label(egui::Color32::LIGHT_BLUE, "=== RAM (64KB) ===");
                self.render_hex_section(ui, &data.ram, 0x0000);

                ui.add_space(10.0);

                // Show Lower ROM if enabled
                if data.lower_rom_enabled {
                    ui.colored_label(egui::Color32::LIGHT_GREEN, "=== Lower ROM ===");
                    self.render_hex_section(ui, &data.lower_rom, 0x0000);
                    ui.add_space(10.0);
                }

                // Show Upper ROM if enabled
                if data.upper_rom_enabled {
                    if let Some(upper_rom) = data.upper_roms.get(&data.selected_upper_rom) {
                        ui.colored_label(
                            egui::Color32::LIGHT_YELLOW,
                            format!("=== Upper ROM #{:02X} ===", data.selected_upper_rom),
                        );
                        self.render_hex_section(ui, upper_rom, 0xC000);
                    }
                }
            });
    }

    fn render_hex_section(&self, ui: &mut egui::Ui, data: &[u8], base_addr: usize) {
        for (row, chunk) in data.chunks(16).enumerate() {
            let addr = base_addr + (row * 16);

            ui.horizontal(|ui| {
                // Address column
                ui.colored_label(egui::Color32::YELLOW, format!("{:04X}:", addr));

                // Hex bytes
                for (i, byte) in chunk.iter().enumerate() {
                    if i == 8 {
                        ui.label(" ");
                    }
                    ui.label(format!("{:02X}", byte));
                }

                // Padding for incomplete rows
                for i in chunk.len()..16 {
                    if i == 8 {
                        ui.label(" ");
                    }
                    ui.label("  ");
                }

                ui.separator();

                // ASCII column
                let ascii: String = chunk
                    .iter()
                    .map(|&b| if b >= 32 && b <= 126 { b as char } else { '.' })
                    .collect();
                ui.monospace(ascii);
            });
        }
    }
}
