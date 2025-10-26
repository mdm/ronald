use eframe::egui;
use serde::{Deserialize, Serialize};

use ronald_core::constants::{FIRMWARE_COLORS, HARDWARE_TO_FIRMWARE_COLORS};
use ronald_core::debug::breakpoint::{AnyBreakpoint, Breakpoint};

use crate::frontend::Frontend;

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GateArrayDebugWindow {
    pub open: bool,

    // Screen mode breakpoint
    #[serde(skip, default)]
    screen_mode_value_input: String,
    #[serde(skip, default)]
    screen_mode_any_change: bool,
    #[serde(skip, default)]
    screen_mode_applied: bool,

    // Pen color breakpoint
    #[serde(skip, default)]
    pen_number_input: String,
    #[serde(skip, default)]
    pen_color_value_input: String,
    #[serde(skip, default)]
    pen_any_number: bool,
    #[serde(skip, default)]
    pen_any_color: bool,
}

impl GateArrayDebugWindow {
    pub fn ui(&mut self, ctx: &egui::Context, frontend: &mut Frontend) {
        let mut open = self.open;
        egui::Window::new("Gate Array Internals")
            .open(&mut open)
            .default_width(400.0)
            .default_height(600.0)
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
                    ui.label("Mode:");
                    let text_edit = ui
                        .add_enabled(
                            !self.screen_mode_any_change,
                            egui::TextEdit::singleline(&mut self.screen_mode_value_input)
                                .desired_width(40.0),
                        )
                        .on_hover_text("Mode 0-3");

                    if ui
                        .checkbox(&mut self.screen_mode_any_change, "Any")
                        .changed()
                        && self.screen_mode_any_change
                    {
                        self.screen_mode_value_input.clear();
                    }

                    ui.checkbox(&mut self.screen_mode_applied, "Applied")
                        .on_hover_text("Break when mode is applied (at next HSYNC) vs when requested (written to register)");

                    let enter_pressed =
                        text_edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                    let add_clicked = ui.button("Add").clicked();

                    if enter_pressed || add_clicked {
                        self.add_screen_mode_breakpoint(frontend);
                    }
                });
                ui.end_row();

                // Pen color breakpoint
                ui.label("Pen color:");

                ui.horizontal(|ui| {
                    ui.label("Pen:");
                    let pen_edit = ui
                        .add_enabled(
                            !self.pen_any_number,
                            egui::TextEdit::singleline(&mut self.pen_number_input)
                                .desired_width(40.0),
                        )
                        .on_hover_text("Pen 0-15 or 16 for border");

                    ui.checkbox(&mut self.pen_any_number, "Any");

                    if self.pen_any_number {
                        self.pen_number_input.clear();
                    }

                    ui.label("Color:");
                    let color_edit = ui
                        .add_enabled(
                            !self.pen_any_color,
                            egui::TextEdit::singleline(&mut self.pen_color_value_input)
                                .desired_width(40.0),
                        )
                        .on_hover_text("Hex value 0-1F");

                    ui.checkbox(&mut self.pen_any_color, "Any");

                    if self.pen_any_color {
                        self.pen_color_value_input.clear();
                    }

                    let enter_pressed = (pen_edit.lost_focus() || color_edit.lost_focus())
                        && ui.input(|i| i.key_pressed(egui::Key::Enter));
                    let add_clicked = ui.button("Add").clicked();

                    if enter_pressed || add_clicked {
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

        let mut ga_breakpoint_found = false;
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

            ga_breakpoint_found = true;

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
                        egui::Color32::from_rgb(200, 50, 50),
                        format!("(triggered at {})", master_clock.value()),
                    );
                }
            });
        }

        if !ga_breakpoint_found {
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
            match self.screen_mode_value_input.trim().parse::<u8>() {
                Ok(val) if val <= 3 => Some(val),
                _ => return, // Invalid input
            }
        };

        let breakpoint =
            AnyBreakpoint::gate_array_screen_mode_breakpoint(mode, self.screen_mode_applied);
        frontend.breakpoint_manager().add_breakpoint(breakpoint);
        self.screen_mode_value_input.clear();
        self.screen_mode_any_change = false;
        self.screen_mode_applied = false;
    }

    fn add_pen_color_breakpoint(&mut self, frontend: &mut Frontend) {
        let pen = if self.pen_any_number {
            None
        } else {
            match self.pen_number_input.trim().parse::<usize>() {
                Ok(val) if val <= 16 => Some(val),
                _ => return, // Invalid input
            }
        };

        let color = if self.pen_any_color {
            None
        } else {
            match usize::from_str_radix(
                self.pen_color_value_input.trim().trim_start_matches("0x"),
                16,
            ) {
                Ok(val) if val <= 0x1F => Some(val as u8),
                _ => return, // Invalid input
            }
        };

        let breakpoint = AnyBreakpoint::gate_array_pen_color_breakpoint(pen, color);
        frontend.breakpoint_manager().add_breakpoint(breakpoint);
        self.pen_number_input.clear();
        self.pen_color_value_input.clear();
        self.pen_any_number = false;
        self.pen_any_color = false;
    }

    fn add_interrupt_breakpoint(&mut self, frontend: &mut Frontend) {
        let breakpoint = AnyBreakpoint::gate_array_interrupt_breakpoint();
        frontend.breakpoint_manager().add_breakpoint(breakpoint);
    }
}
