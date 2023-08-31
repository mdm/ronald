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

pub struct CRTController {
    registers: [u8; 18],
    selected_register: usize,
    horizontal_counter: u8,
    scan_line_counter: u8,
    character_row_counter: u8,
    display_start_address: u16,
    horizontal_sync_active: bool,
    horizontal_sync_counter: u8,
    vertical_sync_active: bool,
    vertical_sync_counter: u8,
}

impl CRTController {
    pub fn new() -> Self {
        CRTController {
            registers: [0; 18],
            selected_register: 0,
            horizontal_counter: 0,
            character_row_counter: 0,
            scan_line_counter: 0,
            display_start_address: 0,
            horizontal_sync_active: false,
            horizontal_sync_counter: 0,
            vertical_sync_active: false,
            vertical_sync_counter: 0,
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
        self.vertical_sync_active
    }

    pub fn step(&mut self) {
        self.horizontal_counter += 1;

        if self.horizontal_counter > self.registers[Register::HorizontalTotal as usize] {
            self.horizontal_counter = 0;
            self.scan_line_counter += 1;

            if self.vertical_sync_active {
                self.vertical_sync_counter += 1;

                if self.vertical_sync_counter > 16 { // TODO: don't hardcode type 2
                    self.vertical_sync_active = false;
                }
            }
        }

        if self.scan_line_counter > self.registers[Register::MaximumRasterAddress as usize] {
            self.scan_line_counter = 0;
            self.character_row_counter += 1;
        }

        if self.character_row_counter > self.registers[Register::VerticalTotal as usize] {
            if self.registers[Register::VerticalTotalAdjust as usize] != 0 {
                log::debug!(
                    "CRTC VERTICAL TOTAL ADJUST: {}",
                    self.registers[Register::VerticalTotalAdjust as usize]
                );
            }
            self.character_row_counter = 0;
        }

        if self.character_row_counter > self.registers[Register::VerticalSyncPosition as usize] {
            self.vertical_sync_active = true;
            self.vertical_sync_counter = 0;
        }

        if self.horizontal_counter == 0 && self.character_row_counter == 0 {
            self.display_start_address =
                ((self.registers[Register::DisplayStartAddressHigh as usize] as u16) << 8)
                    + self.registers[Register::DisplayStartAddressLow as usize] as u16;
        }
    }
}
