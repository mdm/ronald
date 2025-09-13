use eframe::egui;
use serde::{Deserialize, Serialize};

use ronald_core::debug::{
    breakpoint::{AnyBreakpoint, Breakpoint, BreakpointManager},
    view::CpuDebugView,
};

#[derive(Deserialize, Serialize)]
pub struct CpuDebugWindow {
    pub show: bool,
    pc_breakpoint_input: String,
}

impl Default for CpuDebugWindow {
    fn default() -> Self {
        Self {
            show: false,
            pc_breakpoint_input: String::new(),
        }
    }
}

impl CpuDebugWindow {
    pub fn ui(
        &mut self,
        ctx: &egui::Context,
        data: Option<&CpuDebugView>,
        breakpoint_manager: Option<&mut BreakpointManager>,
    ) {
        if !self.show {
            return;
        }

        let mut open = self.show;
        egui::Window::new("CPU Internals")
            .open(&mut open)
            .default_size([400.0, 600.0])
            .resizable(false)
            .show(ctx, |ui| {
                if let Some(data) = data {
                    self.render_cpu_registers(ui, data);

                    if let Some(bp_manager) = breakpoint_manager {
                        ui.separator();
                        self.render_breakpoints_section(ui, data, bp_manager);
                    }
                } else {
                    ui.label("No data available - emulator must be paused");
                }
            });
        self.show = open;
    }

    fn render_cpu_registers(&self, ui: &mut egui::Ui, data: &CpuDebugView) {
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

    fn render_breakpoints_section(
        &mut self,
        ui: &mut egui::Ui,
        _data: &CpuDebugView,
        breakpoint_manager: &mut BreakpointManager,
    ) {
        ui.heading("Breakpoints");

        // PC breakpoint input
        ui.horizontal(|ui| {
            ui.label("PC:");
            let text_edit = ui.text_edit_singleline(&mut self.pc_breakpoint_input);
            let enter_pressed =
                text_edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            let add_clicked = ui.button("Add").clicked();

            if enter_pressed || add_clicked {
                if let Ok(addr) =
                    u16::from_str_radix(self.pc_breakpoint_input.trim_start_matches("0x"), 16)
                {
                    breakpoint_manager.add_breakpoint(AnyBreakpoint::pc_breakpoint(addr));
                    self.pc_breakpoint_input.clear();
                }
            }
        });

        // List active CPU breakpoints only
        ui.separator();
        ui.label("Active CPU Breakpoints:");

        let mut cpu_breakpoint_found = false;
        let mut to_remove = None;
        let mut to_toggle = None;

        breakpoint_manager.with_breakpoints(|(id, breakpoint)| {
            if !matches!(
                breakpoint,
                AnyBreakpoint::CpuRegister8(_) | AnyBreakpoint::CpuRegister16(_)
            ) {
                return;
            }

            cpu_breakpoint_found = true;

            ui.horizontal(|ui| {
                // Show red dot if breakpoint was triggered
                if breakpoint.triggered() {
                    ui.colored_label(egui::Color32::RED, "●");
                }

                // Toggle enabled/disabled
                let mut enabled = breakpoint.enabled();
                if ui.checkbox(&mut enabled, "").changed() {
                    to_toggle = Some((id, enabled));
                }

                // Breakpoint description
                ui.label(breakpoint.to_string());

                // Remove button
                if ui.button("Remove").clicked() {
                    to_remove = Some(id);
                }
            });
        });

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
}
