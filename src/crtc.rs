use std::cell::RefCell;
use std::rc::Rc;

pub type CRTControllerShared = Rc<RefCell<CRTController>>;

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
    pub fn new_shared() -> CRTControllerShared {
        let crtc = CRTController {
            registers: [0; 18],
            selected_register: 0,
            horizontal_counter: 0,
            horizontal_sync_width_counter: 0,
            character_row_counter: 0,
            scan_line_counter: 0,
            display_start_address: 0,
        };

        Rc::new(RefCell::new(crtc))
    }

    pub fn read_byte(&self, port: u16) -> u8 {
        dbg!(port);
        unimplemented!()
    }

    pub fn write_byte(&mut self, port: u16, value: u8) {
        dbg!(port, value);
        unimplemented!()
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
        let sync_end = self.registers[Register::VerticalSyncPosition as usize] + 16; // this can be switched between 0 and 16 on type 0
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
