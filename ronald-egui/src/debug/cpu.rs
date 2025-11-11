use eframe::egui;
use serde::{Deserialize, Serialize};

use ronald_core::debug::breakpoint::{AnyBreakpoint, Breakpoint};
use ronald_core::system::cpu::{Register8, Register16 as PrimaryRegister16};

use crate::colors;
use crate::debug::Debugger;

fn default_register8() -> Register8 {
    Register8::A
}

fn default_ui_register16() -> Register16 {
    Register16::PC
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
enum Register16 {
    PC,
    SP,
    AF,
    BC,
    DE,
    HL,
    IX,
    IY,
    ShadowAF,
    ShadowBC,
    ShadowDE,
    ShadowHL,
}

impl Register16 {
    fn is_shadow(&self) -> bool {
        matches!(
            self,
            Register16::ShadowAF
                | Register16::ShadowBC
                | Register16::ShadowDE
                | Register16::ShadowHL
        )
    }

    fn all_options() -> &'static [Register16] {
        &[
            Register16::PC,
            Register16::SP,
            Register16::AF,
            Register16::BC,
            Register16::DE,
            Register16::HL,
            Register16::IX,
            Register16::IY,
            Register16::ShadowAF,
            Register16::ShadowBC,
            Register16::ShadowDE,
            Register16::ShadowHL,
        ]
    }
}

impl std::fmt::Display for Register16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register16::PC => write!(f, "PC"),
            Register16::SP => write!(f, "SP"),
            Register16::AF => write!(f, "AF"),
            Register16::BC => write!(f, "BC"),
            Register16::DE => write!(f, "DE"),
            Register16::HL => write!(f, "HL"),
            Register16::IX => write!(f, "IX"),
            Register16::IY => write!(f, "IY"),
            Register16::ShadowAF => write!(f, "AF'"),
            Register16::ShadowBC => write!(f, "BC'"),
            Register16::ShadowDE => write!(f, "DE'"),
            Register16::ShadowHL => write!(f, "HL'"),
        }
    }
}

impl From<Register16> for PrimaryRegister16 {
    fn from(reg: Register16) -> Self {
        match reg {
            Register16::PC => PrimaryRegister16::PC,
            Register16::SP => PrimaryRegister16::SP,
            Register16::AF | Register16::ShadowAF => PrimaryRegister16::AF,
            Register16::BC | Register16::ShadowBC => PrimaryRegister16::BC,
            Register16::DE | Register16::ShadowDE => PrimaryRegister16::DE,
            Register16::HL | Register16::ShadowHL => PrimaryRegister16::HL,
            Register16::IX => PrimaryRegister16::IX,
            Register16::IY => PrimaryRegister16::IY,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CpuDebugWindow {
    pub show: bool,

    // 8-bit register breakpoints
    #[serde(skip, default = "default_register8")]
    selected_register8: Register8,
    #[serde(skip, default)]
    register8_value_input: String,
    #[serde(skip, default)]
    register8_any_value: bool,

    // 16-bit register breakpoints (main + shadow)
    #[serde(skip, default = "default_ui_register16")]
    selected_register16: Register16,
    #[serde(skip, default)]
    register16_value_input: String,
    #[serde(skip, default)]
    register16_any_value: bool,
}

impl Default for CpuDebugWindow {
    fn default() -> Self {
        Self {
            show: false,
            selected_register8: Register8::A,
            register8_value_input: String::new(),
            register8_any_value: false,
            selected_register16: Register16::PC,
            register16_value_input: String::new(),
            register16_any_value: false,
        }
    }
}

impl CpuDebugWindow {
    pub fn ui(&mut self, ctx: &egui::Context, debugger: &mut impl Debugger) {
        if !self.show {
            return;
        }

        let mut open = self.show;
        egui::Window::new("CPU Internals")
            .resizable(false)
            .open(&mut open)
            .show(ctx, |ui| {
                self.render_cpu_registers(ui, debugger);
                ui.separator();
                self.render_breakpoints_section(ui, debugger);
            });
        self.show = open;
    }

    fn render_cpu_registers(&self, ui: &mut egui::Ui, debugger: &mut impl Debugger) {
        let data = &debugger.debug_view().cpu;
        ui.heading("Main Registers");
        egui::Grid::new("cpu_main_registers_grid")
            .num_columns(8)
            .show(ui, |ui| {
                ui.label("A:");
                ui.monospace(format!("{:02X}", data.register_a));
                ui.label("F:");
                ui.monospace(format!("{:02X}", data.register_f));
                ui.label("B:");
                ui.monospace(format!("{:02X}", data.register_b));
                ui.label("C:");
                ui.monospace(format!("{:02X}", data.register_c));
                ui.end_row();

                ui.label("D:");
                ui.monospace(format!("{:02X}", data.register_d));
                ui.label("E:");
                ui.monospace(format!("{:02X}", data.register_e));
                ui.label("H:");
                ui.monospace(format!("{:02X}", data.register_h));
                ui.label("L:");
                ui.monospace(format!("{:02X}", data.register_l));
                ui.end_row();
            });

        ui.heading("Shadow Registers");
        egui::Grid::new("cpu_shadow_registers_grid")
            .num_columns(8)
            .show(ui, |ui| {
                ui.label("A':");
                ui.monospace(format!("{:02X}", data.shadow_register_a));
                ui.label("F':");
                ui.monospace(format!("{:02X}", data.shadow_register_f));
                ui.label("B':");
                ui.monospace(format!("{:02X}", data.shadow_register_b));
                ui.label("C':");
                ui.monospace(format!("{:02X}", data.shadow_register_c));
                ui.end_row();

                ui.label("D':");
                ui.monospace(format!("{:02X}", data.shadow_register_d));
                ui.label("E':");
                ui.monospace(format!("{:02X}", data.shadow_register_e));
                ui.label("H':");
                ui.monospace(format!("{:02X}", data.shadow_register_h));
                ui.label("L':");
                ui.monospace(format!("{:02X}", data.shadow_register_l));
                ui.end_row();
            });

        ui.heading("Index & Special Registers");
        egui::Grid::new("cpu_special_registers_grid")
            .num_columns(4)
            .show(ui, |ui| {
                ui.label("IX:");
                ui.monospace(format!(
                    "{:02X}{:02X}",
                    data.register_ixh, data.register_ixl
                ));
                ui.label("IY:");
                ui.monospace(format!(
                    "{:02X}{:02X}",
                    data.register_iyh, data.register_iyl
                ));
                ui.end_row();

                ui.label("SP:");
                ui.monospace(format!("{:04X}", data.register_sp));
                ui.label("PC:");
                ui.monospace(format!("{:04X}", data.register_pc));
                ui.end_row();

                ui.label("I:");
                ui.monospace(format!("{:02X}", data.register_i));
                ui.label("R:");
                ui.monospace(format!("{:02X}", data.register_r));
                ui.end_row();

                ui.label("IM:");
                ui.monospace(format!("{}", data.interrupt_mode));
                ui.end_row();
            });

        ui.heading("Flags (F Register)");
        self.render_flags(ui, data.register_f);

        ui.heading("Status");
        ui.horizontal(|ui| {
            ui.label("IFF1:");
            ui.colored_label(
                if data.iff1 {
                    colors::FORREST_GREEN
                } else {
                    colors::DARK_RED
                },
                if data.iff1 { "ON" } else { "OFF" },
            );
            ui.label("IFF2:");
            ui.colored_label(
                if data.iff2 {
                    colors::FORREST_GREEN
                } else {
                    colors::DARK_RED
                },
                if data.iff2 { "ON" } else { "OFF" },
            );
            ui.label("Halted:");
            ui.colored_label(
                if data.halted {
                    colors::DARK_YELLOW_GOLD
                } else {
                    colors::FORREST_GREEN
                },
                if data.halted { "YES" } else { "NO" },
            );
        });
    }

    fn render_flags(&self, ui: &mut egui::Ui, flags: u8) {
        ui.horizontal(|ui| {
            for (bit, name) in [
                (7, "S"),
                (6, "Z"),
                (5, "Y"),
                (4, "H"),
                (3, "X"),
                (2, "P/V"),
                (1, "N"),
                (0, "C"),
            ] {
                let is_set = (flags >> bit) & 1 != 0;
                ui.colored_label(
                    if is_set {
                        colors::FORREST_GREEN
                    } else {
                        colors::MEDIUM_GRAY
                    },
                    format!("{}: {}", name, if is_set { "1" } else { "0" }),
                );
            }
        });
    }

    fn render_breakpoints_section(&mut self, ui: &mut egui::Ui, debugger: &mut impl Debugger) {
        ui.heading("CPU Breakpoints");

        egui::Grid::new("breakpoint_grid")
            .num_columns(2)
            .spacing([10.0, 4.0])
            .show(ui, |ui| {
                // 8-bit register breakpoints
                let label = ui.label("8-bit register:");

                ui.horizontal(|ui| {
                    egui::ComboBox::from_id_salt("reg8")
                        .selected_text(format!("{:?}", self.selected_register8))
                        .width(50.0)
                        .show_ui(ui, |ui| {
                            for reg in [
                                Register8::A,
                                Register8::F,
                                Register8::B,
                                Register8::C,
                                Register8::D,
                                Register8::E,
                                Register8::H,
                                Register8::L,
                                Register8::I,
                                Register8::R,
                                Register8::IXH,
                                Register8::IXL,
                                Register8::IYH,
                                Register8::IYL,
                            ] {
                                ui.selectable_value(
                                    &mut self.selected_register8,
                                    reg,
                                    format!("{:?}", reg),
                                );
                            }
                        })
                        .response
                        .labelled_by(label.id);

                    let label = ui.label("Value:");
                    ui.add_enabled(
                        !self.register8_any_value,
                        egui::TextEdit::singleline(&mut self.register8_value_input)
                            .desired_width(60.0),
                    )
                    .labelled_by(label.id)
                    .on_hover_text("Hex value (e.g., 42 or 0x42)");

                    if ui.checkbox(&mut self.register8_any_value, "Any").changed()
                        && self.register8_any_value
                    {
                        self.register8_value_input.clear();
                    }

                    if ui.button("Add").clicked() {
                        self.add_register8_breakpoint(debugger);
                    }
                });
                ui.end_row();

                // 16-bit register breakpoints
                ui.label("16-bit register:");

                ui.horizontal(|ui| {
                    egui::ComboBox::from_id_salt("reg16")
                        .selected_text(self.selected_register16.to_string())
                        .width(50.0)
                        .show_ui(ui, |ui| {
                            for reg in Register16::all_options() {
                                ui.selectable_value(
                                    &mut self.selected_register16,
                                    *reg,
                                    reg.to_string(),
                                );
                            }
                        });

                    ui.label("Value:");
                    ui.add_enabled(
                        !self.register16_any_value,
                        egui::TextEdit::singleline(&mut self.register16_value_input)
                            .desired_width(60.0),
                    )
                    .on_hover_text("Hex value (e.g., 1000 or 0x1000)");

                    if ui.checkbox(&mut self.register16_any_value, "Any").changed()
                        && self.register16_any_value
                    {
                        self.register16_value_input.clear();
                    }

                    if ui.button("Add").clicked() {
                        self.add_register16_breakpoint(debugger);
                    }
                });
                ui.end_row();
            });

        // List active CPU breakpoints only
        ui.separator();
        ui.label("Active CPU Breakpoints:");

        let mut breakpoint_found = false;
        let mut to_remove = None;
        let mut to_toggle = None;

        let breakpoint_manager = debugger.breakpoint_manager();
        for (id, breakpoint) in breakpoint_manager.breakpoints_iter() {
            if breakpoint.one_shot()
                || !matches!(
                    breakpoint,
                    AnyBreakpoint::CpuRegister8(_)
                        | AnyBreakpoint::CpuRegister16(_)
                        | AnyBreakpoint::CpuShadowRegister16(_)
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
            ui.label("No CPU breakpoints set");
        }

        // Apply changes (done outside the loop to avoid borrow issues)
        if let Some((id, enabled)) = to_toggle {
            breakpoint_manager.enable_breakpoint(id, enabled);
        }
        if let Some(id) = to_remove {
            breakpoint_manager.remove_breakpoint(id);
        }
    }

    fn add_register8_breakpoint(&mut self, debugger: &mut impl Debugger) {
        dbg!(&self.selected_register8, &self.register8_value_input);
        let value = if self.register8_any_value {
            None
        } else {
            match usize::from_str_radix(self.register8_value_input.trim_start_matches("0x"), 16) {
                Ok(val) => Some((val & 0xFF) as u8),
                Err(_) => return, // Invalid input, don't add breakpoint
            }
        };

        let breakpoint = AnyBreakpoint::cpu_register8_breakpoint(self.selected_register8, value);
        debugger.breakpoint_manager().add_breakpoint(breakpoint);
        self.register8_value_input.clear();
        self.register8_any_value = false;
    }

    fn add_register16_breakpoint(&mut self, debugger: &mut impl Debugger) {
        let value = if self.register16_any_value {
            None
        } else {
            match usize::from_str_radix(self.register16_value_input.trim_start_matches("0x"), 16) {
                Ok(val) => Some((val & 0xFFFF) as u16),
                Err(_) => return, // Invalid input, don't add breakpoint
            }
        };

        let primary_register = self.selected_register16.into();
        let breakpoint = if self.selected_register16.is_shadow() {
            AnyBreakpoint::cpu_shadow_register16_breakpoint(primary_register, value)
        } else {
            AnyBreakpoint::cpu_register16_breakpoint(primary_register, value)
        };

        debugger.breakpoint_manager().add_breakpoint(breakpoint);
        self.register16_value_input.clear();
        self.register16_any_value = false;
    }
}

#[cfg(test)]
mod gui_tests {
    use super::*;

    use egui::accesskit;
    use egui_kittest::{Harness, kittest::Queryable};

    use ronald_core::debug::breakpoint::CpuRegister8Breakpoint;

    use crate::debug::mock::TestDebugger;

    #[test]
    fn test_cpu_debug_window_opens_and_closes() {
        let mut debugger = TestDebugger::default();
        let mut window = CpuDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ctx: &egui::Context| {
            window.ui(ctx, &mut debugger);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Check that the window title is rendered
        harness.get_by_label("CPU Internals");

        // Click close button
        harness.get_by_label("Close window").click();
        harness.run();

        // Window should no longer be visible
        assert!(harness.query_by_label("CPU Internals").is_none());
    }

    #[test]
    fn test_cpu_debug_window_8bit_register_breakpoint_with_value() {
        let mut debugger = TestDebugger::default();
        let mut window = CpuDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ctx: &egui::Context| {
            window.ui(ctx, &mut debugger);
        };

        let mut harness = Harness::new(app);
        harness.run();

        let i = 0;

        // Select register B
        harness
            .get_all_by_role(accesskit::Role::ComboBox)
            .nth(i)
            .unwrap()
            .click();
        harness.run();
        harness.get_by_label("B").click();
        harness.run();

        // Enter value 0x42
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
        drop(harness);

        assert_eq!(debugger.breakpoint_manager().breakpoints_iter().count(), 1);
        assert!(
            debugger
                .breakpoint_manager()
                .breakpoints_iter()
                .any(|(_, bp)| {
                    matches!(
                        bp,
                        AnyBreakpoint::CpuRegister8(CpuRegister8Breakpoint {
                            register: Register8::B,
                            value: Some(0x42),
                            ..
                        })
                    )
                })
        );
    }
}
