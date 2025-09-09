use eframe::egui;
use ronald_core::debug::view::CpuDebugView;

#[derive(Default)]
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
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.monospace(format!("A: {:02X}", data.register_a));
                ui.monospace(format!("B: {:02X}", data.register_b));
                ui.monospace(format!("D: {:02X}", data.register_d));
                ui.monospace(format!("H: {:02X}", data.register_h));
            });
            ui.vertical(|ui| {
                ui.monospace(format!("F: {:02X}", data.register_f));
                ui.monospace(format!("C: {:02X}", data.register_c));
                ui.monospace(format!("E: {:02X}", data.register_e));
                ui.monospace(format!("L: {:02X}", data.register_l));
            });
        });

        ui.separator();
        ui.heading("Shadow Registers");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.monospace(format!("A': {:02X}", data.shadow_register_a));
                ui.monospace(format!("B': {:02X}", data.shadow_register_b));
                ui.monospace(format!("D': {:02X}", data.shadow_register_d));
                ui.monospace(format!("H': {:02X}", data.shadow_register_h));
            });
            ui.vertical(|ui| {
                ui.monospace(format!("F': {:02X}", data.shadow_register_f));
                ui.monospace(format!("C': {:02X}", data.shadow_register_c));
                ui.monospace(format!("E': {:02X}", data.shadow_register_e));
                ui.monospace(format!("L': {:02X}", data.shadow_register_l));
            });
        });

        ui.separator();
        ui.heading("Index & Special Registers");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.monospace(format!(
                    "IX: {:02X}{:02X}",
                    data.register_ixh, data.register_ixl
                ));
                ui.monospace(format!(
                    "IY: {:02X}{:02X}",
                    data.register_iyh, data.register_iyl
                ));
                ui.monospace(format!("SP: {:04X}", data.register_sp));
                ui.monospace(format!("PC: {:04X}", data.register_pc));
            });
            ui.vertical(|ui| {
                ui.monospace(format!("I: {:02X}", data.register_i));
                ui.monospace(format!("R: {:02X}", data.register_r));
                ui.monospace(format!("IM: {}", data.interrupt_mode));
                ui.monospace("");
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
