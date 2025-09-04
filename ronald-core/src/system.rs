pub mod bus;
pub mod cpu;
pub mod instruction;
pub mod memory;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{AudioSink, VideoSink};

use bus::crtc::AnyCrtController;
use bus::gate_array::AnyGateArray;
use bus::{Bus, DummyBus, StandardBus};
use cpu::{Cpu, Register16, Register8, ZilogZ80};
use instruction::{AlgorithmicDecoder, Decoder};
use memory::{AnyMemory, MemManage, MemRead, MemWrite, MemoryCpc6128, MemoryCpcX64, Ram};

pub struct ZexHarness {
    cpu: ZilogZ80<AlgorithmicDecoder>,
    memory: Ram,
    bus: DummyBus,
}

impl ZexHarness {
    pub fn new(rom_path: &str) -> ZexHarness {
        let mut memory = Ram::from_file(0x10000, rom_path, 0x100);
        memory.write_byte(0x0005, 0xc9); // patch with RET instruction
        memory.write_word(0x0006, 0xe400); // patch with initial SP

        ZexHarness {
            cpu: ZilogZ80::new(0x100),
            memory,
            bus: DummyBus::new(),
        }
    }

    pub fn emulate(&mut self) {
        let start = std::time::Instant::now();
        let mut total_cycles = 0;

        loop {
            match self.cpu.registers.read_word(&Register16::PC) {
                0x0000 => break,
                0x0005 => {
                    match self.cpu.registers.read_byte(&Register8::C) {
                        2 => print!("{}", self.cpu.registers.read_byte(&Register8::E) as char),
                        9 => {
                            let mut address =
                                self.cpu.registers.read_word(&Register16::DE) as usize;
                            loop {
                                let character = self.memory.read_byte(address) as char;
                                if character == '$' {
                                    break;
                                } else {
                                    print!("{character}");
                                }
                                address += 1;
                            }
                        }
                        _ => unreachable!(),
                    }
                    let (cycles, _) = self.cpu.fetch_and_execute(&mut self.memory, &mut self.bus);
                    total_cycles += cycles as usize;
                }
                _ => {
                    let (cycles, _) = self.cpu.fetch_and_execute(&mut self.memory, &mut self.bus);
                    total_cycles += cycles as usize;
                }
            }
        }
        println!();

        let elapsed_seconds = start.elapsed().as_secs_f64();
        println!(
            "Executed {total_cycles} in {elapsed_seconds} seconds ({} MHz).",
            total_cycles as f64 / 1_000_000.0 / elapsed_seconds
        );
    }
}

#[derive(Serialize, Deserialize)]
pub struct DisassembledInstruction {
    address: u16,
    instruction: String,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(bound(
    serialize = "C: Serialize, M: Serialize, B: Serialize",
    deserialize = "C: Deserialize<'de>, M: Deserialize<'de>, B: Deserialize<'de>"
))]
#[serde(rename_all = "camelCase")]
pub struct AmstradCpc<C, M, B>
where
    C: Cpu,
    M: MemRead + MemWrite + MemManage + Default,
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
    M: MemRead + MemWrite + MemManage + Default,
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

    pub fn disassemble(&mut self, count: usize) -> Vec<DisassembledInstruction> {
        self.cpu
            .disassemble(&mut self.memory, count)
            .into_iter()
            .map(|(address, instruction)| DisassembledInstruction {
                address,
                instruction,
            })
            .collect()
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
