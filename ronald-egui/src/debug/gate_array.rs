use eframe::egui;
use serde::{Deserialize, Serialize};

use ronald_core::constants::{FIRMWARE_COLORS, HARDWARE_TO_FIRMWARE_COLORS};
use ronald_core::debug::breakpoint::{AnyBreakpoint, Breakpoint};

use crate::colors;
use crate::frontend::Frontend;

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GateArrayDebugWindow {
    pub open: bool,

    // Screen mode breakpoint
    #[serde(skip, default)]
    screen_mode_value: Option<u8>,
    #[serde(skip, default)]
    screen_mode_any_change: bool,
    #[serde(skip, default)]
    screen_mode_applied: bool,

    // Pen color breakpoint
    #[serde(skip, default)]
    pen_number: Option<usize>,
    #[serde(skip, default)]
    pen_color: Option<usize>,
    #[serde(skip, default)]
    pen_any_number: bool,
    #[serde(skip, default)]
    pen_any_color: bool,
}

impl GateArrayDebugWindow {
    fn get_all_hardware_colors() -> Vec<(usize, egui::Color32, String)> {
        let mut colors = Vec::new();

        for hardware_index in 0..32 {
            let firmware_color_index = HARDWARE_TO_FIRMWARE_COLORS[hardware_index];
            let rgba = FIRMWARE_COLORS[firmware_color_index];
            let egui_color =
                egui::Color32::from_rgba_premultiplied(rgba[0], rgba[1], rgba[2], rgba[3]);

            let color_name = match firmware_color_index {
                0 => "Black",
                1 => "Blue",
                2 => "Bright Blue",
                3 => "Red",
                4 => "Magenta",
                5 => "Mauve",
                6 => "Bright Red",
                7 => "Purple",
                8 => "Bright Magenta",
                9 => "Green",
                10 => "Cyan",
                11 => "Sky Blue",
                12 => "Yellow",
                13 => "White", // Actually grey in some contexts
                14 => "Pastel Blue",
                15 => "Orange",
                16 => "Pink",
                17 => "Pastel Magenta",
                18 => "Bright Green",
                19 => "Sea Green",
                20 => "Bright Cyan",
                21 => "Lime",
                22 => "Pastel Green",
                23 => "Pastel Cyan",
                24 => "Bright Yellow",
                25 => "Pastel Yellow",
                26 => "Bright White",
                _ => "Unknown",
            };

            let display_name = format!("{} (0x{:02X})", color_name, hardware_index);
            colors.push((hardware_index, egui_color, display_name));
        }

        colors
    }

    pub fn ui(&mut self, ctx: &egui::Context, frontend: &mut Frontend) {
        let mut open = self.open;
        egui::Window::new("Gate Array Internals")
            .resizable(false)
            .open(&mut open)
            .show(ctx, |ui| {
                self.render_gate_array_state(ui, frontend);
                ui.separator();
                self.render_breakpoints_section(ui, frontend);
            });
        self.open = open;
    }

    fn render_gate_array_state(&self, ui: &mut egui::Ui, frontend: &mut Frontend) {
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
                colors::FORREST_GREEN
            } else {
                colors::MEDIUM_GRAY
            };
            ui.colored_label(hsync_color, "HSYNC");

            ui.separator();

            let vsync_color = if ga.vsync_active {
                colors::FORREST_GREEN
            } else {
                colors::MEDIUM_GRAY
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
                        let (rect, _) =
                            ui.allocate_exact_size(egui::vec2(40.0, 30.0), egui::Sense::hover());
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

            let (rect, _) = ui.allocate_exact_size(egui::vec2(40.0, 30.0), egui::Sense::hover());
            ui.painter().rect_filled(rect, 2.0, egui_color);

            ui.label(format!("{:#04x}", ga.pen_colors[0x10]));
        });
    }

    fn render_breakpoints_section(&mut self, ui: &mut egui::Ui, frontend: &mut Frontend) {
        ui.heading("Gate Array Breakpoints");

        egui::Grid::new("ga_breakpoint_grid")
            .num_columns(2)
            .spacing([10.0, 4.0])
            .show(ui, |ui| {
                // Screen mode breakpoint
                ui.label("Screen mode:");

                ui.horizontal(|ui| {
                    ui.add_enabled_ui(!self.screen_mode_any_change, |ui| {
                        egui::ComboBox::from_id_salt("screen_mode_selector")
                            .width(60.0)
                            .selected_text(match self.screen_mode_value {
                                Some(mode) => format!("Mode {}", mode),
                                None => "Select...".to_string(),
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.screen_mode_value, Some(0), "Mode 0");
                                ui.selectable_value(&mut self.screen_mode_value, Some(1), "Mode 1");
                                ui.selectable_value(&mut self.screen_mode_value, Some(2), "Mode 2");
                                ui.selectable_value(&mut self.screen_mode_value, Some(3), "Mode 3");
                            });
                    });

                    if ui
                        .checkbox(&mut self.screen_mode_any_change, "Any")
                        .changed()
                        && self.screen_mode_any_change
                    {
                        self.screen_mode_value = None;
                    }

                    ui.checkbox(&mut self.screen_mode_applied, "Applied")
                        .on_hover_text("Break when mode is applied (at next HSYNC) vs when requested (written to register)");

                    if ui.button("Add").clicked() {
                        self.add_screen_mode_breakpoint(frontend);
                    }
                });
                ui.end_row();

                // Pen color breakpoint
                ui.label("Pen:");

                ui.horizontal(|ui| {
                    ui.add_enabled_ui(!self.pen_any_number, |ui| {
                        egui::ComboBox::from_id_salt("pen_number_selector")
                            .width(80.0)
                            .selected_text(match self.pen_number {
                                Some(16) => "Border".to_string(),
                                Some(pen) => format!("Pen {}", pen),
                                None => "Select...".to_string(),
                            })
                            .show_ui(ui, |ui| {
                                for pen in 0..16 {
                                    ui.selectable_value(&mut self.pen_number, Some(pen), format!("Pen {}", pen));
                                }
                                ui.selectable_value(&mut self.pen_number, Some(16), "Border");
                            });
                    });

                    if ui.checkbox(&mut self.pen_any_number, "Any").changed() && self.pen_any_number {
                        self.pen_number = None;
                    }

                    ui.label("Color:");

                    let colors = Self::get_all_hardware_colors();
                    let (selected_color, selected_text) = match self.pen_color {
                        Some(selected) =>  {
                            let (_, color, name) = &colors[selected];
                            (*color, name.as_str())
                        }
                        None => (colors::BLACK, "Select..."),
                    };


                    ui.add_enabled_ui(!self.pen_any_color, |ui| {
                        ui.horizontal(|ui| {
                            // Show swatch for currently selected color
                            let (swatch_rect, _) = ui.allocate_exact_size(egui::vec2(20.0, 15.0), egui::Sense::hover());
                            ui.painter().rect_filled(swatch_rect, 2.0, selected_color);

                            // ComboBox with text only, disabled when pen_any_color is true
                            egui::ComboBox::from_id_salt("pen_color_selector")
                                .width(150.0)
                                .selected_text(selected_text)
                                .show_ui(ui, |ui| {
                                    for (hardware_index, color, name) in colors.iter() {
                                        ui.horizontal(|ui| {
                                            // Color swatch
                                            let (swatch_rect, _) = ui.allocate_exact_size(egui::vec2(20.0, 15.0), egui::Sense::hover());
                                            ui.painter().rect_filled(swatch_rect, 2.0, *color);

                                            // Color name with hardware value
                                            ui.selectable_value(&mut self.pen_color, Some(*hardware_index), name);
                                        });
                                    }
                                })
                        })
                    });

                    if ui.checkbox(&mut self.pen_any_color, "Any").changed() && self.pen_any_color {
                        self.pen_color = None
                    }

                    if ui.button("Add").clicked() {
                        self.add_pen_color_breakpoint(frontend);
                    }
                });
                ui.end_row();

                // Interrupt breakpoint
                ui.label("Interrupt:");

                ui.horizontal(|ui| {
                    if ui.button("Add Interrupt Breakpoint").clicked() {
                        self.add_interrupt_breakpoint(frontend);
                    }
                });
                ui.end_row();
            });

        // List active Gate Array breakpoints
        ui.separator();
        ui.label("Active Gate Array Breakpoints:");

        let mut breakpoint_found = false;
        let mut to_remove = None;
        let mut to_toggle = None;

        let breakpoint_manager = frontend.breakpoint_manager();
        for (id, breakpoint) in breakpoint_manager.breakpoints_iter() {
            if breakpoint.one_shot()
                || !matches!(
                    breakpoint,
                    AnyBreakpoint::GateArrayScreenMode(_)
                        | AnyBreakpoint::GateArrayPenColor(_)
                        | AnyBreakpoint::GateArrayInterrupt(_)
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
            ui.label("No Gate Array breakpoints set");
        }

        // Apply changes
        if let Some((id, enabled)) = to_toggle {
            breakpoint_manager.enable_breakpoint(id, enabled);
        }
        if let Some(id) = to_remove {
            breakpoint_manager.remove_breakpoint(id);
        }
    }

    fn add_screen_mode_breakpoint(&mut self, frontend: &mut Frontend) {
        let mode = if self.screen_mode_any_change {
            None
        } else {
            self.screen_mode_value
        };

        let breakpoint =
            AnyBreakpoint::gate_array_screen_mode_breakpoint(mode, self.screen_mode_applied);
        frontend.breakpoint_manager().add_breakpoint(breakpoint);
        self.screen_mode_value = None;
        self.screen_mode_any_change = false;
        self.screen_mode_applied = false;
    }

    fn add_pen_color_breakpoint(&mut self, frontend: &mut Frontend) {
        let pen = if self.pen_any_number {
            None
        } else {
            self.pen_number
        };
        let color = self.pen_color.map(|c| c as u8);

        let breakpoint = AnyBreakpoint::gate_array_pen_color_breakpoint(pen, color);
        frontend.breakpoint_manager().add_breakpoint(breakpoint);
        self.pen_number = None;
        self.pen_color = None;
        self.pen_any_number = false;
        self.pen_any_color = false;
    }

    fn add_interrupt_breakpoint(&mut self, frontend: &mut Frontend) {
        let breakpoint = AnyBreakpoint::gate_array_interrupt_breakpoint();
        frontend.breakpoint_manager().add_breakpoint(breakpoint);
    }
}
