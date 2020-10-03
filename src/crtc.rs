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

pub struct CRTC {
    registers: [u8; 18],
    selected_register: usize,
    horizontal_counter: u8,
    horizontal_sync_width_counter: u8,
    character_row_counter: u8,
    scan_line_counter: u8,
    display_start_address: u16,
    refresh_memory_address: u16,
    row_address: u8,
}

impl CRTC {
    pub fn new() -> CRTC {
        let mut crtc = CRTC {
            registers: [0; 18],
            selected_register: 0,
            horizontal_counter: 0,
            horizontal_sync_width_counter: 0,
            character_row_counter: 0,
            scan_line_counter: 0,
            display_start_address: 0,
            refresh_memory_address: 0,
            row_address: 0,
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

    pub fn generate_address(&mut self) -> usize {
        if self.horizontal_counter == 0 && self.character_row_counter == 0 {
            self.display_start_address = ((self.registers[Register::DisplayStartAddressHigh as usize] as u16) << 8) + self.registers[Register::DisplayStartAddressLow as usize] as u16;
        }

        0
    }
}
