use std::{collections::HashMap, path::PathBuf};

use constants::KeyDefinition;
use debug::{view::SystemDebugView, Snapshotable};
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
}

impl Driver {
    pub fn new() -> Self {
        let keys = HashMap::from(constants::KEYS);
        Self {
            system: AmstradCpc::default(),
            keys,
        }
    }

    pub fn with_config(config: &SystemConfig) -> Self {
        let keys = HashMap::from(constants::KEYS);
        Self {
            system: config.clone().into(),
            keys,
        }
    }

    pub fn step(&mut self, usecs: usize, video: &mut impl VideoSink, audio: &mut impl AudioSink) {
        let mut elapsed_microseconds = 0;
        while elapsed_microseconds < usecs {
            // TODO: tie this to vsync instead of fixed value
            elapsed_microseconds += self.system.emulate(video, audio) as usize;
        }
    }

    pub fn step_single(&mut self, video: &mut impl VideoSink, audio: &mut impl AudioSink) {
        self.system.emulate(video, audio);
    }

    pub fn activate_debugger(&self) {
        todo!()
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

    pub fn disassemble(&mut self, count: usize) -> serde_json::Result<String> {
        let disassembly = self.system.disassemble(count);
        serde_json::to_string(&disassembly)
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

    pub fn debug_view(&self) -> SystemDebugView {
        self.system.debug_view()
    }
}

impl Default for Driver {
    fn default() -> Self {
        Self::new()
    }
}
