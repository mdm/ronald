use eframe::egui;
use serde::{Deserialize, Serialize};

use ronald_core::debug::breakpoint::{AnyBreakpoint, Breakpoint};
use ronald_core::system::bus::fdc::{Phase, Register};

use crate::colors;
use crate::debug::Debugger;

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FdcDebugWindow {
    pub show: bool,

    // Register read breakpoint
    #[serde(skip, default)]
    register_read_register: Option<Register>,
    #[serde(skip, default)]
    register_read_value_input: String,
    #[serde(skip, default)]
    register_read_any_register: bool,
    #[serde(skip, default)]
    register_read_any_value: bool,

    // Register write breakpoint
    #[serde(skip, default)]
    register_write_register: Option<Register>,
    #[serde(skip, default)]
    register_write_value_input: String,
    #[serde(skip, default)]
    register_write_any_register: bool,
    #[serde(skip, default)]
    register_write_any_value: bool,

    // Phase change breakpoint
    #[serde(skip, default)]
    phase: Option<Phase>,
    #[serde(skip, default)]
    phase_on_enter: bool,
    #[serde(skip, default)]
    phase_on_leave: bool,
}

impl FdcDebugWindow {
    pub fn ui(&mut self, ctx: &egui::Context, debugger: &mut impl Debugger) {
        let mut open = self.show;
        egui::Window::new("FDC Internals")
            .resizable(false)
            .open(&mut open)
            .show(ctx, |ui| {
                self.render_fdc_state(ui, debugger);
                ui.separator();
                self.render_breakpoints_section(ui, debugger);
            });
        self.show = open;
    }

    fn render_fdc_state(&self, ui: &mut egui::Ui, debugger: &mut impl Debugger) {
        let debug_view = debugger.debug_view();
        let fdc = &debug_view.fdc;

        // Registers Section
        ui.heading("Buffers");

        egui::Grid::new("fdc_buffers_grid")
            .num_columns(2)
            .spacing([10.0, 4.0])
            .show(ui, |ui| {
                ui.label("Command Buffer:");
                self.render_buffer(ui, &fdc.command_buffer);
                ui.end_row();

                ui.label("Data Buffer:");
                self.render_buffer(ui, &fdc.data_buffer);
                ui.end_row();

                ui.label("Result Buffer:");
                self.render_buffer(ui, &fdc.result_buffer);
                ui.end_row();

                // for (i, value) in crtc.registers.iter().enumerate() {
                //     let register = CrtcRegister::try_from(i).unwrap();
                //     let is_selected = register == crtc.selected_register;
                //
                //     let label = format!("{}:", register);
                //     if is_selected {
                //         ui.colored_label(colors::DARK_YELLOW_GOLD, label);
                //     } else {
                //         ui.label(label);
                //     }
                //
                //     ui.monospace(format!("{:02X}", value));
                //
                //     if (i + 1) % 2 == 0 {
                //         ui.end_row();
                //     } else {
                //         ui.separator();
                //     }
                // }
                //
                // let register = CrtcRegister::Unused;
                // let is_selected = register == crtc.selected_register;
                //
                // let label = format!("{}:", register);
                // if is_selected {
                //     ui.colored_label(colors::DARK_YELLOW_GOLD, label);
                // } else {
                //     ui.label(label);
                // }
                //
                // ui.monospace("-");
                // ui.separator();
                //
                // let register = CrtcRegister::Dummy;
                // let is_selected = register == crtc.selected_register;
                //
                // let label = format!("{}:", register);
                // if is_selected {
                //     ui.colored_label(colors::DARK_YELLOW_GOLD, label);
                // } else {
                //     ui.label(label);
                // }
                //
                // ui.monospace("-");
                // ui.end_row();
            });

        // ui.add_space(8.0);
        //
        // // Counters Section
        // ui.heading("Counters");
        // egui::Grid::new("crtc_counters_grid")
        //     .num_columns(2)
        //     .spacing([10.0, 4.0])
        //     .show(ui, |ui| {
        //         ui.label("Horizontal Counter:");
        //         ui.label(format!("{:02X}", crtc.horizontal_counter));
        //         ui.end_row();
        //
        //         ui.label("Character Row Counter:");
        //         ui.label(format!("{:02X}", crtc.character_row_counter));
        //         ui.end_row();
        //
        //         ui.label("Scan Line Counter:");
        //         ui.label(format!("{:02X}", crtc.scan_line_counter));
        //         ui.end_row();
        //     });
        //
        // ui.add_space(8.0);
        //
        // // Address Section
        // ui.heading("Addresses");
        // egui::Grid::new("crtc_addresses_grid")
        //     .num_columns(2)
        //     .spacing([10.0, 4.0])
        //     .show(ui, |ui| {
        //         ui.label("Display Start Address:");
        //         ui.monospace(format!("{:04X}", crtc.display_start_address));
        //         ui.end_row();
        //
        //         ui.label("Current Address:");
        //         ui.monospace(format!("{:04X}", crtc.current_address));
        //         ui.end_row();
        //     });
        //
        // ui.add_space(8.0);
        //
        // // Status Section
        // ui.heading("Status");
        // ui.horizontal(|ui| {
        //     let hsync_color = if crtc.hsync_active {
        //         colors::FORREST_GREEN
        //     } else {
        //         colors::MEDIUM_GRAY
        //     };
        //     ui.colored_label(hsync_color, "HSYNC");
        //
        //     ui.separator();
        //
        //     let vsync_color = if crtc.vsync_active {
        //         colors::FORREST_GREEN
        //     } else {
        //         colors::MEDIUM_GRAY
        //     };
        //     ui.colored_label(vsync_color, "VSYNC");
        //
        //     ui.separator();
        //
        //     let display_color = if crtc.display_enabled {
        //         colors::FORREST_GREEN
        //     } else {
        //         colors::MEDIUM_GRAY
        //     };
        //     ui.colored_label(display_color, "DISPLAY");
        // });
    }

    fn render_buffer(&self, ui: &mut egui::Ui, buffer: &[u8]) {
        ui.vertical(|ui| {
            for chunk in buffer.chunks(16) {
                ui.horizontal(|ui| {
                    for (i, byte) in chunk.iter().enumerate() {
                        if i == 8 {
                            ui.label(" ");
                        }
                        ui.label(format!("{:02X}", byte));
                    }
                });
            }
        });
    }

    fn render_breakpoints_section(&mut self, ui: &mut egui::Ui, debugger: &mut impl Debugger) {
        ui.heading("FDC Breakpoints");

        egui::Grid::new("fdc_breakpoint_grid")
            .num_columns(2)
            .spacing([10.0, 4.0])
            .show(ui, |ui| {
                // Register read breakpoint
                ui.label("Register read:");
                ui.horizontal(|ui| {
                    ui.add_enabled_ui(!self.register_read_any_register, |ui| {
                        egui::ComboBox::from_id_salt("fdc_register_read_selector")
                            .width(180.0)
                            .selected_text(match self.register_read_register {
                                Some(ref reg) => format!("{}", reg),
                                None => "Select register...".to_string(),
                            })
                            .show_ui(ui, |ui| {
                                let reg = Register::Data;
                                ui.selectable_value(
                                    &mut self.register_read_register,
                                    Some(reg),
                                    format!("{}", reg),
                                );
                                let reg = Register::MainStatus;
                                ui.selectable_value(
                                    &mut self.register_read_register,
                                    Some(reg),
                                    format!("{}", reg),
                                );
                            });
                    });

                    if ui
                        .checkbox(&mut self.register_read_any_register, "Any")
                        .changed()
                        && self.register_read_any_register
                    {
                        self.register_read_register = None;
                    }

                    let label = ui.label("Value:");
                    ui.add_enabled(
                        !self.register_read_any_value,
                        egui::TextEdit::singleline(&mut self.register_read_value_input)
                            .desired_width(40.0),
                    )
                    .labelled_by(label.id)
                    .on_hover_text("Hex value (e.g., 1000 or 0x1000)");

                    if ui
                        .checkbox(&mut self.register_read_any_value, "Any")
                        .changed()
                        && self.register_read_any_value
                    {
                        self.register_read_value_input.clear();
                    }

                    if ui.button("Add").clicked() {
                        self.add_register_read_breakpoint(debugger);
                    }
                });
                ui.end_row();

                // Register write breakpoint
                ui.label("Register write:");
                ui.horizontal(|ui| {
                    ui.add_enabled_ui(!self.register_write_any_register, |ui| {
                        egui::ComboBox::from_id_salt("fdc_register_write_selector")
                            .width(180.0)
                            .selected_text(match self.register_write_register {
                                Some(ref reg) => format!("{}", reg),
                                None => "Select register...".to_string(),
                            })
                            .show_ui(ui, |ui| {
                                let reg = Register::Data;
                                ui.selectable_value(
                                    &mut self.register_write_register,
                                    Some(reg),
                                    format!("{}", reg),
                                );
                                let reg = Register::MotorControl;
                                ui.selectable_value(
                                    &mut self.register_write_register,
                                    Some(reg),
                                    format!("{}", reg),
                                );
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

                // Phase change breakpoint
                ui.label("Phase change:");
                ui.horizontal(|ui| {
                    egui::ComboBox::from_id_salt("fdc_phase_change_selector")
                        .width(180.0)
                        .selected_text(match self.phase {
                            Some(ref reg) => format!("{}", reg),
                            None => "Select phase...".to_string(),
                        })
                        .show_ui(ui, |ui| {
                            let reg = Phase::Command;
                            ui.selectable_value(&mut self.phase, Some(reg), format!("{}", reg));
                            let reg = Phase::Execution;
                            ui.selectable_value(&mut self.phase, Some(reg), format!("{}", reg));
                            let reg = Phase::Result;
                            ui.selectable_value(&mut self.phase, Some(reg), format!("{}", reg));
                        });
                    ui.checkbox(&mut self.phase_on_enter, "Enter");
                    ui.checkbox(&mut self.phase_on_leave, "Leave");

                    if ui.button("Add").clicked() {
                        self.add_phase_breakpoint(debugger);
                    }
                });
                ui.end_row();
            });

        // List active CRTC breakpoints
        ui.separator();
        ui.label("Active FDC Breakpoints:");

        let mut breakpoint_found = false;
        let mut to_remove = None;
        let mut to_toggle = None;

        let breakpoint_manager = debugger.breakpoint_manager();
        for (id, breakpoint) in breakpoint_manager.breakpoints_iter() {
            if breakpoint.one_shot()
                || !matches!(
                    breakpoint,
                    AnyBreakpoint::FdcRegister(_) | AnyBreakpoint::FdcPhase(_)
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
            ui.label("No FDC breakpoints set");
        }

        // Apply changes
        if let Some((id, enabled)) = to_toggle {
            breakpoint_manager.enable_breakpoint(id, enabled);
        }
        if let Some(id) = to_remove {
            breakpoint_manager.remove_breakpoint(id);
        }
    }

    fn add_register_read_breakpoint(&mut self, debugger: &mut impl Debugger) {
        let register = if self.register_read_any_register {
            None
        } else {
            match self.register_read_register {
                Some(reg) => Some(reg),
                None => return, // No register selected, don't add breakpoint
            }
        };

        let value = if self.register_read_any_value {
            None
        } else {
            match usize::from_str_radix(self.register_read_value_input.trim_start_matches("0x"), 16)
            {
                Ok(val) => Some((val & 0xFF) as u8),
                Err(_) => return, // Invalid input, don't add breakpoint
            }
        };

        let breakpoint = AnyBreakpoint::fdc_register_breakpoint(register, value, true, false);
        debugger.breakpoint_manager().add_breakpoint(breakpoint);

        self.register_read_register = None;
        self.register_read_any_register = false;
        self.register_read_value_input.clear();
        self.register_read_any_value = false;
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

        let breakpoint = AnyBreakpoint::fdc_register_breakpoint(register, value, false, true);
        debugger.breakpoint_manager().add_breakpoint(breakpoint);

        self.register_write_register = None;
        self.register_write_any_register = false;
        self.register_write_value_input.clear();
        self.register_write_any_value = false;
    }

    fn add_phase_breakpoint(&mut self, debugger: &mut impl Debugger) {
        if !self.phase_on_enter && !self.phase_on_leave {
            return;
        }

        let breakpoint = AnyBreakpoint::fdc_phase_breakpoint(
            self.phase,
            self.phase_on_enter,
            self.phase_on_leave,
        );
        debugger.breakpoint_manager().add_breakpoint(breakpoint);

        self.phase = None;
        self.phase_on_enter = false;
        self.phase_on_leave = false;
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
        let mut window = FdcDebugWindow {
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
        let mut window = FdcDebugWindow {
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
        let mut window = FdcDebugWindow {
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
        let mut window = FdcDebugWindow {
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
        let mut window = FdcDebugWindow {
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
        let mut window = FdcDebugWindow {
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
        let mut window = FdcDebugWindow {
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
        let mut window = FdcDebugWindow {
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
        let mut window = FdcDebugWindow {
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
        let mut window = FdcDebugWindow {
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
        let mut window = FdcDebugWindow {
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
        let mut window = FdcDebugWindow {
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
        let mut window = FdcDebugWindow {
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
    fn test_crtc_debug_window_hsync_breakpoint_invalid() {
        let mut debugger = TestDebugger::default();
        let mut window = FdcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ctx: &egui::Context| {
            window.ui(ctx, &mut debugger);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Add breakpoint
        harness
            .get_all_by_role_and_label(accesskit::Role::Button, "Add")
            .nth(3)
            .unwrap()
            .click();
        harness.run();
        drop(harness);

        assert_eq!(debugger.breakpoint_manager().breakpoints_iter().count(), 0);
    }

    #[test]
    fn test_crtc_debug_window_vsync_breakpoint() {
        let mut debugger = TestDebugger::default();
        let mut window = FdcDebugWindow {
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
    fn test_crtc_debug_window_vsync_breakpoint_invalid() {
        let mut debugger = TestDebugger::default();
        let mut window = FdcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ctx: &egui::Context| {
            window.ui(ctx, &mut debugger);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Add breakpoint
        harness
            .get_all_by_role_and_label(accesskit::Role::Button, "Add")
            .nth(4)
            .unwrap()
            .click();
        harness.run();
        drop(harness);

        assert_eq!(debugger.breakpoint_manager().breakpoints_iter().count(), 0);
    }

    #[test]
    fn test_crtc_debug_window_display_enable_breakpoint() {
        let mut debugger = TestDebugger::default();
        let mut window = FdcDebugWindow {
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

    #[test]
    fn test_crtc_debug_window_display_enable_breakpoint_invalid() {
        let mut debugger = TestDebugger::default();
        let mut window = FdcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ctx: &egui::Context| {
            window.ui(ctx, &mut debugger);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Add breakpoint
        harness
            .get_all_by_role_and_label(accesskit::Role::Button, "Add")
            .nth(5)
            .unwrap()
            .click();
        harness.run();
        drop(harness);

        assert_eq!(debugger.breakpoint_manager().breakpoints_iter().count(), 0);
    }
}
