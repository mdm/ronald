use std::collections::HashMap;

use crate::system::{clock::MasterClockTick, instruction::InterruptMode};

pub struct SystemDebugView {
    pub master_clock: MasterClockTick,
    pub cpu: CpuDebugView,
    pub memory: MemoryDebugView,
}

pub struct DisassembledInstruction {
    pub address: u16,
    pub instruction: String,
    pub length: usize,
}

pub struct CpuDebugView {
    pub register_a: u8,
    pub register_f: u8,
    pub register_b: u8,
    pub register_c: u8,
    pub register_d: u8,
    pub register_e: u8,
    pub register_h: u8,
    pub register_l: u8,
    pub shadow_register_a: u8,
    pub shadow_register_f: u8,
    pub shadow_register_b: u8,
    pub shadow_register_c: u8,
    pub shadow_register_d: u8,
    pub shadow_register_e: u8,
    pub shadow_register_h: u8,
    pub shadow_register_l: u8,
    pub register_i: u8,
    pub register_r: u8,
    pub register_ixh: u8,
    pub register_ixl: u8,
    pub register_iyh: u8,
    pub register_iyl: u8,
    pub register_sp: u16,
    pub register_pc: u16,
    pub iff1: bool,
    pub iff2: bool,
    pub halted: bool,
    pub interrupt_mode: InterruptMode,
    pub enable_interrupt: bool,
    pub irq_received: bool,
}

pub struct MemoryDebugView {
    pub ram: Vec<u8>,
    pub ram_extension: Vec<u8>,
    pub lower_rom: Vec<u8>,
    pub lower_rom_enabled: bool,
    pub upper_roms: HashMap<u8, Vec<u8>>,
    pub selected_upper_rom: u8,
    pub upper_rom_enabled: bool,
    pub composite_rom_ram: Vec<u8>,
    pub composite_ram: Vec<u8>,
}
