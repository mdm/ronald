mod cpu;
mod crtc;
mod fdc;
mod gate_array;
mod memory;

use ronald_core::{
    debug::{breakpoint::BreakpointManager, view::SystemDebugView},
    system::instruction::DecodedInstruction,
};

pub use cpu::CpuDebugWindow;
pub use crtc::CrtcDebugWindow;
pub use fdc::FdcDebugWindow;
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

    use std::collections::HashMap;

    use ronald_core::{
        Driver,
        system::{SystemConfig, memory::RomSlot},
    };

    const ROM_SIZE: usize = 0x4000;

    pub struct TestDebugger {
        driver: Driver,
    }

    impl Default for TestDebugger {
        fn default() -> Self {
            let roms = HashMap::from([
                (RomSlot::Lower, vec![0; ROM_SIZE]),
                (RomSlot::Upper(0), vec![0; ROM_SIZE]),
            ]);

            Self {
                driver: Driver::with_config(SystemConfig {
                    roms,
                    ..Default::default()
                }),
            }
        }
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
