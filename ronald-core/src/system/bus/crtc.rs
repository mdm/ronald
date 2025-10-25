use serde::{Deserialize, Serialize};

pub trait CrtController: Default {
    fn read_byte(&mut self, port: u16) -> u8;
    fn write_byte(&mut self, port: u16, value: u8);
    fn step(&mut self);
    fn read_address(&self) -> usize;
    fn read_display_enabled(&self) -> bool;
    fn read_horizontal_sync(&self) -> bool;
    fn read_vertical_sync(&self) -> bool;
}

enum Register {
    // TODO: use all registers
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
}

impl HitachiHd6845s {
    fn select_register(&mut self, register: usize) {
        self.selected_register = register;
    }

    fn write_register(&mut self, value: u8) {
        self.registers[self.selected_register] = value; // TODO: restrict to registers 0-15, TODO: restrict bit-width
    }

    fn read_register(&self) -> u8 {
        self.registers[self.selected_register] // TODO: restrict to registers 14-17
    }
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

    fn step(&mut self) {
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

    fn step(&mut self) {
        match self {
            AnyCrtController::HitachiHd6845s(crtc) => crtc.step(),
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
