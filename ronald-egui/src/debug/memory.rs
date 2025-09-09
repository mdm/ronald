use eframe::egui;
use serde::{Deserialize, Serialize};

use ronald_core::debug::view::MemoryDebugView;

#[derive(Deserialize, Serialize)]
pub struct MemoryDebugWindow {
    pub show: bool,
    scroll_to_address: Option<usize>,
    address_input: String,
    view_mode: MemoryViewMode,
}

#[derive(Deserialize, Serialize, PartialEq)]
enum MemoryViewMode {
    CompositeRomRam,
    CompositeRam,
    ExtensionRamOnly,
}

impl Default for MemoryDebugWindow {
    fn default() -> Self {
        Self {
            show: false,
            scroll_to_address: None,
            address_input: String::new(),
            view_mode: MemoryViewMode::CompositeRomRam,
        }
    }
}

impl MemoryDebugWindow {
    fn render_view_mode_selector(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("View:");
            egui::ComboBox::from_label("")
                .selected_text(match self.view_mode {
                    MemoryViewMode::CompositeRomRam => "Composite ROM/RAM",
                    MemoryViewMode::CompositeRam => "Composite RAM",
                    MemoryViewMode::ExtensionRamOnly => "Extension RAM only",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.view_mode,
                        MemoryViewMode::CompositeRomRam,
                        "Composite ROM/RAM",
                    );
                    ui.selectable_value(
                        &mut self.view_mode,
                        MemoryViewMode::CompositeRam,
                        "Composite RAM",
                    );
                    ui.selectable_value(
                        &mut self.view_mode,
                        MemoryViewMode::ExtensionRamOnly,
                        "Extension RAM only",
                    );
                });
        });
    }
    pub fn ui(&mut self, ctx: &egui::Context, data: Option<&MemoryDebugView>) {
        if !self.show {
            return;
        }

        let mut open = self.show;
        egui::Window::new("Memory Internals")
            .open(&mut open)
            .default_size([600.0, 500.0])
            .show(ctx, |ui| {
                if let Some(data) = data {
                    self.render_view_mode_selector(ui);
                    ui.separator();
                    self.render_memory_controls(ui);
                    ui.separator();
                    self.render_memory_status(ui, data);
                    ui.separator();
                    self.render_memory_hex_dump(ui, data);
                } else {
                    ui.label("No data available - emulator must be paused");
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

                let memory_data = match self.view_mode {
                    MemoryViewMode::CompositeRomRam => &data.composite_rom_ram,
                    MemoryViewMode::CompositeRam => &data.composite_ram,
                    MemoryViewMode::ExtensionRamOnly => &data.ram_extension,
                };

                let label = match self.view_mode {
                    MemoryViewMode::CompositeRomRam => "=== Composite ROM/RAM ===",
                    MemoryViewMode::CompositeRam => "=== Composite RAM ===",
                    MemoryViewMode::ExtensionRamOnly => "=== Extension RAM Only ===",
                };

                ui.colored_label(egui::Color32::LIGHT_BLUE, label);
                self.render_hex_section(ui, memory_data, 0x0000);
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
