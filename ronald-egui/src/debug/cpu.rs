use eframe::egui;
use serde::{Deserialize, Serialize};

use ronald_core::debug::view::CpuDebugView;

#[derive(Default, Deserialize, Serialize)]
pub struct CpuDebugWindow {
    pub show: bool,
}

impl CpuDebugWindow {
    pub fn ui(&mut self, ctx: &egui::Context, data: Option<&CpuDebugView>) {
        if !self.show {
            return;
        }

        let mut open = self.show;
        egui::Window::new("CPU Debug")
            .open(&mut open)
            .default_size([400.0, 600.0])
            .show(ctx, |ui| {
                if let Some(data) = data {
                    self.render_cpu_registers(ui, data);
                } else {
                    ui.label("No debug data available - emulator must be paused");
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
                    egui::Color32::GREEN
                } else {
                    egui::Color32::RED
                },
                if data.iff1 { "ON" } else { "OFF" },
            );
            ui.label("IFF2:");
            ui.colored_label(
                if data.iff2 {
                    egui::Color32::GREEN
                } else {
                    egui::Color32::RED
                },
                if data.iff2 { "ON" } else { "OFF" },
            );
            ui.label("Halted:");
            ui.colored_label(
                if data.halted {
                    egui::Color32::YELLOW
                } else {
                    egui::Color32::GREEN
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
                        egui::Color32::GREEN
                    } else {
                        egui::Color32::GRAY
                    },
                    format!("{}: {}", name, if is_set { "1" } else { "0" }),
                );
            }
        });
    }
}
