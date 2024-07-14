mod bus;
mod cpu;
// mod debugger;
mod instruction; // TODO: do we need this at this level? for debugger?
mod memory;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{AudioSink, VideoSink};

use bus::{DummyBus, StandardBus};
use cpu::{Cpu, Register16, Register8};
use memory::{Memory, Ram, Read, Write};

pub struct ZexHarness {
    cpu: Cpu,
    memory: Ram,
    bus: DummyBus,
}

impl ZexHarness {
    pub fn new(rom_path: &str) -> ZexHarness {
        let mut memory = Ram::from_file(0x10000, rom_path, 0x100);
        memory.write_byte(0x0005, 0xc9); // patch with RET instruction
        memory.write_word(0x0006, 0xe400); // patch with initial SP

        ZexHarness {
            cpu: Cpu::new(0x100),
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
                                    print!("{}", character);
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

pub trait System<'de>: Serialize + Deserialize<'de> {
    fn new() -> Self;
    fn emulate(&mut self, video: &mut impl VideoSink, audio: &mut impl AudioSink) -> u8;
    fn activate_debugger(&mut self);
    fn set_key(&mut self, line: usize, bit: u8);
    fn unset_key(&mut self, line: usize, bit: u8);
    fn load_disk(&mut self, drive: usize, rom: Vec<u8>, path: PathBuf);
    fn disassemble(&mut self, count: usize) -> Vec<DisassembledInstruction>;
}

#[derive(Serialize, Deserialize)]
pub struct DisassembledInstruction {
    address: u16,
    instruction: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CPC464 {
    cpu: Cpu,
    memory: Memory,
    #[serde(flatten)]
    bus: StandardBus,
    // debugger: Debugger,
}

impl System<'_> for CPC464 {
    fn new() -> CPC464 {
        let cpu = Cpu::new(0);
        let memory = Memory::new();
        let bus = StandardBus::new();
        // let debugger = Debugger::new();

        CPC464 {
            cpu,
            memory,
            bus,
            // debugger,
        }
    }

    fn emulate(&mut self, video: &mut impl VideoSink, audio: &mut impl AudioSink) -> u8 {
        // if self.debugger.is_active(&self.cpu) {
        //     self.debugger.run_command_shell(&mut self.cpu, &self.memory);
        // }

        let (cycles, interrupt_acknowledged) =
            self.cpu.fetch_and_execute(&mut self.memory, &mut self.bus);

        for _ in 0..cycles {
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

    fn activate_debugger(&mut self) {
        // self.debugger.activate();
    }

    fn set_key(&mut self, line: usize, bit: u8) {
        self.bus.set_key(line, bit);
    }

    fn unset_key(&mut self, line: usize, bit: u8) {
        self.bus.unset_key(line, bit);
    }

    fn load_disk(&mut self, drive: usize, rom: Vec<u8>, path: PathBuf) {
        // TODO: allow loading tapes as well
        self.bus.load_disk(drive, rom, path);
    }

    fn disassemble(&mut self, count: usize) -> Vec<DisassembledInstruction> {
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
