use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrtControllerSnapshot {
    horizontal_total: u8,
    horizontal_displayed: u8,
    horizontal_sync_position: u8,
    horizontal_and_vertical_sync_widths: u8,
    vertical_total: u8,
    vertical_total_adjust: u8,
    vertical_displayed: u8,
    vertical_sync_position: u8,
    interlace_and_skew: u8,
    maximum_raster_address: u8,
    cursor_start_raster: u8,
    cursor_end_raster: u8,
    display_start_address_high: u8,
    cisplay_start_address_low: u8,
    cursor_address_high: u8,
    cursor_address_low: u8,
    light_pen_address_high: u8,
    light_pen_address_low: u8,
    selected_register: usize,
    horizontal_counter: u8,
    horizontal_sync_width_counter: u8,
    character_row_counter: u8,
    scan_line_counter: u8,
    display_start_address: u16,
}

enum Register {
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

pub struct CrtController {
    registers: [u8; 18],
    selected_register: usize,
    horizontal_counter: u8,
    horizontal_sync_width_counter: u8,
    character_row_counter: u8,
    scan_line_counter: u8,
    display_start_address: u16,
}

impl CrtController {
    pub fn new() -> Self {
        CrtController {
            registers: [0; 18],
            selected_register: 0,
            horizontal_counter: 0,
            horizontal_sync_width_counter: 0,
            character_row_counter: 0,
            scan_line_counter: 0,
            display_start_address: 0,
        }
    }

    pub fn read_byte(&self, port: u16) -> u8 {
        // TODO: get rid of port parameter?
        log::error!("Unexpected read from CRT controller");
        unimplemented!()
    }

    pub fn write_byte(&mut self, port: u16, value: u8) {
        let function = (port >> 8) & 0x03;

        match function {
            0 => self.select_register(value as usize),
            1 => self.write_register(value),
            _ => (),
        }
    }

    fn select_register(&mut self, register: usize) {
        self.selected_register = register;
    }

    fn write_register(&mut self, value: u8) {
        self.registers[self.selected_register] = value; // TODO: restrict to registers 0-15, TODO: restrict bit-width
    }

    fn read_register(&self) -> u8 {
        self.registers[self.selected_register] // TODO: restrict to registers 14-17
    }

    pub fn read_address(&self) -> usize {
        let refresh_memory_address = self.display_start_address
            + self.registers[Register::HorizontalDisplayed as usize] as u16
                * self.character_row_counter as u16
            + self.horizontal_counter as u16;

        let bits_14_and_15 = (refresh_memory_address & (0b11 << 12)) << 2;
        let bits_11_to_13 = ((self.scan_line_counter & 0b111) as u16) << 11;
        let bits_0_to_10 = (refresh_memory_address & 0b11_1111_1111) << 1;

        (bits_14_and_15 | bits_11_to_13 | bits_0_to_10) as usize
    }

    pub fn read_display_enabled(&self) -> bool {
        self.horizontal_counter < self.registers[Register::HorizontalDisplayed as usize]
            && self.character_row_counter < self.registers[Register::VerticalDisplayed as usize]
    }

    pub fn read_horizontal_sync(&self) -> bool {
        // TODO: what happens before registers are initialized?
        let sync_start = self.registers[Register::HorizontalSyncPosition as usize];
        let sync_end = self.registers[Register::HorizontalSyncPosition as usize]
            + (self.registers[Register::HorizontalAndVerticalSyncWidths as usize] & 0b1111);
        self.horizontal_counter >= sync_start && self.horizontal_counter < sync_end
        // this results in NO sync if the horizontal sync width is 0
    }

    pub fn read_vertical_sync(&self) -> bool {
        // TODO: what happens before registers are initialized?
        let sync_start = self.registers[Register::VerticalSyncPosition as usize] as i32;
        let character_rows_since_start = self.character_row_counter as i32 - sync_start;
        let scan_lines_since_start =
            (self.registers[Register::MaximumRasterAddress as usize] as i32 + 1)
                * character_rows_since_start
                + self.scan_line_counter as i32;
        (0..16 * 8).contains(&scan_lines_since_start) // should be 16, not 16 * 8 - TODO: find out why shortening this messes with Fruity Frank colors
    }

    pub fn step(&mut self) {
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

    pub fn make_snapshot(&self) -> CrtControllerSnapshot {
        CrtControllerSnapshot {
            horizontal_total: self.registers[0],
            horizontal_displayed: self.registers[1],
            horizontal_sync_position: self.registers[2],
            horizontal_and_vertical_sync_widths: self.registers[3],
            vertical_total: self.registers[4],
            vertical_total_adjust: self.registers[5],
            vertical_displayed: self.registers[6],
            vertical_sync_position: self.registers[7],
            interlace_and_skew: self.registers[8],
            maximum_raster_address: self.registers[9],
            cursor_start_raster: self.registers[10],
            cursor_end_raster: self.registers[11],
            display_start_address_high: self.registers[12],
            cisplay_start_address_low: self.registers[13],
            cursor_address_high: self.registers[14],
            cursor_address_low: self.registers[15],
            light_pen_address_high: self.registers[16],
            light_pen_address_low: self.registers[17],
            selected_register: self.selected_register,
            horizontal_counter: self.horizontal_counter,
            horizontal_sync_width_counter: self.horizontal_sync_width_counter,
            character_row_counter: self.character_row_counter,
            scan_line_counter: self.scan_line_counter,
            display_start_address: self.display_start_address,
        }
    }
}
