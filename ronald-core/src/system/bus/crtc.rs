use std::fmt;

use serde::{Deserialize, Serialize};

use crate::debug::event::CrtcDebugEvent;
use crate::debug::view::CrtcDebugView;
use crate::debug::{DebugSource, Debuggable, Snapshottable};
use crate::system::clock::MasterClockTick;

pub trait CrtController: Default {
    fn read_byte(&mut self, port: u16) -> u8;
    fn write_byte(&mut self, port: u16, value: u8);
    fn step(&mut self, master_clock: MasterClockTick);
    fn read_address(&self) -> usize;
    fn read_display_enabled(&self) -> bool;
    fn read_horizontal_sync(&self) -> bool;
    fn read_vertical_sync(&self) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    HorizontalTotal,
    HorizontalDisplayed,
    HorizontalSyncPosition,
    HorizontalAndVerticalSyncWidths,
    VerticalTotal,
    VerticalTotalAdjust,
    VerticalDisplayed,
    VerticalSyncPosition,
    InterlaceAndSkew,
    MaximumRasterAddress,
    CursorStartRaster,
    CursorEndRaster,
    DisplayStartAddressHigh,
    DisplayStartAddressLow,
    CursorAddressHigh,
    CursorAddressLow,
    LightPenAddressHigh,
    LightPenAddressLow,
}

impl From<usize> for Register {
    fn from(value: usize) -> Self {
        match value {
            0 => Register::HorizontalTotal,
            1 => Register::HorizontalDisplayed,
            2 => Register::HorizontalSyncPosition,
            3 => Register::HorizontalAndVerticalSyncWidths,
            4 => Register::VerticalTotal,
            5 => Register::VerticalTotalAdjust,
            6 => Register::VerticalDisplayed,
            7 => Register::VerticalSyncPosition,
            8 => Register::InterlaceAndSkew,
            9 => Register::MaximumRasterAddress,
            10 => Register::CursorStartRaster,
            11 => Register::CursorEndRaster,
            12 => Register::DisplayStartAddressHigh,
            13 => Register::DisplayStartAddressLow,
            14 => Register::CursorAddressHigh,
            15 => Register::CursorAddressLow,
            16 => Register::LightPenAddressHigh,
            17 => Register::LightPenAddressLow,
            _ => panic!("Invalid CRTC register: {}", value),
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Register::HorizontalTotal => write!(f, "R0 (Horizontal Total)"),
            Register::HorizontalDisplayed => write!(f, "R1 (Horizontal Displayed)"),
            Register::HorizontalSyncPosition => write!(f, "R2 (H. Sync Position)"),
            Register::HorizontalAndVerticalSyncWidths => write!(f, "R3 (H/V Sync Widths)"),
            Register::VerticalTotal => write!(f, "R4 (Vertical Total)"),
            Register::VerticalTotalAdjust => write!(f, "R5 (V. Total Adjust)"),
            Register::VerticalDisplayed => write!(f, "R6 (Vertical Displayed)"),
            Register::VerticalSyncPosition => write!(f, "R7 (V. Sync Position)"),
            Register::InterlaceAndSkew => write!(f, "R8 (Interlace/Skew)"),
            Register::MaximumRasterAddress => write!(f, "R9 (Max Raster Address)"),
            Register::CursorStartRaster => write!(f, "R10 (Cursor Start)"),
            Register::CursorEndRaster => write!(f, "R11 (Cursor End)"),
            Register::DisplayStartAddressHigh => write!(f, "R12 (Display Start High)"),
            Register::DisplayStartAddressLow => write!(f, "R13 (Display Start Low)"),
            Register::CursorAddressHigh => write!(f, "R14 (Cursor Address High)"),
            Register::CursorAddressLow => write!(f, "R15 (Cursor Address Low)"),
            Register::LightPenAddressHigh => write!(f, "R16 (Light Pen High)"),
            Register::LightPenAddressLow => write!(f, "R17 (Light Pen Low)"),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HitachiHd6845s {
    registers: [u8; 18],
    selected_register: usize,
    horizontal_counter: u8,
    horizontal_sync_width_counter: u8,
    character_row_counter: u8,
    scan_line_counter: u8,
    display_start_address: u16,
    master_clock: MasterClockTick,
    previous_hsync: bool,
    previous_vsync: bool,
    previous_display_enabled: bool,
}

impl HitachiHd6845s {
    fn select_register(&mut self, register: usize) {
        self.selected_register = register;
        self.emit_debug_event(
            CrtcDebugEvent::RegisterSelected {
                register: register.into(),
            },
            self.master_clock,
        );
    }

    fn write_register(&mut self, value: u8) {
        let was = self.registers[self.selected_register];
        self.registers[self.selected_register] = value; // TODO: restrict to registers 0-15, TODO: restrict bit-width

        self.emit_debug_event(
            CrtcDebugEvent::RegisterWritten {
                register: self.selected_register.into(),
                is: value,
                was,
            },
            self.master_clock,
        );
    }

    fn read_register(&self) -> u8 {
        self.registers[self.selected_register] // TODO: restrict to registers 14-17
    }
}

impl Snapshottable for HitachiHd6845s {
    type View = CrtcDebugView;

    fn debug_view(&self) -> Self::View {
        CrtcDebugView {
            registers: self.registers,
            selected_register: self.selected_register.into(),
            horizontal_counter: self.horizontal_counter,
            character_row_counter: self.character_row_counter,
            scan_line_counter: self.scan_line_counter,
            display_start_address: self.display_start_address,
            hsync_active: self.read_horizontal_sync(),
            vsync_active: self.read_vertical_sync(),
            display_enabled: self.read_display_enabled(),
            current_address: self.read_address(),
        }
    }
}

impl Debuggable for HitachiHd6845s {
    const SOURCE: DebugSource = DebugSource::Crtc;
    type Event = CrtcDebugEvent;
}

impl CrtController for HitachiHd6845s {
    fn read_byte(&mut self, port: u16) -> u8 {
        // TODO: get rid of port parameter?
        log::error!("Unexpected read from CRT controller");
        unimplemented!()
    }

    fn write_byte(&mut self, port: u16, value: u8) {
        let function = (port >> 8) & 0x03;

        match function {
            0 => self.select_register(value as usize),
            1 => self.write_register(value),
            _ => (),
        }
    }

    fn step(&mut self, master_clock: MasterClockTick) {
        self.master_clock = master_clock;

        // Emit events when counters are at 0 (before incrementing)
        if self.horizontal_counter == 0 {
            self.emit_debug_event(
                CrtcDebugEvent::ScanlineStart {
                    scanline: self.scan_line_counter,
                    character_row: self.character_row_counter,
                },
                master_clock,
            );
        }

        if self.scan_line_counter == 0 && self.horizontal_counter == 0 {
            self.emit_debug_event(
                CrtcDebugEvent::CharacterRowStart {
                    row: self.character_row_counter,
                },
                master_clock,
            );
        }

        if self.character_row_counter == 0
            && self.scan_line_counter == 0
            && self.horizontal_counter == 0
        {
            self.emit_debug_event(CrtcDebugEvent::FrameStart, master_clock);
        }

        self.horizontal_counter += 1;

        if self.horizontal_counter > self.registers[Register::HorizontalTotal as usize] {
            self.scan_line_counter += 1;
            self.horizontal_counter = 0;
        }

        if self.scan_line_counter > self.registers[Register::MaximumRasterAddress as usize] {
            self.character_row_counter += 1;
            self.scan_line_counter = 0;
        }

        if self.character_row_counter > self.registers[Register::VerticalTotal as usize] {
            // TODO: take VerticalTotalAdjust into account
            self.character_row_counter = 0;
        }

        if self.horizontal_counter == 0 && self.character_row_counter == 0 {
            self.display_start_address =
                ((self.registers[Register::DisplayStartAddressHigh as usize] as u16) << 8)
                    + self.registers[Register::DisplayStartAddressLow as usize] as u16;
        }

        // Check for sync state changes
        let new_hsync = self.read_horizontal_sync();
        let new_vsync = self.read_vertical_sync();
        let new_display_enabled = self.read_display_enabled();

        if new_hsync != self.previous_hsync {
            if new_hsync {
                self.emit_debug_event(
                    CrtcDebugEvent::HorizontalSyncStart {
                        horizontal_counter: self.horizontal_counter,
                        character_row: self.character_row_counter,
                        scanline: self.scan_line_counter,
                    },
                    master_clock,
                );
            } else {
                self.emit_debug_event(
                    CrtcDebugEvent::HorizontalSyncEnd {
                        horizontal_counter: self.horizontal_counter,
                        character_row: self.character_row_counter,
                        scanline: self.scan_line_counter,
                    },
                    master_clock,
                );
            }
            self.previous_hsync = new_hsync;
        }

        if new_vsync != self.previous_vsync {
            if new_vsync {
                self.emit_debug_event(
                    CrtcDebugEvent::VerticalSyncStart {
                        character_row: self.character_row_counter,
                    },
                    master_clock,
                );
            } else {
                self.emit_debug_event(
                    CrtcDebugEvent::VerticalSyncEnd {
                        character_row: self.character_row_counter,
                    },
                    master_clock,
                );
            }
            self.previous_vsync = new_vsync;
        }

        if new_display_enabled != self.previous_display_enabled {
            self.emit_debug_event(
                CrtcDebugEvent::DisplayEnableChanged {
                    enabled: new_display_enabled,
                    horizontal_counter: self.horizontal_counter,
                    character_row: self.character_row_counter,
                },
                master_clock,
            );
            self.previous_display_enabled = new_display_enabled;
        }
    }

    fn read_address(&self) -> usize {
        let refresh_memory_address = self.display_start_address
            + self.registers[Register::HorizontalDisplayed as usize] as u16
                * self.character_row_counter as u16
            + self.horizontal_counter as u16;

        let bits_14_and_15 = (refresh_memory_address & (0b11 << 12)) << 2;
        let bits_11_to_13 = ((self.scan_line_counter & 0b111) as u16) << 11;
        let bits_0_to_10 = (refresh_memory_address & 0b11_1111_1111) << 1;

        (bits_14_and_15 | bits_11_to_13 | bits_0_to_10) as usize
    }

    fn read_display_enabled(&self) -> bool {
        self.horizontal_counter < self.registers[Register::HorizontalDisplayed as usize]
            && self.character_row_counter < self.registers[Register::VerticalDisplayed as usize]
    }

    fn read_horizontal_sync(&self) -> bool {
        // TODO: what happens before registers are initialized?
        let sync_start = self.registers[Register::HorizontalSyncPosition as usize];
        let sync_end = self.registers[Register::HorizontalSyncPosition as usize]
            + (self.registers[Register::HorizontalAndVerticalSyncWidths as usize] & 0b1111);
        self.horizontal_counter >= sync_start && self.horizontal_counter < sync_end
        // this results in NO sync if the horizontal sync width is 0
    }

    fn read_vertical_sync(&self) -> bool {
        // TODO: what happens before registers are initialized?
        let sync_start = self.registers[Register::VerticalSyncPosition as usize] as i32;
        let character_rows_since_start = self.character_row_counter as i32 - sync_start;
        let scan_lines_since_start =
            (self.registers[Register::MaximumRasterAddress as usize] as i32 + 1)
                * character_rows_since_start
                + self.scan_line_counter as i32;
        (0..16).contains(&scan_lines_since_start)
    }
}

#[derive(Serialize, Deserialize)]
pub enum AnyCrtController {
    HitachiHd6845s(HitachiHd6845s),
}

impl Default for AnyCrtController {
    fn default() -> Self {
        AnyCrtController::HitachiHd6845s(HitachiHd6845s::default())
    }
}

impl Snapshottable for AnyCrtController {
    type View = CrtcDebugView;

    fn debug_view(&self) -> Self::View {
        match self {
            AnyCrtController::HitachiHd6845s(crtc) => crtc.debug_view(),
        }
    }
}

impl CrtController for AnyCrtController {
    fn read_byte(&mut self, port: u16) -> u8 {
        match self {
            AnyCrtController::HitachiHd6845s(crtc) => crtc.read_byte(port),
        }
    }

    fn write_byte(&mut self, port: u16, value: u8) {
        match self {
            AnyCrtController::HitachiHd6845s(crtc) => crtc.write_byte(port, value),
        }
    }

    fn step(&mut self, master_clock: MasterClockTick) {
        match self {
            AnyCrtController::HitachiHd6845s(crtc) => crtc.step(master_clock),
        }
    }

    fn read_address(&self) -> usize {
        match self {
            AnyCrtController::HitachiHd6845s(crtc) => crtc.read_address(),
        }
    }

    fn read_display_enabled(&self) -> bool {
        match self {
            AnyCrtController::HitachiHd6845s(crtc) => crtc.read_display_enabled(),
        }
    }

    fn read_horizontal_sync(&self) -> bool {
        match self {
            AnyCrtController::HitachiHd6845s(crtc) => crtc.read_horizontal_sync(),
        }
    }

    fn read_vertical_sync(&self) -> bool {
        match self {
            AnyCrtController::HitachiHd6845s(crtc) => crtc.read_vertical_sync(),
        }
    }
}
