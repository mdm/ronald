mod cpu;
mod crtc;
mod gate_array;
mod memory;

use ronald_core::{
    debug::{breakpoint::BreakpointManager, view::SystemDebugView},
    system::instruction::DecodedInstruction,
};

pub use cpu::CpuDebugWindow;
pub use crtc::CrtcDebugWindow;
pub use gate_array::GateArrayDebugWindow;
pub use memory::MemoryDebugWindow;

pub trait Debugger {
    fn debug_view(&mut self) -> &SystemDebugView;
    fn breakpoint_manager(&mut self) -> &mut BreakpointManager;
    fn disassemble(&self, start_address: u16, count: usize) -> Vec<DecodedInstruction>;
}

#[cfg(test)]
mod mock {
    use super::*;
    use ronald_core::Driver;

    #[derive(Default)]
    pub struct TestDebugger {
        driver: Driver,
    }

    impl Debugger for TestDebugger {
        fn debug_view(&mut self) -> &SystemDebugView {
            self.driver.debug_view()
        }

        fn breakpoint_manager(&mut self) -> &mut BreakpointManager {
            self.driver.breakpoint_manager()
        }

        fn disassemble(&self, start_address: u16, count: usize) -> Vec<DecodedInstruction> {
            self.driver.disassemble(start_address, count)
        }
    }
}
