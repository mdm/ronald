use crate::crtc;
use crate::memory;
use crate::memory::Read;
use crate::screen;

use std::cell::RefCell;
use std::rc::Rc;

pub type GateArrayShared = Rc<RefCell<GateArray>>;

pub struct GateArray {
    memory: memory::MemoryShared,
    crtc: crtc::CRTControllerShared,
    screen: screen::ScreenShared,
    current_screen_mode: u8,
    requested_screen_mode: u8,
    hsync_active: bool,
    vsync_active: bool,
    hsyncs_since_last_vsync: u8,
    interrupt_counter: u8,
    selected_pen: usize,
    pen_colors: Vec<u8>,
}

impl GateArray {
    pub fn new_shared(
        memory: memory::MemoryShared,
        crtc: crtc::CRTControllerShared,
        screen: screen::ScreenShared,
    ) -> GateArrayShared {
        let gate_array = GateArray {
            memory,
            crtc,
            screen,
            current_screen_mode: 0,
            requested_screen_mode: 0,
            hsync_active: false,
            vsync_active: false,
            hsyncs_since_last_vsync: 0,
            interrupt_counter: 0,
            selected_pen: 0,
            pen_colors: vec![0; 17],
        };

        Rc::new(RefCell::new(gate_array))
    }

    pub fn write_byte(&mut self, port: u16, value: u8) {
        // TODO: remove port parameter?
        let function = (value >> 6) & 0x03;

        match function {
            0 => {
                if value & 0x10 == 0 {
                    self.selected_pen = value as usize & 0x0f; // TODO: what if pen number exceeds max. number of pens for mode?
                } else {
                    self.selected_pen = 0x10; // select border
                }
            }
            1 => {
                // println!("color select (pen {}): {:#04x} ({:#04x})", self.selected_pen, value, value & 0x1f);
                self.pen_colors[self.selected_pen] = value & 0x1f;
            }
            2 => {
                self.requested_screen_mode = value & 0x03;

                self.memory.borrow_mut().enable_lower_rom(value & 0x04 == 0);
                self.memory.borrow_mut().enable_upper_rom(value & 0x08 == 0);

                if value & 0x10 != 0 {
                    self.interrupt_counter = 0;
                }
            }
            3 => {
                // ROM banking (only available in CPC 6128)
                println!("GA {:#06x} {:#010b}", port, value);
                unimplemented!();
            }
            _ => {
                unreachable!();
            }
        }
    }

    pub fn acknowledge_interrupt(&mut self) {
        self.interrupt_counter &= 0x1f;
    }

    pub fn step(&mut self) -> bool {
        let generate_interrupt = self.update_interrupt_counter();
        self.update_screen_mode();
        self.write_to_screen();

        self.hsync_active = self.crtc.borrow().read_horizontal_sync();
        self.vsync_active = self.crtc.borrow().read_vertical_sync();

        generate_interrupt
    }

    fn update_interrupt_counter(&mut self) -> bool {
        let mut generate_interrupt = false;
        if self.hsync_active && !self.crtc.borrow().read_horizontal_sync() {
            self.interrupt_counter += 1;
            self.hsyncs_since_last_vsync += 1;

            // TODO: handle this case properly
            if self.hsyncs_since_last_vsync > 3 {
                self.hsyncs_since_last_vsync = 3;
            }

            if self.interrupt_counter == 52 {
                self.interrupt_counter = 0;
                generate_interrupt = true;
            }
        }

        if !self.vsync_active && self.crtc.borrow().read_vertical_sync() {
            self.hsyncs_since_last_vsync = 0;
        }

        if self.hsyncs_since_last_vsync == 2 {
            self.interrupt_counter = 0;
            generate_interrupt = self.interrupt_counter & 0x20 != 0;
        }

        generate_interrupt
    }

    fn update_screen_mode(&mut self) {
        if !self.hsync_active && self.crtc.borrow().read_horizontal_sync() {
            self.current_screen_mode = self.requested_screen_mode;
            // println!("screen mode {}", self.current_screen_mode);
        }
    }

    fn write_to_screen(&self) {
        if !self.vsync_active && self.crtc.borrow().read_vertical_sync() {
            self.screen.borrow_mut().trigger_vsync();
        }

        if self.crtc.borrow().read_horizontal_sync() || self.crtc.borrow().read_vertical_sync() {
            // TODO: use modified hsync/vsync durations (see http://www.cpcwiki.eu/index.php?title=CRTC#HSYNC_and_VSYNC)
            for _ in 0..16 {
                self.screen.borrow_mut().write(20); // black
            }
            return;
        }

        if !self.crtc.borrow().read_display_enabled() {
            for _ in 0..16 {
                self.screen.borrow_mut().write(12); // bright red
            }
            return;
        }

        for offset in 0..2 {
            let address = self.crtc.borrow().read_address() + offset;
            let packed = self.memory.borrow().read_byte(address);
            match self.current_screen_mode {
                0 => {
                    let pixels = [
                        ((packed & 0x80) >> 7) | ((packed & 0x08) >> 2) | ((packed & 0x20) >> 3) | ((packed & 0x02) << 2),
                        ((packed & 0x40) >> 6) | ((packed & 0x04) >> 1) | ((packed & 0x10) >> 2) | ((packed & 0x01) << 3),
                    ];

                    for pixel in pixels {
                        for _ in 0..4 {
                            self.screen.borrow_mut().write(self.pen_colors[pixel as usize] as usize);
                        }
                    }
                }
                1 => {
                    let pixels = [
                        ((packed & 0x80) >> 7) | ((packed & 0x08) >> 2),
                        ((packed & 0x40) >> 6) | ((packed & 0x04) >> 1),
                        ((packed & 0x20) >> 5) | ((packed & 0x02) >> 0),
                        ((packed & 0x10) >> 4) | ((packed & 0x01) << 1),
                    ];

                    for pixel in pixels {
                        for _ in 0..2 {
                            self.screen.borrow_mut().write(self.pen_colors[pixel as usize] as usize);
                        }
                    }
                }
                2 => {
                    for bit in 0..8 {
                        let pixel = (packed >> (7 - bit)) & 1;
                        self.screen.borrow_mut().write(self.pen_colors[pixel as usize] as usize);
                    }                    
                }
                _ => unimplemented!(),
            }
        }
    }
}
