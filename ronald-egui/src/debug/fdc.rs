use eframe::egui;
use serde::{Deserialize, Serialize};

use ronald_core::debug::breakpoint::{AnyBreakpoint, Breakpoint};
use ronald_core::system::bus::fdc::{
    Chrn, Command, CommandResult, InterruptCode, Mode, Phase, Register, StandardResult,
    StatusRegister0, StatusRegister1, StatusRegister2, StatusRegister3,
};

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
    phase_any: bool,
    #[serde(skip, default)]
    phase_on_enter: bool,
    #[serde(skip, default)]
    phase_on_leave: bool,
}

impl FdcDebugWindow {
    pub fn ui(&mut self, ui: &mut egui::Ui, debugger: &mut impl Debugger) {
        let mut open = self.show;
        egui::Window::new("FDC Internals")
            .open(&mut open)
            .show(ui, |ui| {
                self.render_fdc_state(ui, debugger);
                ui.separator();
                self.render_breakpoints_section(ui, debugger);
                ui.separator();
                self.render_fdc_buffers(ui, debugger);
            });
        self.show = open;
    }

    fn render_fdc_state(&self, ui: &mut egui::Ui, debugger: &mut impl Debugger) {
        let debug_view = debugger.debug_view();
        let fdc = &debug_view.fdc;

        ui.heading("FDC State");
        egui::Grid::new("fdc_state_grid")
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Current Phase:");
                ui.label(fdc.phase.to_string());
                ui.end_row();

                let mut bits = Vec::new();

                if fdc.main_status_register & (1 << 7) != 0 {
                    bits.push("Request for Master");
                }

                if fdc.main_status_register & (1 << 6) != 0 {
                    bits.push("Data Input");
                } else {
                    bits.push("Data Output");
                }

                if fdc.main_status_register & (1 << 5) != 0 {
                    bits.push("Execution Mode");
                }

                if fdc.main_status_register & (1 << 4) != 0 {
                    bits.push("FDC Busy");
                }

                if fdc.main_status_register & (1 << 1) != 0 {
                    bits.push("Drive B Busy");
                }

                if fdc.main_status_register & (1 << 0) != 0 {
                    bits.push("Drive A Busy");
                }

                ui.label("Main Status:");
                ui.add(egui::Label::new(bits.join(", ")).wrap());
                ui.end_row();

                ui.label("Drive A Track:");
                let track = match fdc.drive_a_track {
                    Some(track) => track.to_string(),
                    None => "No drive connected".to_string(),
                };
                ui.label(track);
                ui.end_row();

                ui.label("Drive B Track:");
                let track = match fdc.drive_b_track {
                    Some(track) => track.to_string(),
                    None => "No drive connected".to_string(),
                };
                ui.label(track);
                ui.end_row();

                ui.label("Motors:");
                ui.label(if fdc.motors_on { "On" } else { "Off" });
                ui.end_row();
            });
        ui.separator();

        let age = match fdc.phase {
            Phase::Command => "Last",
            _ => "Current",
        };

        ui.heading(format!("{} Command Details", age));
        self.render_command(ui, &fdc.current_command);
        ui.separator();

        ui.heading(format!("{} Command Result", age));
        self.render_result(ui, &fdc.current_result);
    }

    fn render_command(&self, ui: &mut egui::Ui, command: &Option<Command>) {
        egui::Grid::new("fdc_command_grid")
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Command:");
                match *command {
                    Some(Command::ReadData {
                        multi_track,
                        mode,
                        skip,
                        head,
                        unit_select,
                        chrn,
                        end_of_track,
                        gap_length,
                        data_length,
                    }) => {
                        ui.label("Read Data");
                        ui.end_row();

                        self.render_command_flags(
                            ui,
                            Some(multi_track),
                            Some(mode),
                            Some(skip),
                            Some(head),
                            Some(unit_select),
                        );

                        self.render_command_parameters(
                            ui,
                            Some(chrn),
                            Some(end_of_track),
                            Some(gap_length),
                            Some(data_length),
                        );
                    }
                    Some(Command::ReadDeletedData {
                        multi_track,
                        mode,
                        skip,
                        head,
                        unit_select,
                        chrn,
                        end_of_track,
                        gap_length,
                        data_length,
                    }) => {
                        ui.label("Read Deleted Data");
                        ui.end_row();

                        self.render_command_flags(
                            ui,
                            Some(multi_track),
                            Some(mode),
                            Some(skip),
                            Some(head),
                            Some(unit_select),
                        );

                        self.render_command_parameters(
                            ui,
                            Some(chrn),
                            Some(end_of_track),
                            Some(gap_length),
                            Some(data_length),
                        );
                    }
                    Some(Command::WriteData {
                        multi_track,
                        mode,
                        head,
                        unit_select,
                        chrn,
                        end_of_track,
                        gap_length,
                        data_length,
                    }) => {
                        ui.label("Write Data");
                        ui.end_row();

                        self.render_command_flags(
                            ui,
                            Some(multi_track),
                            Some(mode),
                            None,
                            Some(head),
                            Some(unit_select),
                        );

                        self.render_command_parameters(
                            ui,
                            Some(chrn),
                            Some(end_of_track),
                            Some(gap_length),
                            Some(data_length),
                        );
                    }
                    Some(Command::WriteDeletedData {
                        multi_track,
                        mode,
                        head,
                        unit_select,
                        chrn,
                        end_of_track,
                        gap_length,
                        data_length,
                    }) => {
                        ui.label("Write Deleted Data");
                        ui.end_row();

                        self.render_command_flags(
                            ui,
                            Some(multi_track),
                            Some(mode),
                            None,
                            Some(head),
                            Some(unit_select),
                        );

                        self.render_command_parameters(
                            ui,
                            Some(chrn),
                            Some(end_of_track),
                            Some(gap_length),
                            Some(data_length),
                        );
                    }
                    Some(Command::ReadTrack {
                        mode,
                        skip,
                        head,
                        unit_select,
                        chrn,
                        end_of_track,
                        gap_length,
                        data_length,
                    }) => {
                        ui.label("Read Track");
                        ui.end_row();

                        self.render_command_flags(
                            ui,
                            None,
                            Some(mode),
                            Some(skip),
                            Some(head),
                            Some(unit_select),
                        );

                        self.render_command_parameters(
                            ui,
                            Some(chrn),
                            Some(end_of_track),
                            Some(gap_length),
                            Some(data_length),
                        );
                    }
                    Some(Command::ReadId {
                        mode,
                        head,
                        unit_select,
                    }) => {
                        ui.label("Read ID");
                        ui.end_row();

                        self.render_command_flags(
                            ui,
                            None,
                            Some(mode),
                            None,
                            Some(head),
                            Some(unit_select),
                        );
                    }
                    Some(Command::FormatTrack {
                        mode,
                        head,
                        unit_select,
                        number,
                        sector,
                        gap_length,
                        data,
                    }) => {
                        ui.label("Format Track");
                        ui.end_row();

                        self.render_command_flags(
                            ui,
                            None,
                            Some(mode),
                            None,
                            Some(head),
                            Some(unit_select),
                        );

                        ui.label("Number:");
                        ui.label(format!("{number}"));
                        ui.end_row();

                        ui.label("Sector:");
                        ui.label(format!("{sector}"));
                        ui.end_row();

                        ui.label("Gap Length:");
                        ui.label(format!("{gap_length}"));
                        ui.end_row();

                        ui.label("Data:");
                        ui.label(format!("{data}"));
                        ui.end_row();
                    }
                    Some(Command::ScanEqual {
                        multi_track,
                        mode,
                        skip,
                        head,
                        unit_select,
                        chrn,
                        end_of_track,
                        gap_length,
                        scan_type,
                    }) => {
                        ui.label("Scan Equal");
                        ui.end_row();

                        self.render_command_flags(
                            ui,
                            Some(multi_track),
                            Some(mode),
                            Some(skip),
                            Some(head),
                            Some(unit_select),
                        );

                        self.render_command_parameters(
                            ui,
                            Some(chrn),
                            Some(end_of_track),
                            Some(gap_length),
                            None,
                        );

                        ui.label("Scan Type:");
                        ui.label(format!("{scan_type}"));
                        ui.end_row();
                    }
                    Some(Command::ScanLowOrEqual {
                        multi_track,
                        mode,
                        skip,
                        head,
                        unit_select,
                        chrn,
                        end_of_track,
                        gap_length,
                        scan_type,
                    }) => {
                        ui.label("Scan Low or Equal");
                        ui.end_row();

                        self.render_command_flags(
                            ui,
                            Some(multi_track),
                            Some(mode),
                            Some(skip),
                            Some(head),
                            Some(unit_select),
                        );

                        self.render_command_parameters(
                            ui,
                            Some(chrn),
                            Some(end_of_track),
                            Some(gap_length),
                            None,
                        );

                        ui.label("Scan Type:");
                        ui.label(format!("{scan_type}"));
                        ui.end_row();
                    }
                    Some(Command::ScanHighOrEqual {
                        multi_track,
                        mode,
                        skip,
                        head,
                        unit_select,
                        chrn,
                        end_of_track,
                        gap_length,
                        scan_type,
                    }) => {
                        ui.label("Scan High or Equal");
                        ui.end_row();

                        self.render_command_flags(
                            ui,
                            Some(multi_track),
                            Some(mode),
                            Some(skip),
                            Some(head),
                            Some(unit_select),
                        );

                        self.render_command_parameters(
                            ui,
                            Some(chrn),
                            Some(end_of_track),
                            Some(gap_length),
                            None,
                        );

                        ui.label("Scan Type:");
                        ui.label(format!("{scan_type}"));
                        ui.end_row();
                    }
                    Some(Command::Recalibrate { unit_select }) => {
                        ui.label("Recalibrate");
                        ui.end_row();

                        self.render_command_flags(ui, None, None, None, None, Some(unit_select));
                    }
                    Some(Command::SenseInterruptStatus) => {
                        ui.label("Sense Interrupt Status");
                        ui.end_row();
                    }
                    Some(Command::Specify {
                        step_rate_time,
                        head_unload_time,
                        head_load_time,
                        non_dma_mode,
                    }) => {
                        ui.label("Specify");
                        ui.end_row();

                        ui.label("Step Rate Time:");
                        ui.label(format!("{step_rate_time}"));
                        ui.end_row();

                        ui.label("Head Unload Time:");
                        ui.label(format!("{head_unload_time}"));
                        ui.end_row();

                        ui.label("Head Load Time:");
                        ui.label(format!("{head_load_time}"));
                        ui.end_row();

                        ui.label("Non-DMA Mode:");
                        match non_dma_mode {
                            true => ui.colored_label(colors::FORREST_GREEN, "YES"),
                            false => ui.colored_label(colors::DARK_RED, "NO"),
                        };
                        ui.end_row();
                    }
                    Some(Command::SenseDriveStatus { head, unit_select }) => {
                        ui.label("Sense Drive Status");
                        ui.end_row();

                        self.render_command_flags(
                            ui,
                            None,
                            None,
                            None,
                            Some(head),
                            Some(unit_select),
                        );
                    }
                    Some(Command::Seek {
                        head,
                        unit_select,
                        new_cylinder_number,
                    }) => {
                        ui.label("Seek");
                        ui.end_row();

                        self.render_command_flags(
                            ui,
                            None,
                            None,
                            None,
                            Some(head),
                            Some(unit_select),
                        );

                        ui.label("New Cylinder Number:");
                        ui.label(format!("{new_cylinder_number}"));
                        ui.end_row();
                    }
                    Some(Command::Invalid) => {
                        ui.label("Invalid");
                        ui.end_row();
                    }
                    None => {
                        ui.label("-");
                        ui.end_row();
                    }
                };
            });
    }

    fn render_command_flags(
        &self,
        ui: &mut egui::Ui,
        multi_track: Option<bool>,
        mode: Option<Mode>,
        skip: Option<bool>,
        head: Option<u8>,
        unit_select: Option<u8>,
    ) {
        match multi_track {
            Some(true) => {
                ui.label("Multi-Track:");
                ui.colored_label(colors::FORREST_GREEN, "YES");
                ui.end_row();
            }
            Some(false) => {
                ui.label("Multi-Track:");
                ui.colored_label(colors::DARK_RED, "NO");
                ui.end_row();
            }
            None => {}
        }

        match mode {
            Some(Mode::ModifiedFrequencyModulation) => {
                ui.label("Mode:");
                ui.label("Modified Frequency Modulation (MFM)");
                ui.end_row();
            }
            Some(Mode::FrequencyModulation) => {
                ui.label("Mode:");
                ui.label("Frequency Modulation (FM)");
                ui.end_row();
            }
            None => {}
        }

        match skip {
            Some(true) => {
                ui.label("Skip Deleted DAM:");
                ui.colored_label(colors::FORREST_GREEN, "YES");
                ui.end_row();
            }
            Some(false) => {
                ui.label("Skip Deleted DAM:");
                ui.colored_label(colors::DARK_RED, "NO");
                ui.end_row();
            }
            None => {}
        }

        if let Some(head) = head {
            ui.label("Head:");
            ui.label(format!("{head}"));
            ui.end_row();
        }

        if let Some(unit) = unit_select {
            ui.label("Unit Select:");
            ui.label(format!("{unit}"));
            ui.end_row();
        }
    }

    fn render_command_parameters(
        &self,
        ui: &mut egui::Ui,
        chrn: Option<Chrn>,
        end_of_track: Option<u8>,
        gap_length: Option<u8>,
        data_length: Option<u8>,
    ) {
        if let Some(Chrn {
            cylinder_number,
            head_address,
            record,
            number,
        }) = chrn
        {
            ui.label("CHRN:");
            ui.label(format!(
                "Cylinder = {}, Head = {}, Record = {},  Number = {}",
                cylinder_number, head_address, record, number
            ));
            ui.end_row();
        };

        if let Some(end_of_track) = end_of_track {
            ui.label("End of Track:");
            ui.label(format!("{end_of_track}"));
            ui.end_row();
        }

        if let Some(gap_length) = gap_length {
            ui.label("Gap Length:");
            ui.label(format!("{gap_length}"));
            ui.end_row();
        }

        if let Some(data_length) = data_length {
            ui.label("Data Length:");
            ui.label(format!("{data_length}"));
            ui.end_row();
        }
    }

    fn render_result(&self, ui: &mut egui::Ui, current_result: &Option<CommandResult>) {
        egui::Grid::new("fdc_result_grid")
            .num_columns(2)
            .show(ui, |ui| match current_result {
                Some(CommandResult::ReadData(standard_result))
                | Some(CommandResult::ReadDeletedData(standard_result))
                | Some(CommandResult::WriteData(standard_result))
                | Some(CommandResult::WriteDeletedData(standard_result))
                | Some(CommandResult::ReadTrack(standard_result))
                | Some(CommandResult::ReadId(standard_result))
                | Some(CommandResult::FormatTrack(standard_result))
                | Some(CommandResult::ScanEqual(standard_result))
                | Some(CommandResult::ScanLowOrEqual(standard_result))
                | Some(CommandResult::ScanHighOrEqual(standard_result)) => {
                    let StandardResult {
                        st0,
                        st1,
                        st2,
                        chrn,
                    } = standard_result;

                    self.render_status_register0(ui, st0);
                    self.render_status_register1(ui, st1);
                    self.render_status_register2(ui, st2);

                    let Chrn {
                        cylinder_number,
                        head_address,
                        record,
                        number,
                    } = chrn;

                    ui.label("CHRN:");
                    ui.label(format!(
                        "Cylinder = {}, Head = {}, Record = {},  Number = {}",
                        cylinder_number, head_address, record, number
                    ));
                    ui.end_row();
                }
                Some(CommandResult::Recalibrate)
                | Some(CommandResult::Specify)
                | Some(CommandResult::Seek) => {
                    ui.label("Command has no result");
                }
                Some(CommandResult::SenseInterruptStatus { st0, pcn }) => {
                    self.render_status_register0(ui, st0);

                    ui.label("Present Cylinder Number:");
                    ui.label(format!("{pcn}"));
                    ui.end_row();
                }
                Some(CommandResult::SenseDriveStatus { st3 }) => {
                    self.render_status_register3(ui, st3);
                }
                Some(CommandResult::Invalid { st0 }) => {
                    self.render_status_register0(ui, st0);
                }
                None => {}
            });
    }

    fn render_status_register0(&self, ui: &mut egui::Ui, st0: &StatusRegister0) {
        ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
            ui.label("Status Register 0:");
        });

        let mut bits = Vec::new();

        match st0.interrupt_code {
            InterruptCode::NormalTermination => {
                bits.push("Normal Termination");
            }
            InterruptCode::AbnormalTermination => {
                bits.push("Abnormal Termination");
            }
            InterruptCode::InvalidCommand => {
                bits.push("Invalid Command");
            }
            InterruptCode::ReadyChanged => {
                bits.push("Ready Changed");
            }
        }

        if st0.seek_end {
            bits.push("Seek End");
        }

        if st0.equipment_check {
            bits.push("Equipment Check");
        }

        if st0.not_ready {
            bits.push("Not Ready");
        }

        let head_address = format!("Head Address\u{00A0}=\u{00A0}{}", st0.head_address);
        bits.push(&head_address);

        let unit_select = format!("Unit Select\u{00A0}=\u{00A0}{}", st0.unit_select);
        bits.push(&unit_select);

        ui.add(egui::Label::new(bits.join(", ")).wrap());

        ui.end_row();
    }

    fn render_status_register1(&self, ui: &mut egui::Ui, st1: &StatusRegister1) {
        ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
            ui.label("Status Register 1:");
        });

        let mut bits = Vec::new();

        if st1.end_of_cylinder {
            bits.push("End of Cylinder");
        }

        if st1.data_error {
            bits.push("Data Error");
        }

        if st1.over_run {
            bits.push("Over Run");
        }

        if st1.no_data {
            bits.push("No Data");
        }

        if st1.not_writeable {
            bits.push("Not Writeable");
        }

        if st1.missing_address_mark {
            bits.push("Missing Address Mark");
        }

        ui.add(egui::Label::new(bits.join(", ")).wrap());

        ui.end_row();
    }

    fn render_status_register2(&self, ui: &mut egui::Ui, st2: &StatusRegister2) {
        ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
            ui.label("Status Register 2:");
        });

        let mut bits = Vec::new();

        if st2.control_mark {
            bits.push("Control Mark");
        }

        if st2.data_error_in_data_field {
            bits.push("Data Error in Data Field");
        }

        if st2.wrong_cylinder {
            bits.push("Wrong Cylinder");
        }

        if st2.scan_equal_hit {
            bits.push("Scan Equal Hit");
        }

        if st2.scan_not_satisfied {
            bits.push("Scan Not Satisfied");
        }

        if st2.bad_cylinder {
            bits.push("Bad Cylinder");
        }

        if st2.missing_address_mark_in_data_field {
            bits.push("Missing Address Mark in Data Field");
        }

        ui.add(egui::Label::new(bits.join(", ")).wrap());

        ui.end_row();
    }

    fn render_status_register3(&self, ui: &mut egui::Ui, st3: &StatusRegister3) {
        ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
            ui.label("Status Register 3:");
        });

        let mut bits = Vec::new();

        if st3.fault {
            bits.push("Fault");
        }

        if st3.write_protected {
            bits.push("Write Protected");
        }

        if st3.ready {
            bits.push("Ready");
        }

        if st3.track_zero {
            bits.push("Track Zero");
        }

        if st3.two_side {
            bits.push("Two Side");
        }

        let head_address = format!("Head Address\u{00A0}=\u{00A0}{}", st3.head_address);
        bits.push(&head_address);

        let unit_select = format!("Unit Select\u{00A0}=\u{00A0}{}", st3.unit_select);
        bits.push(&unit_select);

        ui.add(egui::Label::new(bits.join(", ")).wrap());

        ui.end_row();
    }

    fn render_fdc_buffers(&mut self, ui: &mut egui::Ui, debugger: &mut impl Debugger) {
        let debug_view = debugger.debug_view();
        let fdc = &debug_view.fdc;

        ui.heading("Buffers");

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("fdc_buffers_grid")
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("Command Buffer:");
                    self.render_buffer(ui, &fdc.command_buffer);
                    ui.end_row();

                    ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
                        ui.label("Data Buffer:");
                    });
                    self.render_buffer(ui, &fdc.data_buffer);
                    ui.end_row();

                    ui.label("Result Buffer:");
                    self.render_buffer(ui, &fdc.result_buffer);
                    ui.end_row();
                });
        });
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
                    ui.add_enabled_ui(!self.phase_any, |ui| {
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
                    });
                    ui.checkbox(&mut self.phase_any, "Any");
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

        let phase = if self.phase_any {
            None
        } else {
            match self.phase {
                phase @ Some(_) => phase,
                None => return, // Invalid input, don't add breakpoint
            }
        };

        let breakpoint =
            AnyBreakpoint::fdc_phase_breakpoint(phase, self.phase_on_enter, self.phase_on_leave);
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

    use ronald_core::debug::breakpoint::FdcRegisterBreakpoint;

    use crate::debug::mock::TestDebugger;

    #[test]
    fn test_fdc_debug_window_opens_and_closes() {
        let mut debugger = TestDebugger::default();
        let mut window = FdcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            window.ui(ui, &mut debugger);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        // Check that the window title is rendered
        harness.get_by_label("FDC Internals");

        // Click close button
        harness.get_by_label("Close window").click();
        harness.run();

        // Window should no longer be visible
        assert!(harness.query_by_label("FDC Internals").is_none());
    }

    #[test]
    fn test_fdc_debug_window_register_read_breakpoint_with_register_and_value() {
        let mut debugger = TestDebugger::default();
        let mut window = FdcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            window.ui(ui, &mut debugger);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        let i = 0;

        // Select register "Data"
        harness
            .get_all_by_role(accesskit::Role::ComboBox)
            .nth(i)
            .unwrap()
            .click();
        harness.run();
        harness.get_by_label("Data").click();
        harness.run();

        // Enter value "0x42"
        let input = harness
            .get_all_by_role_and_label(accesskit::Role::TextInput, "Value:")
            .nth(i)
            .unwrap();
        input.focus();
        input.type_text("0x42");
        harness.run();

        // Add breakpoint
        harness
            .get_all_by_role_and_label(accesskit::Role::Button, "Add")
            .nth(i)
            .unwrap()
            .click();
        harness.run();

        assert!(harness.query_by_label("Data read = 0x42").is_some());

        // Remove breakpoint
        harness
            .get_by_role_and_label(accesskit::Role::Button, "Remove")
            .click();
        harness.run();

        assert!(harness.query_by_label("Data read = 0x42").is_none());
    }

    #[test]
    fn test_fdc_debug_window_register_read_breakpoint_any_register() {
        let mut debugger = TestDebugger::default();
        let mut window = FdcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            window.ui(ui, &mut debugger);
        };

        let mut harness = Harness::new_ui(app);
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
        let input = harness
            .get_all_by_role_and_label(accesskit::Role::TextInput, "Value:")
            .nth(i)
            .unwrap();
        input.focus();
        input.type_text("0x42");
        harness.run();

        // Add breakpoint
        harness
            .get_all_by_role_and_label(accesskit::Role::Button, "Add")
            .nth(i)
            .unwrap()
            .click();
        harness.run();

        harness.get_by_label("Any register read = 0x42").click();
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
                        AnyBreakpoint::FdcRegister(FdcRegisterBreakpoint {
                            register: None,
                            value: Some(0x42),
                            on_read: true,
                            on_write: false,
                            ..
                        })
                    ) && !bp.enabled()
                })
        );
    }

    #[test]
    fn test_fdc_debug_window_register_read_breakpoint_any_value() {
        let mut debugger = TestDebugger::default();
        let mut window = FdcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            window.ui(ui, &mut debugger);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        let i = 0;

        // Select register "Data"
        harness
            .get_all_by_role(accesskit::Role::ComboBox)
            .nth(i)
            .unwrap()
            .click();
        harness.run();
        harness.get_by_label("Data").click();
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
        harness.get_by_label("Data read = Any value").click();
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
                        AnyBreakpoint::FdcRegister(FdcRegisterBreakpoint {
                            register: Some(Register::Data),
                            value: None,
                            on_read: true,
                            on_write: false,
                            ..
                        })
                    ) && !bp.enabled()
                })
        );
    }

    #[test]
    fn test_fdc_debug_window_register_read_breakpoint_invalid_value() {
        let mut debugger = TestDebugger::default();
        let mut window = FdcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            window.ui(ui, &mut debugger);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        let i = 0;

        // Select register "Data"
        harness
            .get_all_by_role(accesskit::Role::ComboBox)
            .nth(i)
            .unwrap()
            .click();
        harness.run();
        harness.get_by_label("Data").click();
        harness.run();

        // Enter value "invalid"
        let input = harness
            .get_all_by_role_and_label(accesskit::Role::TextInput, "Value:")
            .nth(i)
            .unwrap();
        input.focus();
        input.type_text("invalid");
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
    fn test_fdc_debug_window_register_write_breakpoint_with_register_and_value() {
        let mut debugger = TestDebugger::default();
        let mut window = FdcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            window.ui(ui, &mut debugger);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        let i = 1;

        // Select register "Data"
        harness
            .get_all_by_role(accesskit::Role::ComboBox)
            .nth(i)
            .unwrap()
            .click();
        harness.run();
        harness.get_by_label("Data").click();
        harness.run();

        // Enter value "0x42"
        let input = harness
            .get_all_by_role_and_label(accesskit::Role::TextInput, "Value:")
            .nth(i)
            .unwrap();
        input.focus();
        input.type_text("0x42");
        harness.run();

        // Add breakpoint
        harness
            .get_all_by_role_and_label(accesskit::Role::Button, "Add")
            .nth(i)
            .unwrap()
            .click();
        harness.run();

        assert!(harness.query_by_label("Data write = 0x42").is_some());

        // Remove breakpoint
        harness
            .get_by_role_and_label(accesskit::Role::Button, "Remove")
            .click();
        harness.run();

        assert!(harness.query_by_label("Data write = 0x42").is_none());
    }

    #[test]
    fn test_fdc_debug_window_register_write_breakpoint_any_register() {
        let mut debugger = TestDebugger::default();
        let mut window = FdcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            window.ui(ui, &mut debugger);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        let i = 1;

        // Select any register
        harness
            .get_all_by_role_and_label(accesskit::Role::CheckBox, "Any")
            .nth(2 * i)
            .unwrap()
            .click();
        harness.run();

        // Enter value "0x42"
        let input = harness
            .get_all_by_role_and_label(accesskit::Role::TextInput, "Value:")
            .nth(i)
            .unwrap();
        input.focus();
        input.type_text("0x42");
        harness.run();

        // Add breakpoint
        harness
            .get_all_by_role_and_label(accesskit::Role::Button, "Add")
            .nth(i)
            .unwrap()
            .click();
        harness.run();

        harness.get_by_label("Any register write = 0x42").click();
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
                        AnyBreakpoint::FdcRegister(FdcRegisterBreakpoint {
                            register: None,
                            value: Some(0x42),
                            on_read: false,
                            on_write: true,
                            ..
                        })
                    ) && !bp.enabled()
                })
        );
    }

    #[test]
    fn test_fdc_debug_window_register_write_breakpoint_any_value() {
        let mut debugger = TestDebugger::default();
        let mut window = FdcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            window.ui(ui, &mut debugger);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        let i = 1;

        // Select register "Data"
        harness
            .get_all_by_role(accesskit::Role::ComboBox)
            .nth(i)
            .unwrap()
            .click();
        harness.run();
        harness.get_by_label("Data").click();
        harness.run();

        // Check any value
        harness
            .get_all_by_role_and_label(accesskit::Role::CheckBox, "Any")
            .nth(2 * i + 1)
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
        harness.get_by_label("Data write = Any value").click();
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
                        AnyBreakpoint::FdcRegister(FdcRegisterBreakpoint {
                            register: Some(Register::Data),
                            value: None,
                            on_read: false,
                            on_write: true,
                            ..
                        })
                    ) && !bp.enabled()
                })
        );
    }

    #[test]
    fn test_fdc_debug_window_register_write_breakpoint_invalid_value() {
        let mut debugger = TestDebugger::default();
        let mut window = FdcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            window.ui(ui, &mut debugger);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        let i = 1;

        // Select register "Data"
        harness
            .get_all_by_role(accesskit::Role::ComboBox)
            .nth(i)
            .unwrap()
            .click();
        harness.run();
        harness.get_by_label("Data").click();
        harness.run();

        // Enter value "invalid"
        let input = harness
            .get_all_by_role_and_label(accesskit::Role::TextInput, "Value:")
            .nth(i)
            .unwrap();
        input.focus();
        input.type_text("invalid");
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
    fn test_fdc_debug_window_phase_breakpoint_entered_or_left() {
        let mut debugger = TestDebugger::default();
        let mut window = FdcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            window.ui(ui, &mut debugger);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        let i = 2;

        // Select phase "Execution"
        harness
            .get_all_by_role(accesskit::Role::ComboBox)
            .nth(i)
            .unwrap()
            .click();
        harness.run();
        harness.get_by_label("Execution").click();
        harness.run();

        // Check start
        harness
            .get_all_by_role_and_label(accesskit::Role::CheckBox, "Enter")
            .next()
            .unwrap()
            .click();
        harness.run();

        // Check end
        harness
            .get_all_by_role_and_label(accesskit::Role::CheckBox, "Leave")
            .next()
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

        assert!(
            harness
                .query_by_label("Execution phase entered or left")
                .is_some()
        );

        // Remove breakpoint
        harness
            .get_by_role_and_label(accesskit::Role::Button, "Remove")
            .click();
        harness.run();

        assert!(
            harness
                .query_by_label("Execution phase entered or left")
                .is_none()
        );
    }

    #[test]
    fn test_fdc_debug_window_phase_breakpoint_any_phase() {
        let mut debugger = TestDebugger::default();
        let mut window = FdcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            window.ui(ui, &mut debugger);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        let i = 2;

        // Check any phase
        harness
            .get_all_by_role_and_label(accesskit::Role::CheckBox, "Any")
            .nth(2 * i)
            .unwrap()
            .click();
        harness.run();

        // Check start
        harness
            .get_all_by_role_and_label(accesskit::Role::CheckBox, "Enter")
            .next()
            .unwrap()
            .click();
        harness.run();

        // Check end
        harness
            .get_all_by_role_and_label(accesskit::Role::CheckBox, "Leave")
            .next()
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

        assert!(harness.query_by_label("Phase entered or left").is_some());

        // Remove breakpoint
        harness
            .get_by_role_and_label(accesskit::Role::Button, "Remove")
            .click();
        harness.run();

        assert!(harness.query_by_label("Phase entered or left").is_none());
    }

    #[test]
    fn test_fdc_debug_window_phase_breakpoint_invalid() {
        let mut debugger = TestDebugger::default();
        let mut window = FdcDebugWindow {
            show: true,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            window.ui(ui, &mut debugger);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        let i = 2;

        // Check any phase
        harness
            .get_all_by_role_and_label(accesskit::Role::CheckBox, "Any")
            .nth(2 * i)
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
        drop(harness);

        assert_eq!(debugger.breakpoint_manager().breakpoints_iter().count(), 0);
    }
}
