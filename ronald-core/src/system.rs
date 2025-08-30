pub mod bus;
pub mod cpu;
mod instruction; // TODO: do we need this at this level? for debugger?
pub mod memory;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{AudioSink, VideoSink};

use bus::{Bus, DummyBus, StandardBus};
use cpu::{Cpu, Register16, Register8, ZilogZ80};
use memory::{MemManage, MemRead, MemWrite, Memory, Ram};

pub struct ZexHarness {
    cpu: ZilogZ80,
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

#[derive(Serialize, Deserialize)]
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
    // debugger: Debugger,
}

impl<C, M, B> AmstradCpc<C, M, B>
where
    C: Cpu,
    M: MemRead + MemWrite + MemManage + Default,
    B: Bus,
{
    pub fn new() -> Self {
        let cpu = C::default();
        let memory = M::default();
        let bus = B::default();

        Self {
            cpu,
            memory,
            bus,
            master_clock: 0,
        }
    }

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

impl<C, M, B> Default for AmstradCpc<C, M, B>
where
    C: Cpu,
    M: MemRead + MemWrite + MemManage + Default,
    B: Bus,
{
    fn default() -> Self {
        Self::new()
    }
}
