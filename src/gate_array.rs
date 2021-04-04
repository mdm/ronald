use crate::memory;
use crate::crtc;

use std::cell::RefCell;
use std::rc::Rc;

pub type GateArrayShared = Rc<RefCell<GateArray>>;

pub struct GateArray {
    memory: memory::MemoryShared,
    crtc: crtc::CRTControllerShared,
}

impl GateArray {
    pub fn new_shared(memory: memory::MemoryShared, crtc: crtc::CRTControllerShared) -> GateArrayShared {
        let gate_array = GateArray {
            memory,
            crtc,
        };

        Rc::new(RefCell::new(gate_array))
    }

    pub fn write_byte(&mut self, port: u16, value: u8) {
        println!("GA {:#06x} {:#10b}", port, value);
        unimplemented!();
    }

    pub fn step(&mut self) -> bool {
        false // TODO: flag interrupt
    }
}