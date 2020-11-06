use crate::memory;
use crate::crtc;


pub struct GateArray {

}

impl GateArray {
    pub fn new() -> GateArray {
        GateArray {}
    }

    pub fn step(&mut self, memory: &mut memory::Memory, crtc: &crtc::CRTController) -> bool {
        false // TODO: flag interrupt
    }
}