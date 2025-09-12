use eframe::egui;
use serde::{Deserialize, Serialize};

use ronald_core::debug::view::{MemoryDebugView, SystemDebugView};

#[derive(Deserialize, Serialize)]
pub struct MemoryDebugWindow {
    pub show: bool,
    scroll_to_address: Option<usize>,
    address_input: String,
    view_mode: MemoryViewMode,
    memory_colors: MemorySourceColors,
}

#[derive(Deserialize, Serialize)]
struct MemorySourceColors {
    lower_rom: egui::Color32,
    upper_rom: egui::Color32,
    ram: egui::Color32,
    extension_ram: egui::Color32,
}

#[derive(Deserialize, Serialize, PartialEq)]
enum MemoryViewMode {
    Disassembly,
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
            memory_colors: MemorySourceColors::default(),
        }
    }
}

impl Default for MemorySourceColors {
    fn default() -> Self {
        Self {
            lower_rom: egui::Color32::from_rgb(255, 165, 0), // Orange
            upper_rom: egui::Color32::from_rgb(255, 20, 147), // Deep pink
            ram: egui::Color32::from_rgb(50, 205, 50),       // Lime green
            extension_ram: egui::Color32::from_rgb(135, 206, 235), // Sky blue
        }
    }
}

impl MemorySourceColors {
    fn get_color_for_mode(&self, mode: &MemoryViewMode) -> egui::Color32 {
        match mode {
            MemoryViewMode::LowerRomOnly => self.lower_rom,
            MemoryViewMode::UpperRomOnly(_) => self.upper_rom,
            MemoryViewMode::RamOnly => self.ram,
            MemoryViewMode::ExtensionRamOnly => self.extension_ram,
            _ => egui::Color32::from_gray(200), // Default text color for composite modes and disassembly
        }
    }
}

impl MemoryDebugWindow {
    fn get_memory_source_color(&self, addr: usize, data: &MemoryDebugView) -> egui::Color32 {
        // Only apply source-based coloring for composite modes
        match &self.view_mode {
            MemoryViewMode::CompositeRomRam => {
                // CPC 464 memory map:
                // 0x0000-0x3FFF: Lower ROM (if enabled) or RAM
                // 0x4000-0x7FFF: RAM
                // 0x8000-0xBFFF: RAM
                // 0xC000-0xFFFF: Upper ROM (if enabled) or RAM
                if addr < 0x4000 && data.lower_rom_enabled {
                    self.memory_colors.lower_rom
                } else if addr >= 0xC000 && data.upper_rom_enabled {
                    self.memory_colors.upper_rom
                } else {
                    self.memory_colors.ram
                }
            }
            MemoryViewMode::CompositeRam => {
                // All composite RAM uses RAM color
                self.memory_colors.ram
            }
            _ => {
                // For single-source modes, use the mode's color
                self.memory_colors.get_color_for_mode(&self.view_mode)
            }
        }
    }
    fn render_view_mode_selector(&mut self, ui: &mut egui::Ui, data: &MemoryDebugView) {
        ui.horizontal(|ui| {
            ui.label("View:");
            egui::ComboBox::from_label("")
                .selected_text(match &self.view_mode {
                    MemoryViewMode::Disassembly => "Disassembly".to_string(),
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
                        MemoryViewMode::Disassembly,
                        "Disassembly",
                    );
                    ui.separator();
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
                    let lower_rom_mode = MemoryViewMode::LowerRomOnly;
                    let color = self.memory_colors.get_color_for_mode(&lower_rom_mode);
                    ui.selectable_value(
                        &mut self.view_mode,
                        lower_rom_mode,
                        egui::RichText::new("Lower ROM only").color(color),
                    );

                    // Display all available upper ROM banks
                    let mut banks: Vec<_> = data.upper_roms.keys().collect();
                    banks.sort();
                    for &bank in banks {
                        let upper_rom_mode = MemoryViewMode::UpperRomOnly(bank);
                        let color = self.memory_colors.get_color_for_mode(&upper_rom_mode);
                        ui.selectable_value(
                            &mut self.view_mode,
                            upper_rom_mode,
                            egui::RichText::new(format!("Upper ROM #{:02X} only", bank))
                                .color(color),
                        );
                    }

                    let ram_mode = MemoryViewMode::RamOnly;
                    let color = self.memory_colors.get_color_for_mode(&ram_mode);
                    ui.selectable_value(
                        &mut self.view_mode,
                        ram_mode,
                        egui::RichText::new("RAM only").color(color),
                    );

                    let ext_ram_mode = MemoryViewMode::ExtensionRamOnly;
                    let color = self.memory_colors.get_color_for_mode(&ext_ram_mode);
                    ui.selectable_value(
                        &mut self.view_mode,
                        ext_ram_mode,
                        egui::RichText::new("Extension RAM only").color(color),
                    );
                });
        });
        ui.separator();
    }
    pub fn ui(&mut self, ctx: &egui::Context, data: Option<&SystemDebugView>) {
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
                    self.render_view_mode_selector(ui, &data.memory);
                    self.render_memory_controls(ui);
                    self.render_memory_status(ui, &data.memory);
                    self.render_memory_view(ui, data);
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

    fn render_memory_view(&mut self, ui: &mut egui::Ui, data: &SystemDebugView) {
        match &self.view_mode {
            MemoryViewMode::Disassembly => {
                self.render_disassembly_view(ui, data);
            }
            _ => {
                self.render_memory_hex_dump(ui, &data.memory);
            }
        }
    }

    fn render_memory_hex_dump(&mut self, ui: &mut egui::Ui, data: &MemoryDebugView) {
        ui.label("Memory Contents:");

        let memory_data = match &self.view_mode {
            MemoryViewMode::Disassembly => {
                unreachable!("Disassembly mode should be handled separately")
            }
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
                    let response = self.render_hex_row(ui, chunk, addr, data);

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

    fn render_hex_row(
        &self,
        ui: &mut egui::Ui,
        chunk: &[u8],
        addr: usize,
        data: &MemoryDebugView,
    ) -> egui::Response {
        let row_color = self.get_memory_source_color(addr, data);

        ui.horizontal(|ui| {
            // Address column
            ui.colored_label(egui::Color32::YELLOW, format!("{:04X}:", addr));

            // Hex bytes
            for (i, byte) in chunk.iter().enumerate() {
                if i == 8 {
                    ui.label(" ");
                }
                ui.colored_label(row_color, format!("{:02X}", byte));
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

            ui.label(ascii);
        })
        .response
    }

    fn render_disassembly_view(&mut self, ui: &mut egui::Ui, data: &SystemDebugView) {
        ui.label("Disassembly:");

        egui::ScrollArea::vertical()
            .max_height(400.0)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.style_mut().override_font_id = Some(egui::FontId::monospace(12.0));

                let target_addr = self.scroll_to_address.take();

                use egui_extras::{Column, TableBuilder};
                TableBuilder::new(ui)
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::remainder())
                    .body(|mut body| {
                        for instruction in &data.disassembly {
                            let is_current_instruction =
                                instruction.address == data.cpu.register_pc;

                            body.row(18.0, |mut row| {
                                // Highlight the entire row if it's the current instruction
                                if is_current_instruction {
                                    row.set_selected(true);
                                }

                                row.col(|ui| {
                                    let color = if is_current_instruction {
                                        egui::Color32::WHITE
                                    } else {
                                        egui::Color32::YELLOW
                                    };
                                    ui.colored_label(
                                        color,
                                        format!("{:04X}:", instruction.address),
                                    );
                                });
                                row.col(|ui| {
                                    let color = if is_current_instruction {
                                        egui::Color32::WHITE
                                    } else {
                                        egui::Color32::LIGHT_BLUE
                                    };
                                    ui.colored_label(color, &instruction.instruction);
                                });
                                row.col(|ui| {
                                    let mut hex_bytes = String::new();
                                    for i in 0..instruction.length {
                                        if i > 0 {
                                            hex_bytes.push(' ');
                                        }
                                        let addr = instruction.address as usize + i;
                                        if let Some(byte) = data.memory.composite_rom_ram.get(addr)
                                        {
                                            hex_bytes.push_str(&format!("{:02X}", byte));
                                        } else {
                                            hex_bytes.push_str("??");
                                        }
                                    }
                                    let response = if is_current_instruction {
                                        ui.colored_label(egui::Color32::WHITE, hex_bytes)
                                    } else {
                                        ui.monospace(hex_bytes)
                                    };

                                    if let Some(target) = target_addr
                                        && target as u16 == instruction.address
                                    {
                                        response.scroll_to_me(Some(egui::Align::Min));
                                    }
                                });
                            });
                        }
                    });
            });
    }
}
