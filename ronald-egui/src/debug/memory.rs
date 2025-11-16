use std::collections::HashMap;

use eframe::egui;
use serde::{Deserialize, Serialize};

use ronald_core::{
    debug::{
        breakpoint::{AnyBreakpoint, Breakpoint, BreakpointId, BreakpointManager},
        view::MemoryDebugView,
    },
    system::{cpu::Register16, instruction::DecodedInstruction},
};

use crate::{colors, debug::Debugger};

#[derive(Deserialize, Serialize)]
pub struct MemoryDebugWindow {
    pub show: bool,
    jump_to_address: Option<usize>,
    address_input: String,
    view_mode: MemoryViewMode,
    memory_colors: MemorySourceColors,
    pc_breakpoint_input: String,
    disassembly_start: Option<u16>,
    #[serde(skip)]
    cached_disassembly: Option<Vec<DecodedInstruction>>,
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
            jump_to_address: None,
            address_input: String::new(),
            view_mode: MemoryViewMode::CompositeRomRam,
            memory_colors: MemorySourceColors::default(),
            pc_breakpoint_input: String::new(),
            disassembly_start: None,
            cached_disassembly: None,
        }
    }
}

impl Default for MemorySourceColors {
    fn default() -> Self {
        Self {
            lower_rom: colors::DARK_ORANGE,
            upper_rom: colors::DEEP_MAGENTA,
            ram: colors::FORREST_GREEN,
            extension_ram: colors::DODGER_BLUE,
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
            _ => colors::DARK_GRAY,
        }
    }
}

impl MemoryDebugWindow {
    fn get_memory_source_color(&self, addr: usize, data: &MemoryDebugView) -> egui::Color32 {
        match &self.view_mode {
            MemoryViewMode::Disassembly | MemoryViewMode::CompositeRomRam => {
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
    fn render_view_mode_selector(&mut self, ui: &mut egui::Ui, debugger: &mut impl Debugger) {
        let data = &debugger.debug_view().memory;
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
    }

    fn render_color_configuration(&mut self, ui: &mut egui::Ui) {
        ui.collapsing("Address Color Coding", |ui| {
            ui.horizontal(|ui| {
                let label = ui.label("Lower ROM:");
                ui.color_edit_button_srgba(&mut self.memory_colors.lower_rom)
                    .labelled_by(label.id);

                ui.separator();

                let label = ui.label("Upper ROM:");
                ui.color_edit_button_srgba(&mut self.memory_colors.upper_rom)
                    .labelled_by(label.id);

                ui.separator();

                let label = ui.label("RAM:");
                ui.color_edit_button_srgba(&mut self.memory_colors.ram)
                    .labelled_by(label.id);

                ui.separator();

                let label = ui.label("Extension RAM:");
                ui.color_edit_button_srgba(&mut self.memory_colors.extension_ram)
                    .labelled_by(label.id);
            });

            if ui.button("Restore Defaults").clicked() {
                self.memory_colors = MemorySourceColors::default();
            }
        });
    }

    pub fn ui(&mut self, ctx: &egui::Context, debugger: &mut impl Debugger) {
        if !self.show {
            return;
        }

        let mut open = self.show;
        egui::Window::new("Memory Internals")
            .open(&mut open)
            .show(ctx, |ui| {
                ui.heading("View Config");
                self.render_view_mode_selector(ui, debugger);
                self.render_color_configuration(ui);
                self.render_memory_controls(ui);
                ui.separator();
                self.render_memory_status(ui, debugger);
                self.render_memory_view(ui, debugger);

                // Reserve remaining vertical space so the window can grow larger
                // ui.allocate_space(ui.available_size());
            });
        self.show = open;
    }

    fn render_memory_controls(&mut self, ui: &mut egui::Ui) {
        let size = match &self.view_mode {
            MemoryViewMode::Disassembly => 0x10000,
            MemoryViewMode::CompositeRomRam => 0x10000,
            MemoryViewMode::CompositeRam => 0x10000,
            MemoryViewMode::LowerRomOnly => 0x4000,
            MemoryViewMode::UpperRomOnly(_) => 0x4000,
            MemoryViewMode::RamOnly => 0x10000,
            MemoryViewMode::ExtensionRamOnly => 0x10000,
        };

        ui.horizontal(|ui| {
            let label = ui.label("Jump to address:");
            let text_edit = ui
                .text_edit_singleline(&mut self.address_input)
                .labelled_by(label.id);
            let enter_pressed =
                text_edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            let go_clicked = ui.button("Go").clicked();

            if enter_pressed || go_clicked {
                if self.view_mode == MemoryViewMode::Disassembly {
                    if let Ok(addr) =
                        usize::from_str_radix(self.address_input.trim_start_matches("0x"), 16)
                    {
                        log::debug!("Setting disassembly start address: {:04X}", addr);
                        // TODO: show toast when wrapping
                        self.disassembly_start = Some((addr % size) as u16);
                        self.cached_disassembly = None; // Invalidate cache
                    } else {
                        log::warn!("Failed to parse address: '{}'", self.address_input);
                    }
                } else if let Ok(addr) =
                    usize::from_str_radix(self.address_input.trim_start_matches("0x"), 16)
                {
                    log::debug!("Setting scroll target to address: {:04X}", addr);
                    // TODO: show toast when wrapping
                    self.jump_to_address = Some(addr % size);
                } else {
                    log::warn!("Failed to parse address: '{}'", self.address_input);
                }
            }

            if self.view_mode == MemoryViewMode::Disassembly
                && ui
                    .add(
                        egui::Button::new("Track Current PC")
                            .selected(self.disassembly_start.is_none()),
                    )
                    .clicked()
            {
                self.disassembly_start = None;
            }
        });
    }

    fn render_memory_status(&self, ui: &mut egui::Ui, debugger: &mut impl Debugger) {
        let data = &debugger.debug_view().memory;
        match &self.view_mode {
            MemoryViewMode::CompositeRomRam => {
                ui.horizontal(|ui| {
                    ui.label("Lower ROM:");
                    ui.colored_label(
                        if data.lower_rom_enabled {
                            colors::FORREST_GREEN
                        } else {
                            colors::DARK_RED
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
                            colors::FORREST_GREEN
                        } else {
                            colors::DARK_RED
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
                            colors::FORREST_GREEN
                        } else {
                            colors::DARK_RED
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
                            colors::FORREST_GREEN
                        } else {
                            colors::DARK_RED
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

    fn render_memory_view(&mut self, ui: &mut egui::Ui, debugger: &mut impl Debugger) {
        match &self.view_mode {
            MemoryViewMode::Disassembly => {
                self.render_disassembly_view(ui, debugger);
            }
            _ => {
                self.render_memory_hex_dump(ui, debugger);
            }
        }
    }

    fn render_memory_hex_dump(&mut self, ui: &mut egui::Ui, debugger: &mut impl Debugger) {
        let data = &debugger.debug_view().memory;
        ui.heading("Memory Contents:");

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
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.style_mut().override_font_id = Some(egui::FontId::monospace(12.0));

                let target_addr = self.jump_to_address.take();
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
            // Address column with color coding
            ui.colored_label(row_color, format!("{:04X}:", addr));

            // Hex bytes
            for (i, byte) in chunk.iter().enumerate() {
                if i == 8 {
                    ui.label(" ");
                }
                ui.label(format!("{:02X}", byte));
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

    fn render_disassembly_view(&mut self, ui: &mut egui::Ui, debugger: &mut impl Debugger) {
        // Generate disassembly if we are tracking the current PC or if cache is empty
        if self.disassembly_start.is_none() || self.cached_disassembly.is_none() {
            let current_pc = debugger.debug_view().cpu.register_pc;
            let start_addr = self.disassembly_start.unwrap_or(current_pc);
            self.cached_disassembly = Some(debugger.disassemble(start_addr, 100));
        }

        // Add PC breakpoint controls
        self.render_pc_breakpoint_controls(ui, debugger.breakpoint_manager());
        ui.separator();

        // Build a HashMap from PC addresses to BreakpointIds for efficient lookup
        let pc_breakpoints: HashMap<u16, BreakpointId> = debugger
            .breakpoint_manager()
            .breakpoints_iter()
            .filter_map(|(id, breakpoint)| {
                if let AnyBreakpoint::CpuRegister16(bp) = breakpoint
                    && let Some(addr) = bp.value
                    && !bp.one_shot()
                {
                    Some((addr, *id))
                } else {
                    None
                }
            })
            .collect();

        ui.label("💡 Click on addresses to toggle PC breakpoints");
        ui.separator();

        ui.heading("Disassembly");
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.style_mut().override_font_id = Some(egui::FontId::monospace(12.0));

                let target_addr = self.jump_to_address.take();
                let mut to_toggle = None;

                use egui_extras::{Column, TableBuilder};
                TableBuilder::new(ui)
                    .column(Column::auto())
                    .column(Column::remainder())
                    .column(Column::remainder())
                    .body(|mut body| {
                        let data = debugger.debug_view();

                        for instruction in self
                            .cached_disassembly
                            .as_ref()
                            .expect("ensured to exist above")
                        {
                            let is_current_instruction =
                                instruction.address == data.cpu.register_pc;
                            let has_breakpoint = pc_breakpoints.contains_key(&instruction.address);

                            body.row(18.0, |mut row| {
                                // Highlight the entire row if it's the current instruction
                                if is_current_instruction {
                                    row.set_selected(true);
                                }

                                row.col(|ui| {
                                    let mut color = if is_current_instruction {
                                        colors::WHITE
                                    } else {
                                        self.get_memory_source_color(
                                            instruction.address as usize,
                                            &data.memory,
                                        )
                                    };

                                    let mut addr_text = format!("  {:04X}:", instruction.address);

                                    // Add breakpoint indicator and change color
                                    if has_breakpoint {
                                        addr_text = format!("● {:04X}:", instruction.address);
                                        if !is_current_instruction {
                                            color = colors::LIGHT_RED;
                                        }
                                    }

                                    // Make address clickable to toggle breakpoint
                                    if ui.colored_label(color, &addr_text).clicked() {
                                        to_toggle = Some(instruction.address);
                                    }
                                });

                                row.col(|ui| {
                                    let color = if is_current_instruction {
                                        colors::WHITE
                                    } else {
                                        colors::STEEL_BLUE
                                    };
                                    ui.colored_label(color, instruction.instruction.to_string());
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
                                        ui.colored_label(colors::WHITE, hex_bytes)
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

                // Handle breakpoint toggling after the table to avoid borrowing issues
                if let Some(addr) = to_toggle {
                    let breakpoint_manager = debugger.breakpoint_manager();
                    if let Some(&id) = pc_breakpoints.get(&addr) {
                        // Remove existing breakpoint
                        breakpoint_manager.remove_breakpoint(id);
                    } else {
                        // Add new breakpoint
                        breakpoint_manager.add_breakpoint(AnyBreakpoint::pc_breakpoint(addr));
                    }
                }
            });
    }

    fn render_pc_breakpoint_controls(
        &mut self,
        ui: &mut egui::Ui,
        breakpoint_manager: &mut BreakpointManager,
    ) {
        ui.heading("PC Breakpoints");

        // PC breakpoint input
        ui.horizontal(|ui| {
            let label = ui.label("PC:");
            ui.text_edit_singleline(&mut self.pc_breakpoint_input)
                .labelled_by(label.id);

            if ui.button("Add").clicked()
                && let Ok(addr) =
                    u16::from_str_radix(self.pc_breakpoint_input.trim_start_matches("0x"), 16)
            {
                breakpoint_manager.add_breakpoint(AnyBreakpoint::pc_breakpoint(addr));
                self.pc_breakpoint_input.clear();
            }
        });

        // List active PC breakpoints
        ui.separator();
        ui.label("Active PC Breakpoints:");

        let mut breakpoint_found = false;
        let mut to_remove = None;
        let mut to_toggle = None;

        for (id, breakpoint) in breakpoint_manager.breakpoints_iter() {
            if breakpoint.one_shot() {
                continue;
            }
            if let AnyBreakpoint::CpuRegister16(bp) = breakpoint {
                if bp.register != Register16::PC {
                    continue;
                }
            } else {
                continue;
            }

            breakpoint_found = true;

            ui.horizontal(|ui| {
                let mut enabled = breakpoint.enabled();
                if ui.checkbox(&mut enabled, breakpoint.to_string()).changed() {
                    to_toggle = Some((*id, enabled));
                }

                if ui.button("Remove").clicked() {
                    to_remove = Some(*id);
                }

                if let Some(master_clock) = breakpoint.triggered() {
                    ui.colored_label(
                        colors::DARK_RED,
                        format!("(triggered at {})", master_clock.value()),
                    );
                }
            });
        }

        if !breakpoint_found {
            ui.label("No PC breakpoints set");
        }

        // Apply changes outside the with_breakpoints closure
        if let Some((id, enabled)) = to_toggle {
            breakpoint_manager.enable_breakpoint(id, enabled);
        }
        if let Some(id) = to_remove {
            breakpoint_manager.remove_breakpoint(id);
        }
    }
}

#[cfg(test)]
mod gui_tests {
    use super::*;

    use egui::accesskit;
    use egui_kittest::{
        Harness,
        kittest::{self, Queryable},
    };

    use ronald_core::debug::breakpoint::{CpuRegister16Breakpoint, GateArrayScreenModeBreakpoint};

    use crate::debug::mock::TestDebugger;

    #[test]
    fn test_memory_debug_window_opens_and_closes() {
        let mut debugger = TestDebugger::default();
        let mut window = MemoryDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ctx: &egui::Context| {
            window.ui(ctx, &mut debugger);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Check that the window title is rendered
        harness.get_by_label("Memory Internals");

        // Click close button
        harness.get_by_label("Close window").click();
        harness.run();

        // Window should no longer be visible
        assert!(harness.query_by_label("Memory Internals").is_none());
    }

    #[test]
    fn test_memory_debug_window_pc_breakpoint_add_via_input_and_remove() {
        let mut debugger = TestDebugger::default();
        let mut window = MemoryDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ctx: &egui::Context| {
            window.ui(ctx, &mut debugger);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Select "Disassembly" view mode
        harness
            .get_all_by_role(accesskit::Role::ComboBox)
            .next()
            .unwrap()
            .click();
        harness.run();
        harness.get_by_label("Disassembly").click();
        harness.run();

        // Type in PC address
        harness
            .get_by_role_and_label(accesskit::Role::TextInput, "PC:")
            .type_text("0x0000");
        harness.run();

        // Add breakpoint
        harness
            .get_by_role_and_label(accesskit::Role::Button, "Add")
            .click();
        harness.run();

        assert!(harness.query_by_label("PC = 0x0000").is_some());

        // Remove breakpoint
        harness
            .get_by_role_and_label(accesskit::Role::Button, "Remove")
            .click();
        harness.run();

        assert!(harness.query_by_label("PC = 0x0000").is_none());
    }

    #[test]
    fn test_memory_debug_window_pc_breakpoint_add_via_input_and_toggle() {
        let mut debugger = TestDebugger::default();
        let mut window = MemoryDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ctx: &egui::Context| {
            window.ui(ctx, &mut debugger);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Select "Disassembly" view mode
        harness
            .get_all_by_role(accesskit::Role::ComboBox)
            .next()
            .unwrap()
            .click();
        harness.run();
        harness.get_by_label("Disassembly").click();
        harness.run();

        // Type in PC address
        harness
            .get_by_role_and_label(accesskit::Role::TextInput, "PC:")
            .type_text("0x0000");
        harness.run();

        // Add breakpoint
        harness
            .get_by_role_and_label(accesskit::Role::Button, "Add")
            .click();
        harness.run();

        // Disable breakpoint
        harness.get_by_label("PC = 0x0000").click();
        harness.run();
        drop(harness);

        assert_eq!(debugger.breakpoint_manager().breakpoints_iter().count(), 1);
        assert!(
            debugger
                .breakpoint_manager()
                .breakpoints_iter()
                .any(|(_, bp)| {
                    matches!(
                        bp,
                        AnyBreakpoint::CpuRegister16(CpuRegister16Breakpoint {
                            register: Register16::PC,
                            value: Some(0x0000),
                            ..
                        })
                    ) && !bp.enabled()
                })
        );
    }

    #[test]
    fn test_memory_debug_window_pc_breakpoint_via_disassembly() {
        let mut debugger = TestDebugger::default();
        let mut window = MemoryDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ctx: &egui::Context| {
            window.ui(ctx, &mut debugger);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Select "Disassembly" view mode
        harness
            .get_all_by_role(accesskit::Role::ComboBox)
            .next()
            .unwrap()
            .click();
        harness.run();
        harness.get_by_label("Disassembly").click();
        harness.run();

        // Click on address
        harness.get_by_label("  0000:").click();
        harness.run();

        assert!(harness.query_by_label("PC = 0x0000").is_some());

        // Click on address
        harness.get_by_label("● 0000:").click();
        harness.run();

        assert!(harness.query_by_label("PC = 0x0000").is_none());
    }

    #[test]
    fn test_memory_debug_window_jump_to_address_works_disassembly() {
        let mut debugger = TestDebugger::default();
        let mut window = MemoryDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ctx: &egui::Context| {
            window.ui(ctx, &mut debugger);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Select "Disassembly" view mode
        harness
            .get_all_by_role(accesskit::Role::ComboBox)
            .next()
            .unwrap()
            .click();
        harness.run();
        harness.get_by_label("Disassembly").click();
        harness.run();

        // Type 0xc000
        harness
            .get_by_role_and_label(accesskit::Role::TextInput, "Jump to address:")
            .type_text("0xc000");
        harness.run();

        // Jump to address
        harness
            .get_by_role_and_label(accesskit::Role::Button, "Go")
            .click();
        harness.run();

        assert!(harness.query_by_label("  C000:").is_some());
        assert!(harness.query_by_label("  0000:").is_none());

        // Track current PC
        harness
            .get_by_role_and_label(accesskit::Role::Button, "Track Current PC")
            .click();
        harness.run();

        assert!(harness.query_by_label("  0000:").is_some());
        assert!(harness.query_by_label("  C000:").is_none());
    }

    fn jump_to_address_works(view: &str) {
        let mut debugger = TestDebugger::default();
        let mut window = MemoryDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ctx: &egui::Context| {
            window.ui(ctx, &mut debugger);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Select view mode
        harness
            .get_all_by_role(accesskit::Role::ComboBox)
            .next()
            .unwrap()
            .click();
        harness.run();
        harness.get_by_label(view).click();
        harness.run();

        // Type 0x2000
        harness
            .get_by_role_and_label(accesskit::Role::TextInput, "Jump to address:")
            .type_text("0x2000");
        harness.run();

        let scroll_area = harness
            .get_by_label("2000:")
            .parent()
            .unwrap()
            .bounding_box()
            .unwrap();

        let address_label = harness.get_by_label("2000:").bounding_box().unwrap();
        assert!(!scroll_area.contains(address_label.origin()));

        // Jump to address
        harness
            .get_by_role_and_label(accesskit::Role::Button, "Go")
            .click();
        harness.run();

        let address_label = harness.get_by_label("2000:").bounding_box().unwrap();
        assert!(scroll_area.contains(address_label.origin()));
    }

    #[test]
    fn test_memory_debug_window_jump_to_address_works_composite_rom_ram() {
        jump_to_address_works("Composite ROM/RAM");
    }

    #[test]
    fn test_memory_debug_window_jump_to_address_works_composite_ram() {
        jump_to_address_works("Composite RAM");
    }

    #[test]
    fn test_memory_debug_window_jump_to_address_works_lower_rom() {
        jump_to_address_works("Lower ROM only");
    }

    #[test]
    fn test_memory_debug_window_jump_to_address_works_upper_rom() {
        jump_to_address_works("Upper ROM #00 only");
    }

    #[test]
    fn test_memory_debug_window_jump_to_address_works_ram() {
        jump_to_address_works("RAM only");
    }

    #[test]
    #[ignore = "extension ram not implemented"]
    fn test_memory_debug_window_jump_to_address_works_extension_ram() {
        jump_to_address_works("Extension RAM only");
    }

    fn pick_color_works(label: &str, snaphot: &str) {
        let mut debugger = TestDebugger::default();
        let mut window = MemoryDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ctx: &egui::Context| {
            window.ui(ctx, &mut debugger);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Show color configuration
        harness.get_by_label("Address Color Coding").click();
        harness.run();

        harness.snapshot("default_memory_colors");

        // Open color picker
        harness
            .get_by_role_and_label(accesskit::Role::ColorWell, label)
            .click();
        harness.run();

        // Edit red
        let value = harness
            .get_all_by_role(accesskit::Role::SpinButton)
            .next()
            .unwrap();
        value.click();
        value.type_text("255");
        harness.run();

        // Edit green
        let value = harness
            .get_all_by_role(accesskit::Role::SpinButton)
            .nth(1)
            .unwrap();
        value.click();
        value.type_text("255");
        harness.run();

        // Edit blue
        let value = harness
            .get_all_by_role(accesskit::Role::SpinButton)
            .nth(2)
            .unwrap();
        value.click();
        value.type_text("255");
        value.key_press(kittest::Key::Escape); // close color picker
        harness.run();

        harness.snapshot(snaphot);

        harness
            .get_by_role_and_label(accesskit::Role::Button, "Restore Defaults")
            .click();
        harness.run();

        harness.snapshot("memory_colors_defaults");
    }

    #[test]
    fn test_memory_debug_window_pick_color_lower_rom() {
        pick_color_works("Lower ROM:", "memory_colors_lower_rom_changed");
    }

    #[test]
    fn test_memory_debug_window_pick_color_upper_rom() {
        pick_color_works("Upper ROM:", "memory_colors_upper_rom_changed");
    }

    #[test]
    fn test_memory_debug_window_pick_color_ram() {
        pick_color_works("RAM:", "memory_colors_ram_changed");
    }

    #[test]
    fn test_memory_debug_window_pick_color_extension_ram() {
        pick_color_works("Extension RAM:", "memory_colors_extension_ram_changed");
    }
}
