pub mod bus;
pub mod cpu;
pub mod instruction;
pub mod memory;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::debug::view::{CpuDebugView, DisassembledInstruction, MemoryDebugView, SystemDebugView};
use crate::debug::Snapshotable;
use crate::{AudioSink, VideoSink};

use bus::crtc::AnyCrtController;
use bus::gate_array::AnyGateArray;
use bus::{Bus, StandardBus};
use cpu::Cpu;
use instruction::{AlgorithmicDecoder, Decoder};
use memory::{AnyMemory, MemManage, MemRead, MemWrite, MemoryCpc6128, MemoryCpcX64};

#[derive(Default, Serialize, Deserialize)]
#[serde(bound(
    serialize = "C: Serialize, M: Serialize, B: Serialize",
    deserialize = "C: Deserialize<'de>, M: Deserialize<'de>, B: Deserialize<'de>"
))]
#[serde(rename_all = "camelCase")]
pub struct AmstradCpc<C, M, B>
where
    C: Cpu,
    M: MemRead + MemWrite + MemManage,
    B: Bus,
{
    cpu: C,
    memory: M,
    #[serde(flatten)]
    bus: B,
    master_clock: u64,
    disk_drives: DiskDrives,
}

impl<C, M, B> AmstradCpc<C, M, B>
where
    C: Cpu,
    M: MemRead + MemWrite + MemManage,
    B: Bus,
{
    pub fn emulate(&mut self, video: &mut impl VideoSink, audio: &mut impl AudioSink) -> u8 {
        let (cycles, interrupt_acknowledged) =
            self.cpu.fetch_and_execute(&mut self.memory, &mut self.bus);

        // Master clock runs at 16MHz
        // CPU runs at 4MHz (master clock / 4)
        // cycles represents NOP time units, where 1 NOP = 4 CPU cycles = 16 master clock ticks
        for _ in 0..cycles {
            self.master_clock += 16;
            let interrupt = self.bus.step(&mut self.memory, video, audio);
            if interrupt {
                self.cpu.request_interrupt();
            }
        }

        if interrupt_acknowledged {
            // TODO: communicate with gate array directly?
            // What about external hardware triggering (non-maskable) interrupts?
            self.bus.acknowledge_interrupt();
        }

        cycles
    }

    pub fn set_key(&mut self, line: usize, bit: u8) {
        self.bus.set_key(line, bit);
    }

    pub fn unset_key(&mut self, line: usize, bit: u8) {
        self.bus.unset_key(line, bit);
    }

    pub fn load_disk(&mut self, drive: usize, rom: Vec<u8>, path: PathBuf) {
        // TODO: allow loading tapes as well
        self.bus.load_disk(drive, rom, path);
    }
}

impl<C, M, B> AmstradCpc<C, M, B>
where
    C: Cpu,
    M: MemRead + MemWrite + MemManage,
    B: Bus,
{
    pub fn disassemble(&self, start_address: u16, count: usize) -> Vec<DisassembledInstruction> {
        let mut decoder = AlgorithmicDecoder::default();
        let mut disassembly = Vec::with_capacity(count);
        let mut address = start_address;
        for _ in 0..count {
            let (instruction, next_address) = decoder.decode(&self.memory, address as usize);
            let instruction = instruction.to_string();
            let length = next_address - address as usize;
            disassembly.push(DisassembledInstruction {
                address,
                instruction,
                length,
            });
            address = next_address as u16;
        }
        disassembly
    }
}

impl<C, M, B> Snapshotable for AmstradCpc<C, M, B>
where
    C: Cpu + Snapshotable<View = CpuDebugView>,
    M: MemRead + MemWrite + MemManage + Snapshotable<View = MemoryDebugView> + Default,
    B: Bus,
{
    type View = SystemDebugView;

    fn debug_view(&self) -> Self::View {
        Self::View {
            master_clock: self.master_clock,
            cpu: self.cpu.debug_view(),
            memory: self.memory.debug_view(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
#[serde(default)]
pub struct SystemConfig {
    pub model: CpcModel,
    pub crtc: CrtcType,
    pub disk_drives: DiskDrives,
}

#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum CpcModel {
    Cpc464,
    Cpc664,
    Cpc6128,
}

#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum CrtcType {
    Type0,
    Type1,
    Type2,
    Type4,
}

#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum DiskDrives {
    None,
    #[default]
    One,
    Two,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            model: CpcModel::Cpc464,
            crtc: CrtcType::Type0,
            disk_drives: DiskDrives::None,
        }
    }
}

impl std::fmt::Display for CpcModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CpcModel::Cpc464 => write!(f, "Amstrad CPC 464"),
            CpcModel::Cpc664 => write!(f, "Amstrad CPC 664"),
            CpcModel::Cpc6128 => write!(f, "Amstrad CPC 6128"),
        }
    }
}

impl std::fmt::Display for CrtcType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CrtcType::Type0 => write!(f, "Type 0 (HD6845S/UM6845)"),
            CrtcType::Type1 => write!(f, "Type 1 (UM6845R)"),
            CrtcType::Type2 => write!(f, "Type 2 (MC6845)"),
            CrtcType::Type4 => write!(f, "Type 4 (AMS40226)"),
        }
    }
}

impl std::fmt::Display for DiskDrives {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiskDrives::None => write!(f, "None"),
            DiskDrives::One => write!(f, "Drive A only"),
            DiskDrives::Two => write!(f, "Drives A and B"),
        }
    }
}

impl<C> From<SystemConfig> for AmstradCpc<C, AnyMemory, StandardBus<AnyCrtController, AnyGateArray>>
where
    C: Cpu + Default,
{
    fn from(config: SystemConfig) -> Self {
        // Select memory implementation based on model
        let memory = match config.model {
            CpcModel::Cpc464 => AnyMemory::CpcX64(MemoryCpcX64::default()),
            CpcModel::Cpc664 => AnyMemory::CpcX64(MemoryCpcX64::default()),
            CpcModel::Cpc6128 => AnyMemory::Cpc6128(MemoryCpc6128::default()),
        };

        // Create CPU using Default trait
        let cpu = C::default();

        // Create bus with Any* components
        // Note: Taking liberties here as requested since not all CRTC and Gate Array variants are implemented
        let bus = StandardBus::default();

        AmstradCpc {
            cpu,
            memory,
            bus,
            master_clock: 0,
            disk_drives: config.disk_drives,
        }
    }
}
