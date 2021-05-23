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

    pub fn step(&mut self) -> bool {
        // TODO: on HSYNC set current screen mode
        // TODO: write to screen
        false // TODO: flag interrupt
    }
}