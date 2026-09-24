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
        debug_view: SystemDebugView,
        lower_rom_enabled: Option<bool>,
        upper_rom_enabled: Option<bool>,
        extended_ram_config: Option<u8>,
    }

    impl TestDebugger {
        pub fn with_memory_config(
            lower_rom_enabled: bool,
            upper_rom_enabled: bool,
            extended_ram_config: u8,
        ) -> Self {
            Self {
                lower_rom_enabled: Some(lower_rom_enabled),
                upper_rom_enabled: Some(upper_rom_enabled),
                extended_ram_config: Some(extended_ram_config),
                ..Default::default()
            }
        }
    }

    impl Default for TestDebugger {
        fn default() -> Self {
            let roms = HashMap::from([
                (RomSlot::Lower, vec![0; ROM_SIZE]),
                (RomSlot::Upper(0), vec![0; ROM_SIZE]),
            ]);
            let mut driver = Driver::with_config(SystemConfig {
                roms,
                ..Default::default()
            });
            let debug_view = driver.debug_view().clone();

            Self {
                driver,
                debug_view,
                lower_rom_enabled: None,
                upper_rom_enabled: None,
                extended_ram_config: None,
            }
        }
    }

    impl Debugger for TestDebugger {
        fn debug_view(&mut self) -> &SystemDebugView {
            self.debug_view = self.driver.debug_view().clone();
            if let Some(lower_rom_enabled) = self.lower_rom_enabled {
                self.debug_view.memory.lower_rom_enabled = lower_rom_enabled;
            }
            if let Some(upper_rom_enabled) = self.upper_rom_enabled {
                self.debug_view.memory.upper_rom_enabled = upper_rom_enabled;
            }
            if let Some(extended_ram_config) = self.extended_ram_config {
                self.debug_view.memory.extended_ram_config = extended_ram_config;
            }

            &self.debug_view
        }

        fn breakpoint_manager(&mut self) -> &mut BreakpointManager {
            self.driver.breakpoint_manager()
        }

        fn disassemble(&self, start_address: u16, count: usize) -> Vec<DecodedInstruction> {
            self.driver.disassemble(start_address, count)
        }
    }
}
