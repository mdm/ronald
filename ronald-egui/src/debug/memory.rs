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
    LowerRomOnly,
    UpperRomOnly(u8),
    RamOnly,
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
    fn render_view_mode_selector(&mut self, ui: &mut egui::Ui, data: &MemoryDebugView) {
        ui.horizontal(|ui| {
            ui.label("View:");
            egui::ComboBox::from_label("")
                .selected_text(match &self.view_mode {
                    MemoryViewMode::CompositeRomRam => "Composite ROM/RAM".to_string(),
                    MemoryViewMode::CompositeRam => "Composite RAM".to_string(),
                    MemoryViewMode::LowerRomOnly => "Lower ROM only".to_string(),
                    MemoryViewMode::UpperRomOnly(bank) => format!("Upper ROM #{:02X} only", bank),
                    MemoryViewMode::RamOnly => "RAM only".to_string(),
                    MemoryViewMode::ExtensionRamOnly => "Extension RAM only".to_string(),
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
                        MemoryViewMode::LowerRomOnly,
                        "Lower ROM only",
                    );

                    // Display all available upper ROM banks
                    let mut banks: Vec<_> = data.upper_roms.keys().collect();
                    banks.sort();
                    for &bank in banks {
                        ui.selectable_value(
                            &mut self.view_mode,
                            MemoryViewMode::UpperRomOnly(bank),
                            format!("Upper ROM #{:02X} only", bank),
                        );
                    }

                    ui.selectable_value(&mut self.view_mode, MemoryViewMode::RamOnly, "RAM only");
                    ui.selectable_value(
                        &mut self.view_mode,
                        MemoryViewMode::ExtensionRamOnly,
                        "Extension RAM only",
                    );
                });
        });
        ui.separator();
    }
    pub fn ui(&mut self, ctx: &egui::Context, data: Option<&MemoryDebugView>) {
        if !self.show {
            return;
        }

        let mut open = self.show;
        egui::Window::new("Memory Internals")
            .open(&mut open)
            .default_size([600.0, 500.0])
            .resizable(false)
            .show(ctx, |ui| {
                if let Some(data) = data {
                    self.render_view_mode_selector(ui, data);
                    self.render_memory_controls(ui);
                    self.render_memory_status(ui, data);
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
            let text_edit = ui.text_edit_singleline(&mut self.address_input);
            let enter_pressed =
                text_edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            let go_clicked = ui.button("Go").clicked();

            if enter_pressed || go_clicked {
                if let Ok(addr) =
                    usize::from_str_radix(self.address_input.trim_start_matches("0x"), 16)
                {
                    log::debug!("Setting scroll target to address: {:04X}", addr);
                    self.scroll_to_address = Some(addr);
                } else {
                    log::warn!("Failed to parse address: '{}'", self.address_input);
                }
            }
        });
        ui.separator();
    }

    fn render_memory_status(&self, ui: &mut egui::Ui, data: &MemoryDebugView) {
        match &self.view_mode {
            MemoryViewMode::CompositeRomRam => {
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
                    ui.label(format!("(Selected: #{:02X})", data.selected_upper_rom));
                });
                ui.separator();
            }
            MemoryViewMode::LowerRomOnly => {
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
                });
                ui.separator();
            }
            MemoryViewMode::UpperRomOnly(bank) => {
                ui.horizontal(|ui| {
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
                    ui.label(format!(
                        "(Selected: #{:02X}, Viewing: #{:02X})",
                        data.selected_upper_rom, bank
                    ));
                });
                ui.separator();
            }
            _ => {
                // No ROM status to show for other view modes
            }
        }
    }

    fn render_memory_hex_dump(&mut self, ui: &mut egui::Ui, data: &MemoryDebugView) {
        ui.label("Memory Contents:");

        let memory_data = match &self.view_mode {
            MemoryViewMode::CompositeRomRam => &data.composite_rom_ram,
            MemoryViewMode::CompositeRam => &data.composite_ram,
            MemoryViewMode::LowerRomOnly => &data.lower_rom,
            MemoryViewMode::UpperRomOnly(bank) => {
                if let Some(upper_rom) = data.upper_roms.get(bank) {
                    upper_rom
                } else {
                    &Vec::<u8>::new()[..]
                }
            }
            MemoryViewMode::RamOnly => &data.ram,
            MemoryViewMode::ExtensionRamOnly => &data.ram_extension,
        };

        egui::ScrollArea::vertical()
            .max_height(400.0)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.style_mut().override_font_id = Some(egui::FontId::monospace(12.0));

                let target_addr = self.scroll_to_address.take();
                for (row, chunk) in memory_data.chunks(16).enumerate() {
                    let addr = row * 16;
                    let response = self.render_hex_row(ui, chunk, addr);

                    // If this row contains our target address, scroll to it immediately
                    if let Some(target) = target_addr
                        && target >= addr
                        && target < addr + 16
                    {
                        response.scroll_to_me(Some(egui::Align::Min));
                    }
                }
            });
    }

    fn render_hex_row(&self, ui: &mut egui::Ui, chunk: &[u8], addr: usize) -> egui::Response {
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
                .map(|&b| {
                    if (32..127).contains(&b) {
                        b as char
                    } else {
                        '.'
                    }
                })
                .collect();
            ui.monospace(ascii);
        })
        .response
    }
}
