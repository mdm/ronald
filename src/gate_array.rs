use crate::memory;
use crate::crtc;

use std::cell::RefCell;
use std::rc::Rc;

pub type GateArrayShared = Rc<RefCell<GateArray>>;

pub struct GateArray {
    memory: memory::MemoryShared,
    crtc: crtc::CRTControllerShared,
    current_screen_mode: u8,
    requested_screen_mode: u8,
    hsync_active: bool,
    vsync_active: bool,
    hsyncs_since_last_vsync: u8,
    interrupt_counter: u8,
}

impl GateArray {
    pub fn new_shared(memory: memory::MemoryShared, crtc: crtc::CRTControllerShared) -> GateArrayShared {
        let gate_array = GateArray {
            memory,
            crtc,
            current_screen_mode: 0,
            requested_screen_mode: 0,
            hsync_active: false,
            vsync_active: false,
            hsyncs_since_last_vsync: 0,
            interrupt_counter: 0,
        };

        Rc::new(RefCell::new(gate_array))
    }

    pub fn write_byte(&mut self, port: u16, value: u8) { // TODO: remove port parameter?
        match value {
            _ if value & 0x80 != 0 && value & 0x40 == 0 => {
                self.requested_screen_mode = value & 0x03;

                self.memory.borrow_mut().enable_lower_rom(value & 0x04 == 0);
                self.memory.borrow_mut().enable_upper_rom(value & 0x08 == 0);

                if value & 0x10 != 0 {
                    self.interrupt_counter = 0;
                }
            }
            _ => {
                println!("GA {:#06x} {:#10b}", port, value);
                unimplemented!();        
            }
        }
    }

    pub fn acknowledge_interrupt(&mut self) {
        self.interrupt_counter &= 0x1f;
    }

    pub fn step(&mut self) -> bool {
        let generate_interrupt = self.update_interrupt_counter();
        self.update_screen_mode();

        self.hsync_active = self.crtc.borrow_mut().read_horizontal_sync();

        // TODO: write to screen

        generate_interrupt
    }

    fn update_interrupt_counter(&mut self) -> bool {
        let mut generate_interrupt = false;
        if self.hsync_active && !self.crtc.borrow_mut().read_horizontal_sync() {
            self.interrupt_counter += 1;
            self.hsyncs_since_last_vsync += 1;

            if self.interrupt_counter == 52 {
                self.interrupt_counter = 0;
                generate_interrupt = true;
            }
        }

        if !self.vsync_active && self.crtc.borrow_mut().read_vertical_sync() {
            self.hsyncs_since_last_vsync = 0;
        }

        if self.hsyncs_since_last_vsync == 2 {
            self.interrupt_counter = 0;
            generate_interrupt = self.interrupt_counter & 0x20 == 0;
        }

        generate_interrupt
    }

    fn update_screen_mode(&mut self) {
        if !self.hsync_active && self.crtc.borrow_mut().read_horizontal_sync() {
            self.current_screen_mode = self.requested_screen_mode;
        }
    }
}