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
}

impl CRTC {
    pub fn new() -> CRTC {
        let mut crtc = CRTC {
            registers: [0; 18],
            selected_register: 0,
        };
        crtc.registers[Register::HorizontalTotal as usize] = 63;
        crtc.registers[Register::HorizontalDisplayed as usize] = 40;
        crtc.registers[Register::HorizontalSyncPosition as usize] = 46;
        crtc.registers[Register::HorizontalAndVerticalSyncWidths as usize] = 128 + 14;
        crtc.registers[Register::VerticalTotal as usize] = 38;
        crtc.registers[Register::VerticalTotalAdjust as usize] = 0;
        crtc.registers[Register::VerticalDisplayed as usize] = 25;
        crtc.registers[Register::VerticalSyncPosition as usize] = 30;
        crtc.registers[Register::InterlaceAndSkew as usize] = 0;
        crtc.registers[Register::MaximumRasterAddress as usize] = 7;
        crtc.registers[Register::CursorStartRaster as usize] = 0;
        crtc.registers[Register::CursorEndRaster as usize] = 0;
        crtc.registers[Register::DisplayStartAddressHigh as usize] = 48;
        crtc.registers[Register::DisplayStartAddressLow as usize] = 0;
        crtc.registers[Register::CursorAddressHigh as usize] = 0;
        crtc.registers[Register::CursorAddressLow as usize] = 0;
        crtc.registers[Register::LightPenAddressHigh as usize] = 0;
        crtc.registers[Register::LightPenAddressLow as usize] = 0;
    
        crtc
    }

    pub fn select_register(&mut self, register: usize) {
        self.selected_register = register;
    }

    pub fn write_register(&mut self, value: u8) {
        self.registers[self.selected_register] = value;
    }
}
