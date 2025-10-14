use eframe::egui;
use serde::{Deserialize, Serialize};

use ronald_core::debug::breakpoint::{AnyBreakpoint, Breakpoint};
use ronald_core::system::bus::crtc::Register as CrtcRegister;

use crate::frontend::Frontend;

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CrtcDebugWindow {
    pub open: bool,

    // Register write breakpoint
    #[serde(skip, default)]
    register_write_register: Option<CrtcRegister>,
    #[serde(skip, default)]
    register_write_value: Option<u8>,
    #[serde(skip, default)]
    register_write_any_register: bool,
    #[serde(skip, default)]
    register_write_any_value: bool,

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

    // Horizontal counter breakpoint
    #[serde(skip, default)]
    hcounter_value: u8,
    #[serde(skip, default)]
    hcounter_character_row: Option<u8>,
    #[serde(skip, default)]
    hcounter_any_row: bool,

    // Character row breakpoint
    #[serde(skip, default)]
    char_row_value: u8,

    // Scanline breakpoint
    #[serde(skip, default)]
    scanline_value: u8,
    #[serde(skip, default)]
    scanline_character_row: Option<u8>,
    #[serde(skip, default)]
    scanline_any_row: bool,
}

impl CrtcDebugWindow {
    pub fn ui(&mut self, ctx: &egui::Context, frontend: &mut Frontend) {
        let mut open = self.open;
        egui::Window::new("CRTC Internals")
            .open(&mut open)
            .default_width(500.0)
            .default_height(700.0)
            .show(ctx, |ui| {
                self.render_crtc_state(ui, frontend);
                ui.separator();
                self.render_breakpoints_section(ui, frontend);
            });
        self.open = open;
    }

    fn render_crtc_state(&self, ui: &mut egui::Ui, frontend: &mut Frontend) {
        let debug_view = frontend.debug_view();
        let crtc = &debug_view.crtc;

        // Registers Section
        ui.heading("CRTC Registers");

        egui::Grid::new("crtc_registers_grid")
            .num_columns(4)
            .spacing([10.0, 4.0])
            .show(ui, |ui| {
                for (i, value) in crtc.registers.iter().enumerate() {
                    let register: CrtcRegister = i.into();
                    let is_selected = matches!(crtc.selected_register, ref r if std::mem::discriminant(r) == std::mem::discriminant(&register));

                    let label = format!("{}", register);
                    if is_selected {
                        ui.colored_label(egui::Color32::YELLOW, label);
                    } else {
                        ui.label(label);
                    }

                    ui.label(format!("{:#04x}", value));

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
                ui.label(format!("{}", crtc.horizontal_counter));
                ui.end_row();

                ui.label("Character Row Counter:");
                ui.label(format!("{}", crtc.character_row_counter));
                ui.end_row();

                ui.label("Scan Line Counter:");
                ui.label(format!("{}", crtc.scan_line_counter));
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
                ui.label(format!("{:#06x}", crtc.display_start_address));
                ui.end_row();

                ui.label("Current Address:");
                ui.label(format!("{:#06x}", crtc.current_address));
                ui.end_row();
            });

        ui.add_space(8.0);

        // Status Section
        ui.heading("Status");
        ui.horizontal(|ui| {
            let hsync_color = if crtc.hsync_active {
                egui::Color32::GREEN
            } else {
                egui::Color32::DARK_GRAY
            };
            ui.colored_label(hsync_color, "HSYNC");

            ui.separator();

            let vsync_color = if crtc.vsync_active {
                egui::Color32::GREEN
            } else {
                egui::Color32::DARK_GRAY
            };
            ui.colored_label(vsync_color, "VSYNC");

            ui.separator();

            let display_color = if crtc.display_enabled {
                egui::Color32::GREEN
            } else {
                egui::Color32::DARK_GRAY
            };
            ui.colored_label(display_color, "DISPLAY");
        });
    }

    fn render_breakpoints_section(&mut self, ui: &mut egui::Ui, frontend: &mut Frontend) {
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
                        .checkbox(&mut self.register_write_any_register, "Any Reg")
                        .changed()
                        && self.register_write_any_register
                    {
                        self.register_write_register = None;
                    }

                    ui.label("Value:");
                    ui.add_enabled_ui(!self.register_write_any_value, |ui| {
                        ui.add(
                            egui::DragValue::new(self.register_write_value.get_or_insert(0))
                                .speed(1)
                                .range(0..=255),
                        );
                    });

                    if ui
                        .checkbox(&mut self.register_write_any_value, "Any")
                        .changed()
                        && self.register_write_any_value
                    {
                        self.register_write_value = None;
                    }

                    if ui.button("Add").clicked() {
                        self.add_register_write_breakpoint(frontend);
                    }
                });
                ui.end_row();

                // Horizontal sync breakpoint
                ui.label("Horizontal sync:");
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.hsync_on_start, "Start");
                    ui.checkbox(&mut self.hsync_on_end, "End");

                    if ui.button("Add").clicked() {
                        self.add_horizontal_sync_breakpoint(frontend);
                    }
                });
                ui.end_row();

                // Vertical sync breakpoint
                ui.label("Vertical sync:");
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.vsync_on_start, "Start");
                    ui.checkbox(&mut self.vsync_on_end, "End");

                    if ui.button("Add").clicked() {
                        self.add_vertical_sync_breakpoint(frontend);
                    }
                });
                ui.end_row();

                // Frame start breakpoint
                ui.label("Frame start:");
                ui.horizontal(|ui| {
                    if ui.button("Add Frame Start Breakpoint").clicked() {
                        self.add_frame_start_breakpoint(frontend);
                    }
                });
                ui.end_row();

                // Horizontal counter breakpoint
                ui.label("Horizontal counter:");
                ui.horizontal(|ui| {
                    ui.label("Value:");
                    ui.add(
                        egui::DragValue::new(&mut self.hcounter_value)
                            .speed(1)
                            .range(0..=255),
                    );

                    ui.label("Char Row:");
                    ui.add_enabled_ui(!self.hcounter_any_row, |ui| {
                        ui.add(
                            egui::DragValue::new(self.hcounter_character_row.get_or_insert(0))
                                .speed(1)
                                .range(0..=255),
                        );
                    });

                    if ui.checkbox(&mut self.hcounter_any_row, "Any").changed()
                        && self.hcounter_any_row
                    {
                        self.hcounter_character_row = None;
                    }

                    if ui.button("Add").clicked() {
                        self.add_horizontal_counter_breakpoint(frontend);
                    }
                });
                ui.end_row();

                // Character row breakpoint
                ui.label("Character row:");
                ui.horizontal(|ui| {
                    ui.label("Row:");
                    ui.add(
                        egui::DragValue::new(&mut self.char_row_value)
                            .speed(1)
                            .range(0..=255),
                    );

                    if ui.button("Add").clicked() {
                        self.add_character_row_breakpoint(frontend);
                    }
                });
                ui.end_row();

                // Scanline breakpoint
                ui.label("Scanline:");
                ui.horizontal(|ui| {
                    ui.label("Line:");
                    ui.add(
                        egui::DragValue::new(&mut self.scanline_value)
                            .speed(1)
                            .range(0..=255),
                    );

                    ui.label("Char Row:");
                    ui.add_enabled_ui(!self.scanline_any_row, |ui| {
                        ui.add(
                            egui::DragValue::new(self.scanline_character_row.get_or_insert(0))
                                .speed(1)
                                .range(0..=255),
                        );
                    });

                    if ui.checkbox(&mut self.scanline_any_row, "Any").changed()
                        && self.scanline_any_row
                    {
                        self.scanline_character_row = None;
                    }

                    if ui.button("Add").clicked() {
                        self.add_scanline_breakpoint(frontend);
                    }
                });
                ui.end_row();
            });

        // List active CRTC breakpoints
        ui.separator();
        ui.label("Active CRTC Breakpoints:");

        let mut crtc_breakpoint_found = false;
        let mut to_remove = None;
        let mut to_toggle = None;

        let breakpoint_manager = frontend.breakpoint_manager();
        for (id, breakpoint) in breakpoint_manager.breakpoints_iter() {
            if breakpoint.one_shot()
                || !matches!(
                    breakpoint,
                    AnyBreakpoint::CrtcRegisterWrite(_)
                        | AnyBreakpoint::CrtcHorizontalSync(_)
                        | AnyBreakpoint::CrtcVerticalSync(_)
                        | AnyBreakpoint::CrtcHorizontalCounter(_)
                        | AnyBreakpoint::CrtcCharacterRow(_)
                        | AnyBreakpoint::CrtcScanline(_)
                        | AnyBreakpoint::CrtcFrameStart(_)
                )
            {
                continue;
            }

            crtc_breakpoint_found = true;

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

        if !crtc_breakpoint_found {
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

    fn add_register_write_breakpoint(&mut self, frontend: &mut Frontend) {
        let register = if self.register_write_any_register {
            None
        } else {
            self.register_write_register
        };

        let value = if self.register_write_any_value {
            None
        } else {
            self.register_write_value
        };

        let breakpoint = AnyBreakpoint::crtc_register_write_breakpoint(register, value);
        frontend.breakpoint_manager().add_breakpoint(breakpoint);

        self.register_write_register = None;
        self.register_write_value = None;
        self.register_write_any_register = false;
        self.register_write_any_value = false;
    }

    fn add_horizontal_sync_breakpoint(&mut self, frontend: &mut Frontend) {
        let breakpoint =
            AnyBreakpoint::crtc_horizontal_sync_breakpoint(self.hsync_on_start, self.hsync_on_end);
        frontend.breakpoint_manager().add_breakpoint(breakpoint);

        self.hsync_on_start = false;
        self.hsync_on_end = false;
    }

    fn add_vertical_sync_breakpoint(&mut self, frontend: &mut Frontend) {
        let breakpoint =
            AnyBreakpoint::crtc_vertical_sync_breakpoint(self.vsync_on_start, self.vsync_on_end);
        frontend.breakpoint_manager().add_breakpoint(breakpoint);

        self.vsync_on_start = false;
        self.vsync_on_end = false;
    }

    fn add_frame_start_breakpoint(&mut self, frontend: &mut Frontend) {
        let breakpoint = AnyBreakpoint::crtc_frame_start_breakpoint();
        frontend.breakpoint_manager().add_breakpoint(breakpoint);
    }

    fn add_horizontal_counter_breakpoint(&mut self, frontend: &mut Frontend) {
        let character_row = if self.hcounter_any_row {
            None
        } else {
            self.hcounter_character_row
        };

        let breakpoint =
            AnyBreakpoint::crtc_horizontal_counter_breakpoint(self.hcounter_value, character_row);
        frontend.breakpoint_manager().add_breakpoint(breakpoint);

        self.hcounter_value = 0;
        self.hcounter_character_row = None;
        self.hcounter_any_row = false;
    }

    fn add_character_row_breakpoint(&mut self, frontend: &mut Frontend) {
        let breakpoint = AnyBreakpoint::crtc_character_row_breakpoint(self.char_row_value);
        frontend.breakpoint_manager().add_breakpoint(breakpoint);

        self.char_row_value = 0;
    }

    fn add_scanline_breakpoint(&mut self, frontend: &mut Frontend) {
        let character_row = if self.scanline_any_row {
            None
        } else {
            self.scanline_character_row
        };

        let breakpoint =
            AnyBreakpoint::crtc_scanline_breakpoint(self.scanline_value, character_row);
        frontend.breakpoint_manager().add_breakpoint(breakpoint);

        self.scanline_value = 0;
        self.scanline_character_row = None;
        self.scanline_any_row = false;
    }
}
