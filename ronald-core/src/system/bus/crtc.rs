use std::fmt;

use num_enum::{IntoPrimitive, TryFromPrimitive};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(usize)]
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
    #[num_enum(alternatives = [19..31])]
    Unused,
    Dummy = 31,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Register::HorizontalTotal => write!(f, "R{} (Horizontal Total)", usize::from(*self)),
            Register::HorizontalDisplayed => {
                write!(f, "R{} (Horizontal Displayed)", usize::from(*self))
            }
            Register::HorizontalSyncPosition => {
                write!(f, "R{} (H. Sync Position)", usize::from(*self))
            }
            Register::HorizontalAndVerticalSyncWidths => {
                write!(f, "R{} (H/V Sync Widths)", usize::from(*self))
            }
            Register::VerticalTotal => write!(f, "R{} (Vertical Total)", usize::from(*self)),
            Register::VerticalTotalAdjust => write!(f, "R{} (V. Total Adjust)", usize::from(*self)),
            Register::VerticalDisplayed => {
                write!(f, "R{} (Vertical Displayed)", usize::from(*self))
            }
            Register::VerticalSyncPosition => {
                write!(f, "R{} (V. Sync Position)", usize::from(*self))
            }
            Register::InterlaceAndSkew => write!(f, "R{} (Interlace/Skew)", usize::from(*self)),
            Register::MaximumRasterAddress => {
                write!(f, "R{} (Max Raster Address)", usize::from(*self))
            }
            Register::CursorStartRaster => write!(f, "R{} (Cursor Start)", usize::from(*self)),
            Register::CursorEndRaster => write!(f, "R{} (Cursor End)", usize::from(*self)),
            Register::DisplayStartAddressHigh => {
                write!(f, "R{} (Display Start High)", usize::from(*self))
            }
            Register::DisplayStartAddressLow => {
                write!(f, "R{} (Display Start Low)", usize::from(*self))
            }
            Register::CursorAddressHigh => {
                write!(f, "R{} (Cursor Address High)", usize::from(*self))
            }
            Register::CursorAddressLow => write!(f, "R{} (Cursor Address Low)", usize::from(*self)),
            Register::LightPenAddressHigh => write!(f, "R{} (Light Pen High)", usize::from(*self)),
            Register::LightPenAddressLow => write!(f, "R{} (Light Pen Low)", usize::from(*self)),
            Register::Unused => write!(f, "R18-R30 (Unused)"),
            Register::Dummy => write!(f, "R{} (Dummy)", usize::from(*self)),
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
    previous_address: usize,
}

impl HitachiHd6845s {
    fn select_register(&mut self, register: usize) {
        self.selected_register = register;
        self.emit_debug_event(
            CrtcDebugEvent::RegisterSelected {
                register: Register::try_from(register).expect("Invalid CRTC register selected"),
            },
            self.master_clock,
        );
    }

    fn write_register(&mut self, value: u8) {
        // TODO: restrict to writable registers
        let was = self.registers[self.selected_register];
        self.registers[self.selected_register] = value;

        self.emit_debug_event(
            CrtcDebugEvent::RegisterWritten {
                register: Register::try_from(self.selected_register).unwrap(),
                is: value,
                was,
            },
            self.master_clock,
        );
    }

    fn read_register(&self) -> u8 {
        // TODO: restrict to readable registers
        // TODO: handle type 4 reads (see https://www.cpcwiki.eu/index.php/Extra_CPC_Plus_Hardware_Information#CRTC)
        self.registers[self.selected_register]
    }
}

impl Snapshottable for HitachiHd6845s {
    type View = CrtcDebugView;

    fn debug_view(&self) -> Self::View {
        CrtcDebugView {
            registers: self.registers,
            selected_register: Register::try_from(self.selected_register).unwrap(),
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
        let function = (port >> 8) & 0x03;

        match function {
            2 => todo!("handle read depending on CRTC type"),
            3 => self.read_register(),
            _ => 0xff, // TODO: properly emulate floating bus
        }
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

        let horizontal_counter_was = self.horizontal_counter;
        let scan_line_was = self.scan_line_counter;
        let character_row_was = self.character_row_counter;

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

        self.emit_debug_event(
            CrtcDebugEvent::CountersChanged {
                character_row_is: self.character_row_counter,
                character_row_was,
                scan_line_is: self.scan_line_counter,
                scan_line_was,
                horizontal_counter_is: self.horizontal_counter,
                horizontal_counter_was,
            },
            master_clock,
        );

        if self.horizontal_counter == 0 && self.character_row_counter == 0 {
            self.display_start_address =
                ((self.registers[Register::DisplayStartAddressHigh as usize] as u16) << 8)
                    + self.registers[Register::DisplayStartAddressLow as usize] as u16;
        }

        // Check for sync state changes
        let new_hsync = self.read_horizontal_sync();
        let new_vsync = self.read_vertical_sync();
        let new_display_enabled = self.read_display_enabled();
        let new_address = self.read_address();

        if new_hsync != self.previous_hsync {
            self.emit_debug_event(
                CrtcDebugEvent::HorizontalSync { enabled: new_hsync },
                master_clock,
            );
            self.previous_hsync = new_hsync;
        }

        if new_vsync != self.previous_vsync {
            self.emit_debug_event(
                CrtcDebugEvent::VerticalSync { enabled: new_vsync },
                master_clock,
            );
            self.previous_vsync = new_vsync;
        }

        if new_display_enabled != self.previous_display_enabled {
            self.emit_debug_event(
                CrtcDebugEvent::DisplayEnableChanged {
                    enabled: new_display_enabled,
                },
                master_clock,
            );
            self.previous_display_enabled = new_display_enabled;
        }

        if new_address != self.previous_address {
            self.emit_debug_event(
                CrtcDebugEvent::AddressChanged {
                    is: new_address,
                    was: self.previous_address,
                },
                master_clock,
            );
            self.previous_address = new_address;
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
