use eframe::egui;
use serde::{Deserialize, Serialize};

use ronald_core::debug::breakpoint::{AnyBreakpoint, Breakpoint};
use ronald_core::system::bus::crtc::Register as CrtcRegister;

use crate::colors;
use crate::debug::Debugger;

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CrtcDebugWindow {
    pub show: bool,

    // Register write breakpoint
    #[serde(skip, default)]
    register_write_register: Option<CrtcRegister>,
    #[serde(skip, default)]
    register_write_value_input: String,
    #[serde(skip, default)]
    register_write_any_register: bool,
    #[serde(skip, default)]
    register_write_any_value: bool,

    // Counters breakpoint
    #[serde(skip, default)]
    character_row_value_input: String,
    #[serde(skip, default)]
    scan_line_value_input: String,
    #[serde(skip, default)]
    horizontal_counter_value_input: String,
    #[serde(skip, default)]
    character_row_any_value: bool,
    #[serde(skip, default)]
    scan_line_any_value: bool,
    #[serde(skip, default)]
    horizontal_counter_any_value: bool,

    // Address breakpoint
    #[serde(skip, default)]
    address_value_input: String,
    #[serde(skip, default)]
    address_any_value: bool,

    // Horizontal sync breakpoint
    #[serde(skip, default)]
    hsync_on_start: bool,
    #[serde(skip, default)]
    hsync_on_end: bool,

    // Vertical sync breakpoint
    #[serde(skip, default)]
    vsync_on_start: bool,
    #[serde(skip, default)]
    vsync_on_end: bool,

    // Display enable breakpoint
    #[serde(skip, default)]
    display_enable_on_start: bool,
    #[serde(skip, default)]
    display_enable_on_end: bool,
}

impl CrtcDebugWindow {
    pub fn ui(&mut self, ctx: &egui::Context, debugger: &mut impl Debugger) {
        let mut open = self.show;
        egui::Window::new("CRTC Internals")
            .resizable(false)
            .open(&mut open)
            .show(ctx, |ui| {
                self.render_crtc_state(ui, debugger);
                ui.separator();
                self.render_breakpoints_section(ui, debugger);
            });
        self.show = open;
    }

    fn render_crtc_state(&self, ui: &mut egui::Ui, debugger: &mut impl Debugger) {
        let debug_view = debugger.debug_view();
        let crtc = &debug_view.crtc;

        // Registers Section
        ui.heading("CRTC Registers");

        egui::Grid::new("crtc_registers_grid")
            .num_columns(4)
            .spacing([10.0, 4.0])
            .show(ui, |ui| {
                for (i, value) in crtc.registers.iter().enumerate() {
                    let register: CrtcRegister = i.into();
                    let is_selected = register == crtc.selected_register;

                    let label = format!("{}:", register);
                    if is_selected {
                        ui.colored_label(colors::DARK_YELLOW_GOLD, label);
                    } else {
                        ui.label(label);
                    }

                    ui.monospace(format!("{:02X}", value));

                    if (i + 1) % 2 == 0 {
                        ui.end_row();
                    } else {
                        ui.separator();
                    }
                }
            });

        ui.add_space(8.0);

        // Counters Section
        ui.heading("Counters");
        egui::Grid::new("crtc_counters_grid")
            .num_columns(2)
            .spacing([10.0, 4.0])
            .show(ui, |ui| {
                ui.label("Horizontal Counter:");
                ui.label(format!("{:02X}", crtc.horizontal_counter));
                ui.end_row();

                ui.label("Character Row Counter:");
                ui.label(format!("{:02X}", crtc.character_row_counter));
                ui.end_row();

                ui.label("Scan Line Counter:");
                ui.label(format!("{:02X}", crtc.scan_line_counter));
                ui.end_row();
            });

        ui.add_space(8.0);

        // Address Section
        ui.heading("Addresses");
        egui::Grid::new("crtc_addresses_grid")
            .num_columns(2)
            .spacing([10.0, 4.0])
            .show(ui, |ui| {
                ui.label("Display Start Address:");
                ui.monospace(format!("{:04X}", crtc.display_start_address));
                ui.end_row();

                ui.label("Current Address:");
                ui.monospace(format!("{:04X}", crtc.current_address));
                ui.end_row();
            });

        ui.add_space(8.0);

        // Status Section
        ui.heading("Status");
        ui.horizontal(|ui| {
            let hsync_color = if crtc.hsync_active {
                colors::FORREST_GREEN
            } else {
                colors::MEDIUM_GRAY
            };
            ui.colored_label(hsync_color, "HSYNC");

            ui.separator();

            let vsync_color = if crtc.vsync_active {
                colors::FORREST_GREEN
            } else {
                colors::MEDIUM_GRAY
            };
            ui.colored_label(vsync_color, "VSYNC");

            ui.separator();

            let display_color = if crtc.display_enabled {
                colors::FORREST_GREEN
            } else {
                colors::MEDIUM_GRAY
            };
            ui.colored_label(display_color, "DISPLAY");
        });
    }

    fn render_breakpoints_section(&mut self, ui: &mut egui::Ui, debugger: &mut impl Debugger) {
        ui.heading("CRTC Breakpoints");

        egui::Grid::new("crtc_breakpoint_grid")
            .num_columns(2)
            .spacing([10.0, 4.0])
            .show(ui, |ui| {
                // Register write breakpoint
                ui.label("Register write:");
                ui.horizontal(|ui| {
                    ui.add_enabled_ui(!self.register_write_any_register, |ui| {
                        egui::ComboBox::from_id_salt("crtc_register_selector")
                            .width(180.0)
                            .selected_text(match self.register_write_register {
                                Some(ref reg) => format!("{}", reg),
                                None => "Select register...".to_string(),
                            })
                            .show_ui(ui, |ui| {
                                for i in 0..18 {
                                    let reg: CrtcRegister = i.into();
                                    ui.selectable_value(
                                        &mut self.register_write_register,
                                        Some(reg),
                                        format!("{}", reg),
                                    );
                                }
                            });
                    });

                    if ui
                        .checkbox(&mut self.register_write_any_register, "Any")
                        .changed()
                        && self.register_write_any_register
                    {
                        self.register_write_register = None;
                    }

                    let label = ui.label("Value:");
                    ui.add_enabled(
                        !self.register_write_any_value,
                        egui::TextEdit::singleline(&mut self.register_write_value_input)
                            .desired_width(40.0),
                    )
                    .labelled_by(label.id)
                    .on_hover_text("Hex value (e.g., 1000 or 0x1000)");

                    if ui
                        .checkbox(&mut self.register_write_any_value, "Any")
                        .changed()
                        && self.register_write_any_value
                    {
                        self.register_write_value_input.clear();
                    }

                    if ui.button("Add").clicked() {
                        self.add_register_write_breakpoint(debugger);
                    }
                });
                ui.end_row();

                // Counters breakpoint
                ui.label("Counters:");
                ui.horizontal(|ui| {
                    let label = ui.label("Char. row:");
                    ui.add_enabled(
                        !self.character_row_any_value,
                        egui::TextEdit::singleline(&mut self.character_row_value_input)
                            .desired_width(40.0),
                    )
                    .labelled_by(label.id)
                    .on_hover_text("Hex value (e.g., 10 or 0x10)");

                    if ui
                        .checkbox(&mut self.character_row_any_value, "Any")
                        .changed()
                        && self.character_row_any_value
                    {
                        self.character_row_value_input.clear();
                    }

                    let label = ui.label("Scan line:");
                    ui.add_enabled(
                        !self.scan_line_any_value,
                        egui::TextEdit::singleline(&mut self.scan_line_value_input)
                            .desired_width(40.0),
                    )
                    .labelled_by(label.id)
                    .on_hover_text("Hex value (e.g., 10 or 0x10)");

                    if ui.checkbox(&mut self.scan_line_any_value, "Any").changed()
                        && self.scan_line_any_value
                    {
                        self.scan_line_value_input.clear();
                    }

                    let label = ui.label("Horizontal:");
                    ui.add_enabled(
                        !self.horizontal_counter_any_value,
                        egui::TextEdit::singleline(&mut self.horizontal_counter_value_input)
                            .desired_width(40.0),
                    )
                    .labelled_by(label.id)
                    .on_hover_text("Hex value (e.g., 10 or 0x10)");

                    if ui
                        .checkbox(&mut self.horizontal_counter_any_value, "Any")
                        .changed()
                        && self.horizontal_counter_any_value
                    {
                        self.horizontal_counter_value_input.clear();
                    }

                    if ui.button("Add").clicked() {
                        self.add_counters_breakpoint(debugger);
                    }
                });
                ui.end_row();

                // Address breakpoint
                let label = ui.label("Address:");
                ui.horizontal(|ui| {
                    ui.add_enabled(
                        !self.address_any_value,
                        egui::TextEdit::singleline(&mut self.address_value_input)
                            .desired_width(60.0),
                    )
                    .labelled_by(label.id)
                    .on_hover_text("Hex value (e.g., 1000 or 0x1000)");

                    ui.checkbox(&mut self.address_any_value, "Any");

                    if ui.button("Add").clicked() {
                        self.add_address_breakpoint(debugger);
                    }
                });
                ui.end_row();

                // Horizontal sync breakpoint
                ui.label("Horizontal sync:");
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.hsync_on_start, "Start");
                    ui.checkbox(&mut self.hsync_on_end, "End");

                    if ui.button("Add").clicked() {
                        self.add_horizontal_sync_breakpoint(debugger);
                    }
                });
                ui.end_row();

                // Vertical sync breakpoint
                ui.label("Vertical sync:");
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.vsync_on_start, "Start");
                    ui.checkbox(&mut self.vsync_on_end, "End");

                    if ui.button("Add").clicked() {
                        self.add_vertical_sync_breakpoint(debugger);
                    }
                });
                ui.end_row();

                // Display enable breakpoint
                ui.label("Display enable:");
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.display_enable_on_start, "Start");
                    ui.checkbox(&mut self.display_enable_on_end, "End");

                    if ui.button("Add").clicked() {
                        self.add_display_enable_breakpoint(debugger);
                    }
                });
                ui.end_row();
            });

        // List active CRTC breakpoints
        ui.separator();
        ui.label("Active CRTC Breakpoints:");

        let mut breakpoint_found = false;
        let mut to_remove = None;
        let mut to_toggle = None;

        let breakpoint_manager = debugger.breakpoint_manager();
        for (id, breakpoint) in breakpoint_manager.breakpoints_iter() {
            if breakpoint.one_shot()
                || !matches!(
                    breakpoint,
                    AnyBreakpoint::CrtcRegisterWrite(_)
                        | AnyBreakpoint::CrtcCounters(_)
                        | AnyBreakpoint::CrtcAddress(_)
                        | AnyBreakpoint::CrtcHorizontalSync(_)
                        | AnyBreakpoint::CrtcVerticalSync(_)
                        | AnyBreakpoint::CrtcDisplayEnable(_)
                )
            {
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
            ui.label("No CRTC breakpoints set");
        }

        // Apply changes
        if let Some((id, enabled)) = to_toggle {
            breakpoint_manager.enable_breakpoint(id, enabled);
        }
        if let Some(id) = to_remove {
            breakpoint_manager.remove_breakpoint(id);
        }
    }

    fn add_register_write_breakpoint(&mut self, debugger: &mut impl Debugger) {
        let register = if self.register_write_any_register {
            None
        } else {
            match self.register_write_register {
                Some(reg) => Some(reg),
                None => return, // No register selected, don't add breakpoint
            }
        };

        let value = if self.register_write_any_value {
            None
        } else {
            match usize::from_str_radix(
                self.register_write_value_input.trim_start_matches("0x"),
                16,
            ) {
                Ok(val) => Some((val & 0xFF) as u8),
                Err(_) => return, // Invalid input, don't add breakpoint
            }
        };

        let breakpoint = AnyBreakpoint::crtc_register_write_breakpoint(register, value);
        debugger.breakpoint_manager().add_breakpoint(breakpoint);

        self.register_write_register = None;
        self.register_write_any_register = false;
        self.register_write_value_input.clear();
        self.register_write_any_value = false;
    }

    fn add_counters_breakpoint(&mut self, debugger: &mut impl Debugger) {
        let character_row = if self.character_row_any_value {
            None
        } else {
            match usize::from_str_radix(self.character_row_value_input.trim_start_matches("0x"), 16)
            {
                Ok(val) => Some((val & 0xFF) as u8),
                Err(_) => return, // Invalid input, don't add breakpoint
            }
        };

        let scan_line = if self.scan_line_any_value {
            None
        } else {
            match usize::from_str_radix(self.scan_line_value_input.trim_start_matches("0x"), 16) {
                Ok(val) => Some((val & 0xFF) as u8),
                Err(_) => return, // Invalid input, don't add breakpoint
            }
        };

        let horizontal_counter = if self.horizontal_counter_any_value {
            None
        } else {
            match usize::from_str_radix(
                self.horizontal_counter_value_input.trim_start_matches("0x"),
                16,
            ) {
                Ok(val) => Some((val & 0xFF) as u8),
                Err(_) => return, // Invalid input, don't add breakpoint
            }
        };

        let breakpoint =
            AnyBreakpoint::crtc_counters_breakpoint(character_row, scan_line, horizontal_counter);
        debugger.breakpoint_manager().add_breakpoint(breakpoint);

        self.character_row_value_input.clear();
        self.character_row_any_value = false;
        self.scan_line_value_input.clear();
        self.scan_line_any_value = false;
        self.horizontal_counter_value_input.clear();
        self.horizontal_counter_any_value = false;
    }

    fn add_address_breakpoint(&mut self, debugger: &mut impl Debugger) {
        let address = if self.address_any_value {
            None
        } else {
            match usize::from_str_radix(self.address_value_input.trim_start_matches("0x"), 16) {
                Ok(val) => Some(val & 0xFFFF),
                Err(_) => return, // Invalid input, don't add breakpoint
            }
        };

        let breakpoint = AnyBreakpoint::crtc_address_breakpoint(address);
        debugger.breakpoint_manager().add_breakpoint(breakpoint);

        self.address_value_input.clear();
        self.address_any_value = false;
    }

    fn add_horizontal_sync_breakpoint(&mut self, debugger: &mut impl Debugger) {
        let breakpoint =
            AnyBreakpoint::crtc_horizontal_sync_breakpoint(self.hsync_on_start, self.hsync_on_end);
        debugger.breakpoint_manager().add_breakpoint(breakpoint);

        self.hsync_on_start = false;
        self.hsync_on_end = false;
    }

    fn add_vertical_sync_breakpoint(&mut self, debugger: &mut impl Debugger) {
        let breakpoint =
            AnyBreakpoint::crtc_vertical_sync_breakpoint(self.vsync_on_start, self.vsync_on_end);
        debugger.breakpoint_manager().add_breakpoint(breakpoint);

        self.vsync_on_start = false;
        self.vsync_on_end = false;
    }

    fn add_display_enable_breakpoint(&mut self, debugger: &mut impl Debugger) {
        let breakpoint = AnyBreakpoint::crtc_dispaly_enable_breakpoint(
            self.display_enable_on_start,
            self.display_enable_on_end,
        );
        debugger.breakpoint_manager().add_breakpoint(breakpoint);

        self.display_enable_on_start = false;
        self.display_enable_on_end = false;
    }
}

#[cfg(test)]
mod gui_tests {
    use super::*;

    use egui::accesskit;
    use egui_kittest::{Harness, kittest::Queryable};

    use ronald_core::debug::breakpoint::{
        CrtcAddressBreakpoint, CrtcCountersBreakpoint, CrtcRegisterWriteBreakpoint,
    };

    use crate::debug::mock::TestDebugger;

    #[test]
    fn test_crtc_debug_window_opens_and_closes() {
        let mut debugger = TestDebugger::default();
        let mut window = CrtcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ctx: &egui::Context| {
            window.ui(ctx, &mut debugger);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Check that the window title is rendered
        harness.get_by_label("CRTC Internals");

        // Click close button
        harness.get_by_label("Close window").click();
        harness.run();

        // Window should no longer be visible
        assert!(harness.query_by_label("CRTC Internals").is_none());
    }

    #[test]
    fn test_crtc_debug_window_register_breakpoint_with_register_and_value() {
        let mut debugger = TestDebugger::default();
        let mut window = CrtcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ctx: &egui::Context| {
            window.ui(ctx, &mut debugger);
        };

        let mut harness = Harness::new(app);
        harness.run();

        let i = 0;

        // Select register "Vertical Total"
        harness
            .get_all_by_role(accesskit::Role::ComboBox)
            .nth(i)
            .unwrap()
            .click();
        harness.run();
        harness.get_by_label("R4 (Vertical Total)").click();
        harness.run();

        // Enter value "0x42"
        harness
            .get_all_by_role_and_label(accesskit::Role::TextInput, "Value:")
            .nth(i)
            .unwrap()
            .type_text("0x42");
        harness.run();

        // Add breakpoint
        harness
            .get_all_by_role_and_label(accesskit::Role::Button, "Add")
            .nth(i)
            .unwrap()
            .click();
        harness.run();

        assert!(
            harness
                .query_by_label("R4 (Vertical Total) = 0x42")
                .is_some()
        );

        // Remove breakpoint
        harness
            .get_by_role_and_label(accesskit::Role::Button, "Remove")
            .click();
        harness.run();

        assert!(
            harness
                .query_by_label("R4 (Vertical Total) = 0x42")
                .is_none()
        );
    }

    #[test]
    fn test_crtc_debug_window_register_breakpoint_any_register() {
        let mut debugger = TestDebugger::default();
        let mut window = CrtcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ctx: &egui::Context| {
            window.ui(ctx, &mut debugger);
        };

        let mut harness = Harness::new(app);
        harness.run();

        let i = 0;

        // Select any register
        harness
            .get_all_by_role_and_label(accesskit::Role::CheckBox, "Any")
            .nth(i)
            .unwrap()
            .click();
        harness.run();

        // Enter value "0x42"
        harness
            .get_all_by_role_and_label(accesskit::Role::TextInput, "Value:")
            .nth(i)
            .unwrap()
            .type_text("0x42");
        harness.run();

        // Add breakpoint
        harness
            .get_all_by_role_and_label(accesskit::Role::Button, "Add")
            .nth(i)
            .unwrap()
            .click();
        harness.run();

        harness.get_by_label("Any register = 0x42").click();
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
                        AnyBreakpoint::CrtcRegisterWrite(CrtcRegisterWriteBreakpoint {
                            register: None,
                            value: Some(0x42),
                            ..
                        })
                    ) && !bp.enabled()
                })
        );
    }

    #[test]
    fn test_crtc_debug_window_register_breakpoint_any_value() {
        let mut debugger = TestDebugger::default();
        let mut window = CrtcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ctx: &egui::Context| {
            window.ui(ctx, &mut debugger);
        };

        let mut harness = Harness::new(app);
        harness.run();

        let i = 0;

        // Select register "Vertical Total"
        harness
            .get_all_by_role(accesskit::Role::ComboBox)
            .nth(i)
            .unwrap()
            .click();
        harness.run();
        harness.get_by_label("R4 (Vertical Total)").click();
        harness.run();

        // Check any value
        harness
            .get_all_by_role_and_label(accesskit::Role::CheckBox, "Any")
            .nth(i + 1)
            .unwrap()
            .click();
        harness.run();

        // Add breakpoint
        harness
            .get_all_by_role_and_label(accesskit::Role::Button, "Add")
            .nth(i)
            .unwrap()
            .click();
        harness.run();

        // Disable breakpoint
        harness.get_by_label("R4 (Vertical Total) written").click();
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
                        AnyBreakpoint::CrtcRegisterWrite(CrtcRegisterWriteBreakpoint {
                            register: Some(CrtcRegister::VerticalTotal),
                            value: None,
                            ..
                        })
                    ) && !bp.enabled()
                })
        );
    }

    #[test]
    fn test_crtc_debug_window_register_breakpoint_invalid_value() {
        let mut debugger = TestDebugger::default();
        let mut window = CrtcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ctx: &egui::Context| {
            window.ui(ctx, &mut debugger);
        };

        let mut harness = Harness::new(app);
        harness.run();

        let i = 0;

        // Select register "Vertical Total"
        harness
            .get_all_by_role(accesskit::Role::ComboBox)
            .nth(i)
            .unwrap()
            .click();
        harness.run();
        harness.get_by_label("R4 (Vertical Total)").click();
        harness.run();

        // Enter value "invalid"
        harness
            .get_all_by_role_and_label(accesskit::Role::TextInput, "Value:")
            .nth(i)
            .unwrap()
            .type_text("invalid");
        harness.run();

        // Add breakpoint
        harness
            .get_all_by_role_and_label(accesskit::Role::Button, "Add")
            .nth(i)
            .unwrap()
            .click();
        harness.run();
        drop(harness);

        assert_eq!(debugger.breakpoint_manager().breakpoints_iter().count(), 0);
    }

    #[test]
    fn test_crtc_debug_window_counters_breakpoint_with_values() {
        let mut debugger = TestDebugger::default();
        let mut window = CrtcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ctx: &egui::Context| {
            window.ui(ctx, &mut debugger);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Enter character row value "0x42"
        harness
            .get_by_role_and_label(accesskit::Role::TextInput, "Char. row:")
            .type_text("0x42");
        harness.run();

        // Enter scan line value "0x08"
        harness
            .get_by_role_and_label(accesskit::Role::TextInput, "Scan line:")
            .type_text("0x08");
        harness.run();

        // Enter horizontal value "0xaf"
        harness
            .get_by_role_and_label(accesskit::Role::TextInput, "Horizontal:")
            .type_text("0xaf");
        harness.run();

        // Add breakpoint
        harness
            .get_all_by_role_and_label(accesskit::Role::Button, "Add")
            .nth(1)
            .unwrap()
            .click();
        harness.run();

        assert!(
            harness
                .query_by_label("Counters = 0x42/0x08/0xAF")
                .is_some()
        );

        // Remove breakpoint
        harness
            .get_by_role_and_label(accesskit::Role::Button, "Remove")
            .click();
        harness.run();

        assert!(
            harness
                .query_by_label("Counters = 0x42/0x08/0xAF")
                .is_none()
        );
    }

    #[test]
    fn test_crtc_debug_window_counters_breakpoint_invalid_character_row() {
        let mut debugger = TestDebugger::default();
        let mut window = CrtcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ctx: &egui::Context| {
            window.ui(ctx, &mut debugger);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Enter character row value "invalid"
        harness
            .get_by_role_and_label(accesskit::Role::TextInput, "Char. row:")
            .type_text("invalid");
        harness.run();

        // Enter scan line value "0x08"
        harness
            .get_by_role_and_label(accesskit::Role::TextInput, "Scan line:")
            .type_text("0x08");
        harness.run();

        // Enter horizontal value "0xaf"
        harness
            .get_by_role_and_label(accesskit::Role::TextInput, "Horizontal:")
            .type_text("0xaf");
        harness.run();

        // Add breakpoint
        harness
            .get_all_by_role_and_label(accesskit::Role::Button, "Add")
            .nth(1)
            .unwrap()
            .click();
        harness.run();
        drop(harness);

        assert_eq!(debugger.breakpoint_manager().breakpoints_iter().count(), 0);
    }

    #[test]
    fn test_crtc_debug_window_counters_breakpoint_invalid_scan_line() {
        let mut debugger = TestDebugger::default();
        let mut window = CrtcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ctx: &egui::Context| {
            window.ui(ctx, &mut debugger);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Enter character row value "0x42"
        harness
            .get_by_role_and_label(accesskit::Role::TextInput, "Char. row:")
            .type_text("0x42");
        harness.run();

        // Enter scan line value "invalid"
        harness
            .get_by_role_and_label(accesskit::Role::TextInput, "Scan line:")
            .type_text("invalid");
        harness.run();

        // Enter horizontal value "0xaf"
        harness
            .get_by_role_and_label(accesskit::Role::TextInput, "Horizontal:")
            .type_text("0xaf");
        harness.run();

        // Add breakpoint
        harness
            .get_all_by_role_and_label(accesskit::Role::Button, "Add")
            .nth(1)
            .unwrap()
            .click();
        harness.run();
        drop(harness);

        assert_eq!(debugger.breakpoint_manager().breakpoints_iter().count(), 0);
    }

    #[test]
    fn test_crtc_debug_window_counters_breakpoint_invalid_horizontal() {
        let mut debugger = TestDebugger::default();
        let mut window = CrtcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ctx: &egui::Context| {
            window.ui(ctx, &mut debugger);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Enter character row value "0x42"
        harness
            .get_by_role_and_label(accesskit::Role::TextInput, "Char. row:")
            .type_text("0x42");
        harness.run();

        // Enter scan line value "0x08"
        harness
            .get_by_role_and_label(accesskit::Role::TextInput, "Scan line:")
            .type_text("0x08");
        harness.run();

        // Enter horizontal value "invalid"
        harness
            .get_by_role_and_label(accesskit::Role::TextInput, "Horizontal:")
            .type_text("invalid");
        harness.run();

        // Add breakpoint
        harness
            .get_all_by_role_and_label(accesskit::Role::Button, "Add")
            .nth(1)
            .unwrap()
            .click();
        harness.run();
        drop(harness);

        assert_eq!(debugger.breakpoint_manager().breakpoints_iter().count(), 0);
    }

    #[test]
    fn test_crtc_debug_window_counters_breakpoint_any_values() {
        let mut debugger = TestDebugger::default();
        let mut window = CrtcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ctx: &egui::Context| {
            window.ui(ctx, &mut debugger);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Enter character row value "0x42"
        harness
            .get_all_by_role_and_label(accesskit::Role::CheckBox, "Any")
            .nth(2)
            .unwrap()
            .click();
        harness.run();

        // Enter scan line value "0x42"
        harness
            .get_all_by_role_and_label(accesskit::Role::CheckBox, "Any")
            .nth(3)
            .unwrap()
            .click();
        harness.run();

        // Enter horizontal value "0x42"
        harness
            .get_all_by_role_and_label(accesskit::Role::CheckBox, "Any")
            .nth(4)
            .unwrap()
            .click();
        harness.run();

        // Add breakpoint
        harness
            .get_all_by_role_and_label(accesskit::Role::Button, "Add")
            .nth(1)
            .unwrap()
            .click();
        harness.run();

        // Disable breakpoint
        harness.get_by_label("Counters = Any/Any/Any").click();
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
                        AnyBreakpoint::CrtcCounters(CrtcCountersBreakpoint {
                            character_row: None,
                            scan_line: None,
                            horizontal_counter: None,
                            ..
                        })
                    ) && !bp.enabled()
                })
        );
    }

    #[test]
    fn test_crtc_debug_window_address_breakpoint_with_value() {
        let mut debugger = TestDebugger::default();
        let mut window = CrtcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ctx: &egui::Context| {
            window.ui(ctx, &mut debugger);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Enter character row value "0x42"
        harness
            .get_by_role_and_label(accesskit::Role::TextInput, "Address:")
            .type_text("0xbeef");
        harness.run();

        // Add breakpoint
        harness
            .get_all_by_role_and_label(accesskit::Role::Button, "Add")
            .nth(2)
            .unwrap()
            .click();
        harness.run();

        assert!(harness.query_by_label("Address = 0xBEEF").is_some());

        // Remove breakpoint
        harness
            .get_by_role_and_label(accesskit::Role::Button, "Remove")
            .click();
        harness.run();

        assert!(harness.query_by_label("Address = 0xBEEF").is_none());
    }

    #[test]
    fn test_crtc_debug_window_address_breakpoint_any_value() {
        let mut debugger = TestDebugger::default();
        let mut window = CrtcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ctx: &egui::Context| {
            window.ui(ctx, &mut debugger);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Enter character row value "0x42"
        harness
            .get_all_by_role_and_label(accesskit::Role::CheckBox, "Any")
            .nth(5)
            .unwrap()
            .click();
        harness.run();

        // Add breakpoint
        harness
            .get_all_by_role_and_label(accesskit::Role::Button, "Add")
            .nth(2)
            .unwrap()
            .click();
        harness.run();

        // Disable breakpoint
        harness.get_by_label("Any address change").click();
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
                        AnyBreakpoint::CrtcAddress(CrtcAddressBreakpoint { value: None, .. })
                    ) && !bp.enabled()
                })
        );
    }

    #[test]
    fn test_crtc_debug_window_hsync_breakpoint() {
        let mut debugger = TestDebugger::default();
        let mut window = CrtcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ctx: &egui::Context| {
            window.ui(ctx, &mut debugger);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Check start
        harness
            .get_all_by_role_and_label(accesskit::Role::CheckBox, "Start")
            .next()
            .unwrap()
            .click();
        harness.run();

        // Check end
        harness
            .get_all_by_role_and_label(accesskit::Role::CheckBox, "End")
            .next()
            .unwrap()
            .click();
        harness.run();

        // Add breakpoint
        harness
            .get_all_by_role_and_label(accesskit::Role::Button, "Add")
            .nth(3)
            .unwrap()
            .click();
        harness.run();

        assert!(harness.query_by_label("HSYNC start or end").is_some());

        // Remove breakpoint
        harness
            .get_by_role_and_label(accesskit::Role::Button, "Remove")
            .click();
        harness.run();

        assert!(harness.query_by_label("HSYNC start or end").is_none());
    }

    #[test]
    fn test_crtc_debug_window_vsync_breakpoint() {
        let mut debugger = TestDebugger::default();
        let mut window = CrtcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ctx: &egui::Context| {
            window.ui(ctx, &mut debugger);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Check start
        harness
            .get_all_by_role_and_label(accesskit::Role::CheckBox, "Start")
            .nth(1)
            .unwrap()
            .click();
        harness.run();

        // Check end
        harness
            .get_all_by_role_and_label(accesskit::Role::CheckBox, "End")
            .nth(1)
            .unwrap()
            .click();
        harness.run();

        // Add breakpoint
        harness
            .get_all_by_role_and_label(accesskit::Role::Button, "Add")
            .nth(4)
            .unwrap()
            .click();
        harness.run();

        assert!(harness.query_by_label("VSYNC start or end").is_some());

        // Remove breakpoint
        harness
            .get_by_role_and_label(accesskit::Role::Button, "Remove")
            .click();
        harness.run();

        assert!(harness.query_by_label("VSYNC start or end").is_none());
    }

    #[test]
    fn test_crtc_debug_window_display_enable_breakpoint() {
        let mut debugger = TestDebugger::default();
        let mut window = CrtcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ctx: &egui::Context| {
            window.ui(ctx, &mut debugger);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Check start
        harness
            .get_all_by_role_and_label(accesskit::Role::CheckBox, "Start")
            .nth(2)
            .unwrap()
            .click();
        harness.run();

        // Check end
        harness
            .get_all_by_role_and_label(accesskit::Role::CheckBox, "End")
            .nth(2)
            .unwrap()
            .click();
        harness.run();

        // Add breakpoint
        harness
            .get_all_by_role_and_label(accesskit::Role::Button, "Add")
            .nth(5)
            .unwrap()
            .click();
        harness.run();

        assert!(
            harness
                .query_by_label("DISP. ENABLE start or end")
                .is_some()
        );

        // Remove breakpoint
        harness
            .get_by_role_and_label(accesskit::Role::Button, "Remove")
            .click();
        harness.run();

        assert!(
            harness
                .query_by_label("DISP. ENABLE start or end")
                .is_none()
        );
    }
}
