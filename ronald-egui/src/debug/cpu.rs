use eframe::egui;
use serde::{Deserialize, Serialize};

use ronald_core::debug::breakpoint::{AnyBreakpoint, Breakpoint};
use ronald_core::system::cpu::{Register8, Register16 as PrimaryRegister16};

use crate::frontend::Frontend;

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
    register8_any_change: bool,

    // 16-bit register breakpoints (main + shadow)
    #[serde(skip, default = "default_ui_register16")]
    selected_register16: Register16,
    #[serde(skip, default)]
    register16_value_input: String,
    #[serde(skip, default)]
    register16_any_change: bool,
}

impl Default for CpuDebugWindow {
    fn default() -> Self {
        Self {
            show: false,
            selected_register8: Register8::A,
            register8_value_input: String::new(),
            register8_any_change: false,
            selected_register16: Register16::PC,
            register16_value_input: String::new(),
            register16_any_change: false,
        }
    }
}

impl CpuDebugWindow {
    pub fn ui(&mut self, ctx: &egui::Context, frontend: &mut Frontend) {
        if !self.show {
            return;
        }

        let mut open = self.show;
        egui::Window::new("CPU Internals")
            .open(&mut open)
            .default_size([400.0, 600.0])
            .resizable(false)
            .show(ctx, |ui| {
                self.render_cpu_registers(ui, frontend);
                ui.separator();
                self.render_breakpoints_section(ui, frontend);
            });
        self.show = open;
    }

    fn render_cpu_registers(&self, ui: &mut egui::Ui, frontend: &mut Frontend) {
        let data = &frontend.debug_view().cpu;
        ui.heading("Main Registers");
        egui_extras::TableBuilder::new(ui)
            .column(egui_extras::Column::exact(20.0))
            .column(egui_extras::Column::exact(20.0))
            .column(egui_extras::Column::exact(20.0))
            .column(egui_extras::Column::exact(20.0))
            .column(egui_extras::Column::exact(20.0))
            .column(egui_extras::Column::exact(20.0))
            .column(egui_extras::Column::exact(20.0))
            .column(egui_extras::Column::exact(20.0))
            .body(|mut body| {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label("A:");
                    });
                    row.col(|ui| {
                        ui.monospace(format!("{:02X}", data.register_a));
                    });
                    row.col(|ui| {
                        ui.label("F:");
                    });
                    row.col(|ui| {
                        ui.monospace(format!("{:02X}", data.register_f));
                    });
                    row.col(|ui| {
                        ui.label("B:");
                    });
                    row.col(|ui| {
                        ui.monospace(format!("{:02X}", data.register_b));
                    });
                    row.col(|ui| {
                        ui.label("C:");
                    });
                    row.col(|ui| {
                        ui.monospace(format!("{:02X}", data.register_c));
                    });
                });
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label("D:");
                    });
                    row.col(|ui| {
                        ui.monospace(format!("{:02X}", data.register_d));
                    });
                    row.col(|ui| {
                        ui.label("E:");
                    });
                    row.col(|ui| {
                        ui.monospace(format!("{:02X}", data.register_e));
                    });
                    row.col(|ui| {
                        ui.label("H:");
                    });
                    row.col(|ui| {
                        ui.monospace(format!("{:02X}", data.register_h));
                    });
                    row.col(|ui| {
                        ui.label("L:");
                    });
                    row.col(|ui| {
                        ui.monospace(format!("{:02X}", data.register_l));
                    });
                });
            });

        ui.separator();
        ui.heading("Shadow Registers");
        egui_extras::TableBuilder::new(ui)
            .column(egui_extras::Column::exact(20.0))
            .column(egui_extras::Column::exact(20.0))
            .column(egui_extras::Column::exact(20.0))
            .column(egui_extras::Column::exact(20.0))
            .column(egui_extras::Column::exact(20.0))
            .column(egui_extras::Column::exact(20.0))
            .column(egui_extras::Column::exact(20.0))
            .column(egui_extras::Column::exact(20.0))
            .body(|mut body| {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label("A':");
                    });
                    row.col(|ui| {
                        ui.monospace(format!("{:02X}", data.shadow_register_a));
                    });
                    row.col(|ui| {
                        ui.label("F':");
                    });
                    row.col(|ui| {
                        ui.monospace(format!("{:02X}", data.shadow_register_f));
                    });
                    row.col(|ui| {
                        ui.label("B':");
                    });
                    row.col(|ui| {
                        ui.monospace(format!("{:02X}", data.shadow_register_b));
                    });
                    row.col(|ui| {
                        ui.label("C':");
                    });
                    row.col(|ui| {
                        ui.monospace(format!("{:02X}", data.shadow_register_c));
                    });
                });
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label("D':");
                    });
                    row.col(|ui| {
                        ui.monospace(format!("{:02X}", data.shadow_register_d));
                    });
                    row.col(|ui| {
                        ui.label("E':");
                    });
                    row.col(|ui| {
                        ui.monospace(format!("{:02X}", data.shadow_register_e));
                    });
                    row.col(|ui| {
                        ui.label("H':");
                    });
                    row.col(|ui| {
                        ui.monospace(format!("{:02X}", data.shadow_register_h));
                    });
                    row.col(|ui| {
                        ui.label("L':");
                    });
                    row.col(|ui| {
                        ui.monospace(format!("{:02X}", data.shadow_register_l));
                    });
                });
            });

        ui.separator();
        ui.heading("Index & Special Registers");
        egui_extras::TableBuilder::new(ui)
            .column(egui_extras::Column::exact(20.0))
            .column(egui_extras::Column::exact(40.0))
            .column(egui_extras::Column::exact(20.0))
            .column(egui_extras::Column::exact(40.0))
            .body(|mut body| {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label("IX:");
                    });
                    row.col(|ui| {
                        ui.monospace(format!(
                            "{:02X}{:02X}",
                            data.register_ixh, data.register_ixl
                        ));
                    });
                    row.col(|ui| {
                        ui.label("IY:");
                    });
                    row.col(|ui| {
                        ui.monospace(format!(
                            "{:02X}{:02X}",
                            data.register_iyh, data.register_iyl
                        ));
                    });
                });
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label("SP:");
                    });
                    row.col(|ui| {
                        ui.monospace(format!("{:04X}", data.register_sp));
                    });
                    row.col(|ui| {
                        ui.label("PC:");
                    });
                    row.col(|ui| {
                        ui.monospace(format!("{:04X}", data.register_pc));
                    });
                });
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label("I:");
                    });
                    row.col(|ui| {
                        ui.monospace(format!("{:02X}", data.register_i));
                    });
                    row.col(|ui| {
                        ui.label("R:");
                    });
                    row.col(|ui| {
                        ui.monospace(format!("{:02X}", data.register_r));
                    });
                });
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label("IM:");
                    });
                    row.col(|ui| {
                        ui.monospace(format!("{}", data.interrupt_mode));
                    });
                    row.col(|_ui| {});
                    row.col(|_ui| {});
                });
            });

        ui.separator();
        ui.heading("Flags (F Register)");
        self.render_flags(ui, data.register_f);

        ui.separator();
        ui.heading("Status");
        ui.horizontal(|ui| {
            ui.label("IFF1:");
            ui.colored_label(
                if data.iff1 {
                    egui::Color32::from_rgb(0, 150, 0) // Forest green - better contrast
                } else {
                    egui::Color32::from_rgb(200, 50, 50) // Dark red - better contrast
                },
                if data.iff1 { "ON" } else { "OFF" },
            );
            ui.label("IFF2:");
            ui.colored_label(
                if data.iff2 {
                    egui::Color32::from_rgb(0, 150, 0) // Forest green - better contrast
                } else {
                    egui::Color32::from_rgb(200, 50, 50) // Dark red - better contrast
                },
                if data.iff2 { "ON" } else { "OFF" },
            );
            ui.label("Halted:");
            ui.colored_label(
                if data.halted {
                    egui::Color32::from_rgb(200, 150, 0) // Dark yellow/gold - better contrast
                } else {
                    egui::Color32::from_rgb(0, 150, 0) // Forest green - better contrast
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
                        egui::Color32::from_rgb(0, 150, 0) // Forest green - better contrast
                    } else {
                        egui::Color32::from_gray(120) // Darker gray - better contrast
                    },
                    format!("{}: {}", name, if is_set { "1" } else { "0" }),
                );
            }
        });
    }

    fn render_breakpoints_section(&mut self, ui: &mut egui::Ui, frontend: &mut Frontend) {
        ui.heading("CPU Breakpoints");

        egui::Grid::new("breakpoint_grid")
            .num_columns(2)
            .spacing([10.0, 4.0])
            .show(ui, |ui| {
                // 8-bit register breakpoints
                ui.label("8-bit register:");

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
                        });

                    ui.label("Value:");
                    let text_edit = ui
                        .add_enabled(
                            !self.register8_any_change,
                            egui::TextEdit::singleline(&mut self.register8_value_input)
                                .desired_width(60.0),
                        )
                        .on_hover_text("Hex value (e.g., 42 or 0x42)");

                    if ui
                        .checkbox(&mut self.register8_any_change, "Any Change")
                        .changed()
                        && self.register8_any_change
                    {
                        self.register8_value_input.clear();
                    }

                    let enter_pressed =
                        text_edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                    let add_clicked = ui.button("Add").clicked();

                    if enter_pressed || add_clicked {
                        self.add_register8_breakpoint(frontend);
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
                    let text_edit = ui
                        .add_enabled(
                            !self.register16_any_change,
                            egui::TextEdit::singleline(&mut self.register16_value_input)
                                .desired_width(60.0),
                        )
                        .on_hover_text("Hex value (e.g., 1000 or 0x1000)");

                    if ui
                        .checkbox(&mut self.register16_any_change, "Any Change")
                        .changed()
                        && self.register16_any_change
                    {
                        self.register16_value_input.clear();
                    }

                    let enter_pressed =
                        text_edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                    let add_clicked = ui.button("Add").clicked();

                    if enter_pressed || add_clicked {
                        self.add_register16_breakpoint(frontend);
                    }
                });
                ui.end_row();
            });

        // List active CPU breakpoints only
        ui.separator();
        ui.label("Active CPU Breakpoints:");

        let mut cpu_breakpoint_found = false;
        let mut to_remove = None;
        let mut to_toggle = None;

        let breakpoint_manager = frontend.breakpoint_manager();
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

            cpu_breakpoint_found = true;

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
                        egui::Color32::from_rgb(200, 50, 50), // Dark red - better contrast
                        format!("(triggered at {})", master_clock.value()),
                    );
                }
            });
        }

        if !cpu_breakpoint_found {
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

    fn add_register8_breakpoint(&mut self, frontend: &mut Frontend) {
        let value = if self.register8_any_change {
            None
        } else {
            match usize::from_str_radix(self.register8_value_input.trim_start_matches("0x"), 16) {
                Ok(val) => Some((val & 0xFF) as u8),
                Err(_) => return, // Invalid input, don't add breakpoint
            }
        };

        let breakpoint = AnyBreakpoint::cpu_register8_breakpoint(self.selected_register8, value);
        frontend.breakpoint_manager().add_breakpoint(breakpoint);
        self.clear_register8_inputs();
    }

    fn add_register16_breakpoint(&mut self, frontend: &mut Frontend) {
        let value = if self.register16_any_change {
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

        frontend.breakpoint_manager().add_breakpoint(breakpoint);
        self.clear_register16_inputs();
    }

    fn clear_register8_inputs(&mut self) {
        self.register8_value_input.clear();
        self.register8_any_change = false;
    }

    fn clear_register16_inputs(&mut self) {
        self.register16_value_input.clear();
        self.register16_any_change = false;
    }
}
