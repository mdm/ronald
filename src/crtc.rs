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
    horizontal_sync_width_counter: u8,
    character_row_counter: u8,
    scan_line_counter: u8,
    display_start_address: u16,
}

impl CRTController {
    pub fn new() -> CRTController {
        let mut crtc = CRTController {
            registers: [0; 18],
            selected_register: 0,
            horizontal_counter: 0,
            horizontal_sync_width_counter: 0,
            character_row_counter: 0,
            scan_line_counter: 0,
            display_start_address: 0,
        };
        // crtc.registers[Register::HorizontalTotal as usize] = 63;
        // crtc.registers[Register::HorizontalDisplayed as usize] = 40;
        // crtc.registers[Register::HorizontalSyncPosition as usize] = 46;
        // crtc.registers[Register::HorizontalAndVerticalSyncWidths as usize] = 128 + 14; // VVVVHHHH: VVVV is fixed to 16 on type 2 CRTCs, HHHH == 0 means no HS
        // crtc.registers[Register::VerticalTotal as usize] = 38;
        // crtc.registers[Register::VerticalTotalAdjust as usize] = 0;
        // crtc.registers[Register::VerticalDisplayed as usize] = 25;
        // crtc.registers[Register::VerticalSyncPosition as usize] = 30;
        // crtc.registers[Register::InterlaceAndSkew as usize] = 0;
        // crtc.registers[Register::MaximumRasterAddress as usize] = 7;
        // crtc.registers[Register::CursorStartRaster as usize] = 0;
        // crtc.registers[Register::CursorEndRaster as usize] = 0;
        // crtc.registers[Register::DisplayStartAddressHigh as usize] = 48;
        // crtc.registers[Register::DisplayStartAddressLow as usize] = 0;
        // crtc.registers[Register::CursorAddressHigh as usize] = 0;
        // crtc.registers[Register::CursorAddressLow as usize] = 0;
        // crtc.registers[Register::LightPenAddressHigh as usize] = 0;
        // crtc.registers[Register::LightPenAddressLow as usize] = 0;
    
        crtc
    }

    pub fn select_register(&mut self, register: usize) {
        self.selected_register = register;
    }

    pub fn write_register(&mut self, value: u8) {
        self.registers[self.selected_register] = value; // TODO: restrict to registers 0-15, TODO: restrict bit-width
    }

    pub fn read_register(&self) -> u8 {
        self.registers[self.selected_register] // TODO: restrict to registers 14-17
    }

    pub fn read_address(&self) -> usize {
        let refresh_memory_address = self.display_start_address + self.registers[Register::HorizontalDisplayed as usize] as u16 * self.character_row_counter as u16 + self.horizontal_counter as u16;

        let bits_14_and_15 = (refresh_memory_address & (0b11 << 12)) << 2;
        let bits_11_to_13 = ((self.scan_line_counter & 0b111) as u16) << 11;
        let bits_0_to_10 = (refresh_memory_address & 0b11_11111111) << 1;

        (bits_14_and_15 | bits_11_to_13 | bits_0_to_10) as usize
    }

    pub fn read_display_enabled(&self) -> bool {
        self.horizontal_counter < self.registers[Register::HorizontalDisplayed as usize] && self.character_row_counter < self.registers[Register::VerticalDisplayed as usize]
    }

    pub fn read_horizontal_sync(&self) -> bool {
        let sync_start = self.registers[Register::HorizontalSyncPosition as usize];
        let sync_end = self.registers[Register::HorizontalSyncPosition as usize] + (self.registers[Register::HorizontalAndVerticalSyncWidths as usize] & 0b1111);
        self.horizontal_counter >= sync_start && self.horizontal_counter < sync_end // this results in NO sync if the horizontal sync width is 0
    }

    pub fn read_vertical_sync(&self) -> bool {
        let sync_start = self.registers[Register::VerticalSyncPosition as usize];
        let sync_end = self.registers[Register::VerticalSyncPosition as usize] + 16;
        self.character_row_counter >= sync_start && self.character_row_counter < sync_end
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

        if self.character_row_counter > self.registers[Register::VerticalTotal as usize] { // TODO: take VerticalTotalAdjust into account
            self.character_row_counter = 0;
        }

        if self.horizontal_counter == 0 && self.character_row_counter == 0 {
            self.display_start_address = ((self.registers[Register::DisplayStartAddressHigh as usize] as u16) << 8) + self.registers[Register::DisplayStartAddressLow as usize] as u16;
        }
    }
}
