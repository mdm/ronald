use eframe::egui;
use serde::{Deserialize, Serialize};

use ronald_core::constants::{FIRMWARE_COLORS, HARDWARE_TO_FIRMWARE_COLORS};

use crate::frontend::Frontend;

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct GateArrayDebugWindow {
    #[serde(skip)]
    pub open: bool,
}

impl Default for GateArrayDebugWindow {
    fn default() -> Self {
        Self { open: false }
    }
}

impl GateArrayDebugWindow {
    pub fn ui(&mut self, ctx: &egui::Context, frontend: &mut Frontend) {
        egui::Window::new("Gate Array Internals")
            .open(&mut self.open)
            .default_width(400.0)
            .show(ctx, |ui| {
                let debug_view = frontend.debug_view();
                let ga = &debug_view.gate_array;

                // Screen Mode Section
                ui.heading("Screen Mode");
                ui.horizontal(|ui| {
                    ui.label("Current:");
                    ui.label(format!("{}", ga.current_screen_mode));
                    ui.separator();
                    ui.label("Requested:");
                    let requested_text = match ga.requested_screen_mode {
                        Some(mode) => format!("{}", mode),
                        None => "-".to_string(),
                    };
                    ui.label(requested_text);
                });
                ui.add_space(8.0);

                // Sync Status Section
                ui.heading("Sync Status");
                ui.horizontal(|ui| {
                    let hsync_color = if ga.hsync_active {
                        egui::Color32::GREEN
                    } else {
                        egui::Color32::DARK_GRAY
                    };
                    ui.colored_label(hsync_color, "HSYNC");

                    ui.separator();

                    let vsync_color = if ga.vsync_active {
                        egui::Color32::GREEN
                    } else {
                        egui::Color32::DARK_GRAY
                    };
                    ui.colored_label(vsync_color, "VSYNC");

                    ui.separator();

                    ui.label("HSYNCs since VSYNC:");
                    ui.label(format!("{}", ga.hsyncs_since_last_vsync));
                });
                ui.add_space(8.0);

                // Interrupt System Section
                ui.heading("Interrupt System");
                ui.horizontal(|ui| {
                    ui.label("Counter:");
                    ui.label(format!("{}", ga.interrupt_counter));
                    ui.separator();
                    ui.label("Hold:");
                    let hold_text = if ga.hold_interrupt { "Yes" } else { "No" };
                    ui.label(hold_text);
                });
                ui.add_space(8.0);

                // Palette Section
                ui.heading("Palette");
                ui.label("Selected Pen:");
                ui.label(format!("{}", ga.selected_pen));
                ui.add_space(4.0);

                // Display pens 0-15 in a 4x4 grid
                egui::Grid::new("pen_colors_grid")
                    .num_columns(4)
                    .spacing([8.0, 8.0])
                    .show(ui, |ui| {
                        for pen in 0..16 {
                            ui.vertical(|ui| {
                                let color_index = ga.pen_colors[pen] as usize;
                                let firmware_color_index = HARDWARE_TO_FIRMWARE_COLORS[color_index];
                                let rgba = FIRMWARE_COLORS[firmware_color_index];
                                let egui_color = egui::Color32::from_rgba_premultiplied(
                                    rgba[0], rgba[1], rgba[2], rgba[3],
                                );

                                ui.label(format!("Pen {}", pen));
                                let (rect, _) = ui.allocate_exact_size(
                                    egui::vec2(40.0, 30.0),
                                    egui::Sense::hover(),
                                );
                                ui.painter().rect_filled(rect, 2.0, egui_color);

                                // Show hardware color value
                                ui.label(format!("{:#04x}", ga.pen_colors[pen]));
                            });

                            if (pen + 1) % 4 == 0 {
                                ui.end_row();
                            }
                        }
                    });

                ui.add_space(8.0);

                // Border Color
                ui.label("Border:");
                ui.horizontal(|ui| {
                    let color_index = ga.pen_colors[0x10] as usize;
                    let firmware_color_index = HARDWARE_TO_FIRMWARE_COLORS[color_index];
                    let rgba = FIRMWARE_COLORS[firmware_color_index];
                    let egui_color =
                        egui::Color32::from_rgba_premultiplied(rgba[0], rgba[1], rgba[2], rgba[3]);

                    let (rect, _) =
                        ui.allocate_exact_size(egui::vec2(40.0, 30.0), egui::Sense::hover());
                    ui.painter().rect_filled(rect, 2.0, egui_color);

                    ui.label(format!("{:#04x}", ga.pen_colors[0x10]));
                });
            });
    }
}
