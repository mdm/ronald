use crate::memory;
use crate::gate_array;
use crate::crtc;


pub struct Bus {
    gate_array: gate_array::GateArray,
    crtc: crtc::CRTC,
}

impl Bus {
    pub fn new(gate_array: gate_array::GateArray, crtc: crtc::CRTC) -> Bus {
        Bus {
            gate_array,
            crtc,
        }
    }

    pub fn read_byte(&self, port: u16) -> u8 {
        0
    }

    pub fn write_byte(&mut self, port: u16, value: u8) {

    }

    pub fn step(&mut self, memory: &mut memory::Memory) -> bool {
        self.crtc.step();
        self.gate_array.step(memory, &self.crtc)
    }
}
