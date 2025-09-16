use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};

use constants::KeyDefinition;
use debug::{
    breakpoint::{Breakpoint, BreakpointManager},
    view::SystemDebugView,
    Snapshotable,
};
use system::bus::{crtc::AnyCrtController, gate_array::AnyGateArray, StandardBus};
use system::cpu::ZilogZ80;
use system::instruction::AlgorithmicDecoder;
use system::memory::AnyMemory;
use system::{AmstradCpc, SystemConfig};

pub mod constants;
pub mod debug;
pub mod system;

pub trait VideoSink {
    fn draw_frame(&mut self, buffer: &Vec<u8>);
}

pub trait AudioSink {
    fn get_sample_rate(&self) -> Option<f32>;
    fn play_audio(&self);
    fn pause_audio(&self);
    fn add_sample(&self, sample: f32);
}

pub struct Driver {
    system: AmstradCpc<
        ZilogZ80<AlgorithmicDecoder>,
        AnyMemory,
        StandardBus<AnyCrtController, AnyGateArray>,
    >,
    keys: HashMap<&'static str, KeyDefinition>,
    breakpoint_manager: BreakpointManager,
    cached_debug_view: Option<SystemDebugView>,
}

impl Driver {
    pub fn new() -> Self {
        let keys = HashMap::from(constants::KEYS);
        Self {
            system: AmstradCpc::default(),
            keys,
            breakpoint_manager: BreakpointManager::default(),
            cached_debug_view: None,
        }
    }

    pub fn with_config(config: &SystemConfig) -> Self {
        let keys = HashMap::from(constants::KEYS);
        Self {
            system: config.clone().into(),
            keys,
            breakpoint_manager: BreakpointManager::default(),
            cached_debug_view: None,
        }
    }

    pub fn step(
        &mut self,
        usecs: usize,
        video: &mut impl VideoSink,
        audio: &mut impl AudioSink,
    ) -> bool {
        self.cached_debug_view = None;
        self.breakpoint_manager.prepare_breakpoints();

        let mut elapsed_microseconds = 0;
        while elapsed_microseconds < usecs {
            // TODO: tie this to vsync instead of fixed value
            elapsed_microseconds += self.system.emulate(video, audio) as usize;

            if self.breakpoint_manager.any_triggered() {
                return true;
            }
        }
        false // No breakpoint hit
    }

    pub fn press_key(&mut self, key: &str) {
        if let Some(key_definition) = self.keys.get(key) {
            self.system.set_key(key_definition.line, key_definition.bit);
        }
    }

    pub fn release_key(&mut self, key: &str) {
        if let Some(key_definition) = self.keys.get(key) {
            self.system
                .unset_key(key_definition.line, key_definition.bit);
        }
    }

    pub fn load_disk(&mut self, drive: usize, rom: Vec<u8>, path: PathBuf) {
        // TODO: Return result -> Err if slot unsuitable
        self.system.load_disk(drive, rom, path)
    }

    pub fn get_json_snapshot(&self) -> serde_json::Result<String> {
        serde_json::to_string(&self.system)
    }

    pub fn save_rom(&self) -> Vec<u8> {
        todo!()
    }

    pub fn load_snapshot(&self, rom: Vec<u8>) {
        todo!()
    }

    pub fn save_snapshot(&self) -> Vec<u8> {
        todo!()
    }

    pub fn debug_view(&mut self) -> &SystemDebugView {
        if self.cached_debug_view.is_none() {
            self.cached_debug_view = Some(self.system.debug_view());
        }

        self.cached_debug_view.as_ref().unwrap()
    }

    pub fn disassemble(
        &self,
        start_address: u16,
        count: usize,
    ) -> Vec<debug::view::DisassembledInstruction> {
        self.system.disassemble(start_address, count)
    }

    pub fn breakpoint_manager(&mut self) -> &mut BreakpointManager {
        &mut self.breakpoint_manager
    }
}

impl Default for Driver {
    fn default() -> Self {
        Self::new()
    }
}
