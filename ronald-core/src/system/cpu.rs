use std::fmt;

use serde::{Deserialize, Serialize};

use crate::debug::event::CpuDebugEvent;
use crate::debug::view::CpuDebugView;
use crate::debug::{DebugSource, Debuggable, Snapshotable};
use crate::system::bus::Bus;
use crate::system::instruction::{Decoder, Instruction, InterruptMode, JumpTest, Operand};
use crate::system::memory::{MemManage, MemRead, MemWrite};

#[allow(clippy::upper_case_acronyms)] // Registers are names as in the CPU manual
pub enum Register8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
    I,
    R,
    IXH,
    IXL,
    IYH,
    IYL,
}

pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
    IX,
    IY,
    SP,
    PC,
}

#[derive(Clone, Serialize, Deserialize)]
struct RegisterFile {
    #[serde(rename = "registers")]
    data: Vec<u16>,
}

impl RegisterFile {
    fn new() -> RegisterFile {
        RegisterFile { data: vec![0; 14] }
    }

    pub fn read_byte(&self, register: &Register8) -> u8 {
        let value = match register {
            Register8::A => self.data[0] >> 8,
            Register8::F => self.data[0] & 0xff,
            Register8::B => self.data[1] >> 8,
            Register8::C => self.data[1] & 0xff,
            Register8::D => self.data[2] >> 8,
            Register8::E => self.data[2] & 0xff,
            Register8::H => self.data[3] >> 8,
            Register8::L => self.data[3] & 0xff,
            Register8::I => self.data[8] & 0xff,
            Register8::R => self.data[9] & 0xff,
            Register8::IXH => self.data[10] >> 8,
            Register8::IXL => self.data[10] & 0xff,
            Register8::IYH => self.data[11] >> 8,
            Register8::IYL => self.data[11] & 0xff,
        };

        value as u8
    }

    fn write_byte(&mut self, register: &Register8, value: u8) {
        // TODO: add tests
        let value = value as u16;

        match register {
            Register8::A => {
                let was = self.data[0];
                self.data[0] = (value << 8) + (self.data[0] & 0xff);
                self.emit_debug_event(CpuDebugEvent::Register8Changed {
                    register: Register8::A,
                    is: value as u8,
                    was: (was >> 8) as u8,
                });
                self.emit_debug_event(CpuDebugEvent::Register16Changed {
                    register: Register16::AF,
                    is: self.data[0],
                    was,
                });
            }
            Register8::F => {
                let was = self.data[0];
                self.data[0] = (self.data[0] & 0xff00) + value;
                self.emit_debug_event(CpuDebugEvent::Register8Changed {
                    register: Register8::F,
                    is: value as u8,
                    was: (was & 0xff) as u8,
                });
                self.emit_debug_event(CpuDebugEvent::Register16Changed {
                    register: Register16::AF,
                    is: self.data[0],
                    was,
                });
            }
            Register8::B => {
                let was = self.data[1];
                self.data[1] = (value << 8) + (self.data[1] & 0xff);
                self.emit_debug_event(CpuDebugEvent::Register8Changed {
                    register: Register8::B,
                    is: value as u8,
                    was: (was >> 8) as u8,
                });
                self.emit_debug_event(CpuDebugEvent::Register16Changed {
                    register: Register16::BC,
                    is: self.data[1],
                    was,
                });
            }
            Register8::C => {
                let was = self.data[1];
                self.data[1] = (self.data[1] & 0xff00) + value;
                self.emit_debug_event(CpuDebugEvent::Register8Changed {
                    register: Register8::C,
                    is: value as u8,
                    was: (was & 0xff) as u8,
                });
                self.emit_debug_event(CpuDebugEvent::Register16Changed {
                    register: Register16::BC,
                    is: self.data[1],
                    was,
                });
            }
            Register8::D => {
                let was = self.data[2];
                self.data[2] = (value << 8) + (self.data[2] & 0xff);
                self.emit_debug_event(CpuDebugEvent::Register8Changed {
                    register: Register8::D,
                    is: value as u8,
                    was: (was >> 8) as u8,
                });
                self.emit_debug_event(CpuDebugEvent::Register16Changed {
                    register: Register16::DE,
                    is: self.data[2],
                    was,
                });
            }
            Register8::E => {
                let was = self.data[2];
                self.data[2] = (self.data[2] & 0xff00) + value;
                self.emit_debug_event(CpuDebugEvent::Register8Changed {
                    register: Register8::E,
                    is: value as u8,
                    was: (was & 0xff) as u8,
                });
                self.emit_debug_event(CpuDebugEvent::Register16Changed {
                    register: Register16::DE,
                    is: self.data[2],
                    was,
                });
            }
            Register8::H => {
                let was = self.data[3];
                self.data[3] = (value << 8) + (self.data[3] & 0xff);
                self.emit_debug_event(CpuDebugEvent::Register8Changed {
                    register: Register8::H,
                    is: value as u8,
                    was: (was >> 8) as u8,
                });
                self.emit_debug_event(CpuDebugEvent::Register16Changed {
                    register: Register16::HL,
                    is: self.data[3],
                    was,
                });
            }
            Register8::L => {
                let was = self.data[3];
                self.data[3] = (self.data[3] & 0xff00) + value;
                self.emit_debug_event(CpuDebugEvent::Register8Changed {
                    register: Register8::L,
                    is: value as u8,
                    was: (was & 0xff) as u8,
                });
                self.emit_debug_event(CpuDebugEvent::Register16Changed {
                    register: Register16::HL,
                    is: self.data[3],
                    was,
                });
            }
            Register8::I => {
                let was = self.data[8];
                self.data[8] = (self.data[8] & 0xff00) + value;
                self.emit_debug_event(CpuDebugEvent::Register8Changed {
                    register: Register8::I,
                    is: value as u8,
                    was: (was & 0xff) as u8,
                });
            }
            Register8::R => {
                let was = self.data[9];
                self.data[9] = (self.data[9] & 0xff00) + value;
                self.emit_debug_event(CpuDebugEvent::Register8Changed {
                    register: Register8::R,
                    is: value as u8,
                    was: (was & 0xff) as u8,
                });
            }
            Register8::IXH => {
                let was = self.data[10];
                self.data[10] = (value << 8) + (self.data[10] & 0xff);
                self.emit_debug_event(CpuDebugEvent::Register8Changed {
                    register: Register8::IXH,
                    is: value as u8,
                    was: (was >> 8) as u8,
                });
                self.emit_debug_event(CpuDebugEvent::Register16Changed {
                    register: Register16::IX,
                    is: self.data[10],
                    was,
                });
            }
            Register8::IXL => {
                let was = self.data[10];
                self.data[10] = (self.data[10] & 0xff00) + value;
                self.emit_debug_event(CpuDebugEvent::Register8Changed {
                    register: Register8::IXL,
                    is: value as u8,
                    was: (was & 0xff) as u8,
                });
                self.emit_debug_event(CpuDebugEvent::Register16Changed {
                    register: Register16::IX,
                    is: self.data[10],
                    was,
                });
            }
            Register8::IYH => {
                let was = self.data[11];
                self.data[11] = (value << 8) + (self.data[11] & 0xff);
                self.emit_debug_event(CpuDebugEvent::Register8Changed {
                    register: Register8::IYH,
                    is: value as u8,
                    was: (was >> 8) as u8,
                });
                self.emit_debug_event(CpuDebugEvent::Register16Changed {
                    register: Register16::IY,
                    is: self.data[11],
                    was,
                });
            }
            Register8::IYL => {
                let was = self.data[11];
                self.data[11] = (self.data[11] & 0xff00) + value;
                self.emit_debug_event(CpuDebugEvent::Register8Changed {
                    register: Register8::IYL,
                    is: value as u8,
                    was: (was & 0xff) as u8,
                });
                self.emit_debug_event(CpuDebugEvent::Register16Changed {
                    register: Register16::IY,
                    is: self.data[11],
                    was,
                });
            }
        }
    }

    pub fn read_word(&self, register: &Register16) -> u16 {
        match register {
            Register16::AF => self.data[0],
            Register16::BC => self.data[1],
            Register16::DE => self.data[2],
            Register16::HL => self.data[3],
            Register16::IX => self.data[10],
            Register16::IY => self.data[11],
            Register16::SP => self.data[12],
            Register16::PC => self.data[13],
        }
    }

    fn write_word(&mut self, register: &Register16, value: u16) {
        match register {
            Register16::AF => {
                let was = self.data[0];
                self.data[0] = value;
                self.emit_debug_event(CpuDebugEvent::Register16Changed {
                    register: Register16::AF,
                    is: self.data[0],
                    was,
                });
            }
            Register16::BC => {
                let was = self.data[1];
                self.data[1] = value;
                self.emit_debug_event(CpuDebugEvent::Register16Changed {
                    register: Register16::BC,
                    is: self.data[1],
                    was,
                });
            }
            Register16::DE => {
                let was = self.data[2];
                self.data[2] = value;
                self.emit_debug_event(CpuDebugEvent::Register16Changed {
                    register: Register16::DE,
                    is: self.data[2],
                    was,
                });
            }
            Register16::HL => {
                let was = self.data[3];
                self.data[3] = value;
                self.emit_debug_event(CpuDebugEvent::Register16Changed {
                    register: Register16::HL,
                    is: self.data[3],
                    was,
                });
            }
            Register16::IX => {
                let was = self.data[10];
                self.data[10] = value;
                self.emit_debug_event(CpuDebugEvent::Register16Changed {
                    register: Register16::IX,
                    is: self.data[10],
                    was,
                });
            }
            Register16::IY => {
                let was = self.data[11];
                self.data[11] = value;
                self.emit_debug_event(CpuDebugEvent::Register16Changed {
                    register: Register16::IY,
                    is: self.data[11],
                    was,
                });
            }
            Register16::SP => {
                let was = self.data[12];
                self.data[12] = value;
                self.emit_debug_event(CpuDebugEvent::Register16Changed {
                    register: Register16::SP,
                    is: self.data[12],
                    was,
                });
            }
            Register16::PC => {
                let was = self.data[13];
                self.data[13] = value;
                self.emit_debug_event(CpuDebugEvent::Register16Changed {
                    register: Register16::PC,
                    is: self.data[13],
                    was,
                });
            }
        }
    }

    fn swap_word(&mut self, register: &Register16) {
        match register {
            Register16::AF => {
                self.data.swap(0, 4);
                self.emit_debug_event(CpuDebugEvent::ShadowRegister16Swapped {
                    register: Register16::AF,
                    is: self.data[4],
                    was: self.data[0],
                });
            }
            Register16::BC => {
                self.data.swap(1, 5);
                self.emit_debug_event(CpuDebugEvent::ShadowRegister16Swapped {
                    register: Register16::AF,
                    is: self.data[5],
                    was: self.data[1],
                });
            }
            Register16::DE => {
                self.data.swap(2, 6);
                self.emit_debug_event(CpuDebugEvent::ShadowRegister16Swapped {
                    register: Register16::AF,
                    is: self.data[6],
                    was: self.data[2],
                });
            }
            Register16::HL => {
                self.data.swap(3, 7);
                self.emit_debug_event(CpuDebugEvent::ShadowRegister16Swapped {
                    register: Register16::AF,
                    is: self.data[7],
                    was: self.data[3],
                });
            }
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for RegisterFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AF: {:#06x}    ", self.data[0])?;
        writeln!(f, "AF': {:#06x}", self.data[4])?;
        write!(f, "BC: {:#06x}    ", self.data[1])?;
        writeln!(f, "BC': {:#06x}", self.data[5])?;
        write!(f, "DE: {:#06x}    ", self.data[2])?;
        writeln!(f, "DE': {:#06x}", self.data[6])?;
        write!(f, "HL: {:#06x}    ", self.data[3])?;
        writeln!(f, "HL': {:#06x}", self.data[7])?;
        writeln!(f, " I: {:#04x}", self.data[8] & 0xff)?;
        writeln!(f, " R: {:#04x}", self.data[9] & 0xff)?;
        writeln!(f, "IX: {:#06x}", self.data[10])?;
        writeln!(f, "IY: {:#06x}", self.data[11])?;
        writeln!(f, "SP: {:#06x}", self.data[12])?;
        writeln!(f, "PC: {:#06x}", self.data[13])
    }
}

struct RegisterFileDebugView {
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
}

impl Snapshotable for RegisterFile {
    type View = RegisterFileDebugView;

    fn debug_view(&self) -> Self::View {
        let mut registers = self.clone();
        let register_a = registers.read_byte(&Register8::A);
        let register_f = registers.read_byte(&Register8::F);
        let register_b = registers.read_byte(&Register8::B);
        let register_c = registers.read_byte(&Register8::C);
        let register_d = registers.read_byte(&Register8::D);
        let register_e = registers.read_byte(&Register8::E);
        let register_h = registers.read_byte(&Register8::H);
        let register_l = registers.read_byte(&Register8::L);
        registers.swap_word(&Register16::AF);
        registers.swap_word(&Register16::BC);
        registers.swap_word(&Register16::DE);
        registers.swap_word(&Register16::HL);
        let shadow_register_a = registers.read_byte(&Register8::A);
        let shadow_register_f = registers.read_byte(&Register8::F);
        let shadow_register_b = registers.read_byte(&Register8::B);
        let shadow_register_c = registers.read_byte(&Register8::C);
        let shadow_register_d = registers.read_byte(&Register8::D);
        let shadow_register_e = registers.read_byte(&Register8::E);
        let shadow_register_h = registers.read_byte(&Register8::H);
        let shadow_register_l = registers.read_byte(&Register8::L);
        let register_i = registers.read_byte(&Register8::I);
        let register_r = registers.read_byte(&Register8::R);
        let register_ixh = registers.read_byte(&Register8::IXH);
        let register_ixl = registers.read_byte(&Register8::IXL);
        let register_iyh = registers.read_byte(&Register8::IYH);
        let register_iyl = registers.read_byte(&Register8::IYL);
        let register_sp = registers.read_word(&Register16::SP);
        let register_pc = registers.read_word(&Register16::PC);

        Self::View {
            register_a,
            register_f,
            register_b,
            register_c,
            register_d,
            register_e,
            register_h,
            register_l,
            shadow_register_a,
            shadow_register_f,
            shadow_register_b,
            shadow_register_c,
            shadow_register_d,
            shadow_register_e,
            shadow_register_h,
            shadow_register_l,
            register_i,
            register_r,
            register_ixh,
            register_ixl,
            register_iyh,
            register_iyl,
            register_sp,
            register_pc,
        }
    }
}

impl Debuggable for RegisterFile {
    const SOURCE: DebugSource = DebugSource::Cpu;
    type Event = CpuDebugEvent;
}

enum Flag {
    Carry,
    AddSubtract,
    ParityOverflow, // even parity makes this true
    HalfCarry,
    Zero,
    Sign, // negative values make this true
}

impl Flag {
    fn mask(&self) -> u8 {
        match self {
            Flag::Carry => 1 << 0,
            Flag::AddSubtract => 1 << 1,
            Flag::ParityOverflow => 1 << 2,
            Flag::HalfCarry => 1 << 4,
            Flag::Zero => 1 << 6,
            Flag::Sign => 1 << 7,
        }
    }
}

pub trait Cpu: Default {
    fn fetch_and_execute(
        &mut self,
        memory: &mut (impl MemRead + MemWrite + MemManage),
        bus: &mut impl Bus,
    ) -> (u8, bool);
    fn request_interrupt(&mut self);
    fn disassemble(
        &mut self,
        memory: &mut (impl MemRead + MemWrite + MemManage),
        count: usize,
    ) -> Vec<(u16, String)>;
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZilogZ80<D>
where
    D: Decoder,
{
    #[serde(flatten)]
    registers: RegisterFile, // TODO: make this private, public because of ZexHarness reads it, use debug view instead?
    decoder: D, // TODO: make this private
    iff1: bool,
    iff2: bool,
    halted: bool,
    interrupt_mode: InterruptMode,
    enable_interrupt: bool,
    irq_received: bool,
}

impl<D> Default for ZilogZ80<D>
where
    D: Decoder,
{
    fn default() -> Self {
        Self::new(0)
    }
}

impl<D> ZilogZ80<D>
where
    D: Decoder,
{
    pub fn new(initial_pc: u16) -> Self {
        let mut registers = RegisterFile::new();
        registers.write_word(&Register16::PC, initial_pc);

        let mut cpu = Self {
            registers,
            iff1: false,
            iff2: false,
            halted: false,
            decoder: D::default(),
            interrupt_mode: InterruptMode::default(),
            enable_interrupt: false,
            irq_received: false,
        };

        cpu.reset();

        cpu
    }

    fn reset(&mut self) {
        // TODO: implement reset
    }

    fn check_flag(&self, flag: Flag) -> bool {
        // TODO: move this from CPU to register file
        let flags = self.registers.read_byte(&Register8::F);

        flags & flag.mask() != 0
    }

    fn set_flag(&mut self, flag: Flag, value: bool) {
        // TODO: move this from CPU to register file
        let old_flags = self.registers.read_byte(&Register8::F);

        let new_flags = if value {
            old_flags | flag.mask()
        } else {
            old_flags & (!flag.mask()) // TODO: add tests
        };

        self.registers.write_byte(&Register8::F, new_flags);
    }

    fn check_jump(&self, jump_test: &JumpTest) -> bool {
        match jump_test {
            JumpTest::Unconditional => true,
            JumpTest::NonZero => !self.check_flag(Flag::Zero),
            JumpTest::Zero => self.check_flag(Flag::Zero),
            JumpTest::NoCarry => !self.check_flag(Flag::Carry),
            JumpTest::Carry => self.check_flag(Flag::Carry),
            JumpTest::ParityOdd => !self.check_flag(Flag::ParityOverflow),
            JumpTest::ParityEven => self.check_flag(Flag::ParityOverflow),
            JumpTest::SignPositive => !self.check_flag(Flag::Sign),
            JumpTest::SignNegative => self.check_flag(Flag::Sign),
        }
    }

    fn load_byte(&self, memory: &impl MemRead, operand: &Operand) -> u8 {
        match operand {
            Operand::Immediate8(value) => *value,
            Operand::Register8(register) => self.registers.read_byte(register),
            Operand::Direct8(_) => unimplemented!(),
            Operand::Direct16(address) => memory.read_byte(*address as usize),
            Operand::RegisterIndirect(register) => {
                let address = self.registers.read_word(register);
                memory.read_byte(address as usize)
            }
            Operand::Indexed(register, displacement) => {
                let address = self.registers.read_word(register);
                memory.read_byte((address as i64 + *displacement as i64) as usize)
            }
            _ => unreachable!(),
        }
    }

    fn store_byte(&mut self, memory: &mut impl MemWrite, operand: &Operand, value: u8) {
        match operand {
            Operand::Register8(register) => self.registers.write_byte(register, value),
            Operand::Direct8(_) => unimplemented!(),
            Operand::Direct16(address) => memory.write_byte(*address as usize, value),
            Operand::RegisterIndirect(register) => {
                let address = self.registers.read_word(register);
                memory.write_byte(address as usize, value)
            }
            Operand::Indexed(register, displacement) => {
                let address = self.registers.read_word(register);
                memory.write_byte((address as i64 + *displacement as i64) as usize, value)
            }
            _ => unreachable!(),
        }
    }

    fn load_word(&self, memory: &impl MemRead, operand: &Operand) -> u16 {
        match operand {
            Operand::Immediate16(value) => *value,
            Operand::Register16(register) => self.registers.read_word(register),
            Operand::Direct16(address) => memory.read_word(*address as usize),
            Operand::RegisterIndirect(register) => {
                let address = self.registers.read_word(register);
                memory.read_word(address as usize)
            }
            Operand::Indexed(register, displacement) => {
                let address = self.registers.read_word(register);
                memory.read_word((address as i64 + *displacement as i64) as usize)
            }
            _ => unreachable!(),
        }
    }

    fn store_word(&mut self, memory: &mut impl MemWrite, operand: &Operand, value: u16) {
        match operand {
            Operand::Register16(register) => self.registers.write_word(register, value),
            Operand::Direct16(address) => memory.write_word(*address as usize, value),
            Operand::RegisterIndirect(register) => {
                let address = self.registers.read_word(register);
                memory.write_word(address as usize, value)
            }
            Operand::Indexed(register, displacement) => {
                let address = self.registers.read_word(register);
                memory.write_word((address as i64 + *displacement as i64) as usize, value)
            }
            _ => unreachable!(),
        }
    }

    fn handle_interrupt(&mut self, memory: &mut (impl MemRead + MemWrite + MemManage)) -> bool {
        if self.irq_received && self.iff1 {
            // TODO: allow non-maskable interrupts (they are not used in the CPC)?
            self.halted = false;
            self.irq_received = false; // TODO: make requester hold interrupt until acknowledged?

            match self.interrupt_mode {
                InterruptMode::Mode1 => {
                    log::trace!("Handling interrupt");
                    let old_pc = self.registers.read_word(&Register16::PC); // PC has already been set to next instruction
                    let new_sp = self.registers.read_word(&Register16::SP) - 2;
                    self.registers.write_word(&Register16::SP, new_sp);
                    memory.write_word(new_sp as usize, old_pc);

                    self.registers.write_word(&Register16::PC, 0x0038);

                    // timing_in_nops += 4; // + Instruction::Rst(_).timing()
                }
                _ => unimplemented!(),
            }

            true
        } else {
            false
        }
    }
}

impl<D> Cpu for ZilogZ80<D>
where
    D: Decoder,
{
    fn fetch_and_execute(
        &mut self,
        memory: &mut (impl MemRead + MemWrite + MemManage),
        bus: &mut impl Bus,
    ) -> (u8, bool) {
        if self.halted {
            if self.handle_interrupt(memory) {
                return (4, true);
            }

            return (1, false);
        }

        if self.enable_interrupt {
            self.iff1 = true;
            self.iff2 = true;
            self.enable_interrupt = false;
        }

        let mut prevent_interrupt = false;

        let pc = self.registers.read_word(&Register16::PC);

        let (instruction, next_address) = self.decoder.decode(memory, pc as usize);

        log::trace!("{:#06x}: {}", pc, &instruction);

        let mut timing_in_nops = instruction.timing();

        match &instruction {
            Instruction::Adc(destination, source) => {
                match destination {
                    Operand::Register8(Register8::A) => {
                        let left = self.load_byte(memory, destination);
                        let right = self.load_byte(memory, source);
                        let carry_value = if self.check_flag(Flag::Carry) { 1 } else { 0 };
                        let (value, carry1) = right.overflowing_add(carry_value);
                        let (value, carry2) = left.overflowing_add(value);
                        let overflow =
                            (left & 0x80) == (right & 0x80) && (right & 0x80) != (value & 0x80);
                        self.store_byte(memory, destination, value);

                        self.set_flag(Flag::Sign, (value as i8) < 0);
                        self.set_flag(Flag::Zero, value == 0);
                        self.set_flag(
                            Flag::HalfCarry,
                            (((left & 0xf) + (right & 0xf) + carry_value) & 0x10) != 0,
                        );
                        self.set_flag(Flag::ParityOverflow, overflow);
                        self.set_flag(Flag::AddSubtract, false);
                        self.set_flag(Flag::Carry, carry1 || carry2);
                    }
                    Operand::Register16(Register16::HL) => {
                        let left = self.load_word(memory, destination);
                        let right = self.load_word(memory, source);
                        let carry_value = if self.check_flag(Flag::Carry) { 1 } else { 0 };
                        let (value, carry1) = right.overflowing_add(carry_value);
                        let (value, carry2) = left.overflowing_add(value);
                        let overflow = (left & 0x8000) == (right & 0x8000)
                            && (right & 0x8000) != (value & 0x8000);
                        self.store_word(memory, destination, value);

                        self.set_flag(Flag::Sign, (value as i16) < 0);
                        self.set_flag(Flag::Zero, value == 0);
                        self.set_flag(
                            Flag::HalfCarry,
                            (((left & 0xfff) + (right & 0xfff) + carry_value) & 0x1000) != 0,
                        );
                        self.set_flag(Flag::ParityOverflow, overflow);
                        self.set_flag(Flag::AddSubtract, false);
                        self.set_flag(Flag::Carry, carry1 || carry2);
                    }
                    _ => unreachable!(),
                }

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Add(destination, source) => {
                match destination {
                    Operand::Register8(Register8::A) => {
                        // ALUOP / checked
                        let left = self.load_byte(memory, destination);
                        let right = self.load_byte(memory, source);
                        let (value, carry) = left.overflowing_add(right);
                        let (_, overflow) = (left as i8).overflowing_add(right as i8);
                        self.store_byte(memory, destination, value);

                        self.set_flag(Flag::Sign, (value as i8) < 0); // TODO: make this reusable?
                        self.set_flag(Flag::Zero, value == 0);
                        self.set_flag(
                            Flag::HalfCarry,
                            (((left & 0xf) + (right & 0xf)) & 0x10) != 0,
                        );
                        self.set_flag(Flag::ParityOverflow, overflow);
                        self.set_flag(Flag::AddSubtract, false);
                        self.set_flag(Flag::Carry, carry);
                    }
                    _ => {
                        let left = self.load_word(memory, destination);
                        let right = self.load_word(memory, source);
                        let (value, carry) = left.overflowing_add(right);
                        self.store_word(memory, destination, value);

                        self.set_flag(
                            Flag::HalfCarry,
                            (((left & 0xfff) + (right & 0xfff)) & 0x1000) != 0,
                        );
                        self.set_flag(Flag::AddSubtract, false);
                        self.set_flag(Flag::Carry, carry);
                    }
                }

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::And(operand) => {
                let value = self.load_byte(memory, operand);

                let result = self.registers.read_byte(&Register8::A) & value;
                self.registers.write_byte(&Register8::A, result);

                self.set_flag(Flag::Sign, (result as i8) < 0);
                self.set_flag(Flag::Zero, result == 0);
                self.set_flag(Flag::HalfCarry, true);
                self.set_flag(Flag::ParityOverflow, (result.count_ones() & 1) == 0);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, false);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Bit(Operand::Bit(bit), operand) => {
                let value = self.load_byte(memory, operand);

                self.set_flag(Flag::Zero, (value & (1 << bit)) == 0);
                self.set_flag(Flag::HalfCarry, true);
                self.set_flag(Flag::AddSubtract, false);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Bit(_, _) => {
                unreachable!();
            }
            Instruction::Call(jump_test, Operand::Immediate16(address)) => {
                if self.check_jump(jump_test) {
                    let new_sp = self.registers.read_word(&Register16::SP) - 2;
                    self.registers.write_word(&Register16::SP, new_sp);
                    memory.write_word(new_sp as usize, next_address as u16);
                    self.registers.write_word(&Register16::PC, *address);
                } else {
                    self.registers
                        .write_word(&Register16::PC, next_address as u16);
                }
            }
            Instruction::Call(_, _) => {
                unreachable!();
            }
            Instruction::Ccf => {
                let carry = self.check_flag(Flag::Carry);

                self.set_flag(Flag::HalfCarry, carry);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, !carry);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Cp(operand) => {
                // ALUOP / checked
                let left = self.registers.read_byte(&Register8::A);
                let right = self.load_byte(memory, operand);
                let (value, carry) = left.overflowing_sub(right);
                let (_, overflow) = (left as i8).overflowing_sub(right as i8);

                self.set_flag(Flag::Sign, (value as i8) < 0); // TODO: make this reusable?
                self.set_flag(Flag::Zero, value == 0);
                self.set_flag(Flag::HalfCarry, (left & 0xf) < (right & 0xf));
                self.set_flag(Flag::ParityOverflow, overflow);
                self.set_flag(Flag::AddSubtract, true);
                self.set_flag(Flag::Carry, carry);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Cpd => {
                let left = self.registers.read_byte(&Register8::A);
                let right = self.load_byte(memory, &Operand::RegisterIndirect(Register16::HL));
                let value = left.wrapping_sub(right);

                let source = self.registers.read_word(&Register16::HL).wrapping_sub(1);
                self.registers.write_word(&Register16::HL, source);

                let counter = self.registers.read_word(&Register16::BC).wrapping_sub(1);
                self.registers.write_word(&Register16::BC, counter);

                self.set_flag(Flag::Sign, (value as i8) < 0);
                self.set_flag(Flag::Zero, value == 0);
                self.set_flag(Flag::HalfCarry, (left & 0xf) < (right & 0xf));
                self.set_flag(Flag::ParityOverflow, counter != 0);
                self.set_flag(Flag::AddSubtract, true);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Cpdr => {
                let left = self.registers.read_byte(&Register8::A);
                let right = self.load_byte(memory, &Operand::RegisterIndirect(Register16::HL));
                let value = left.wrapping_sub(right);

                let source = self.registers.read_word(&Register16::HL).wrapping_sub(1);
                self.registers.write_word(&Register16::HL, source);

                let counter = self.registers.read_word(&Register16::BC).wrapping_sub(1);
                self.registers.write_word(&Register16::BC, counter);

                self.set_flag(Flag::Sign, (value as i8) < 0);
                self.set_flag(Flag::Zero, value == 0);
                self.set_flag(Flag::HalfCarry, (left & 0xf) < (right & 0xf));
                self.set_flag(Flag::ParityOverflow, counter != 0);
                self.set_flag(Flag::AddSubtract, true);

                if counter == 0 || value == 0 {
                    self.registers
                        .write_word(&Register16::PC, next_address as u16);
                    timing_in_nops = 5; // not having to adjust the PC saves time
                }
            }
            Instruction::Cpi => {
                let left = self.registers.read_byte(&Register8::A);
                let right = self.load_byte(memory, &Operand::RegisterIndirect(Register16::HL));
                let value = left.wrapping_sub(right);

                let source = self.registers.read_word(&Register16::HL).wrapping_add(1);
                self.registers.write_word(&Register16::HL, source);

                let counter = self.registers.read_word(&Register16::BC).wrapping_sub(1);
                self.registers.write_word(&Register16::BC, counter);

                self.set_flag(Flag::Sign, (value as i8) < 0);
                self.set_flag(Flag::Zero, value == 0);
                self.set_flag(Flag::HalfCarry, (left & 0xf) < (right & 0xf));
                self.set_flag(Flag::ParityOverflow, counter != 0);
                self.set_flag(Flag::AddSubtract, true);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Cpir => {
                let left = self.registers.read_byte(&Register8::A);
                let right = self.load_byte(memory, &Operand::RegisterIndirect(Register16::HL));
                let value = left.wrapping_sub(right);

                let source = self.registers.read_word(&Register16::HL).wrapping_add(1);
                self.registers.write_word(&Register16::HL, source);

                let counter = self.registers.read_word(&Register16::BC).wrapping_sub(1);
                self.registers.write_word(&Register16::BC, counter);

                self.set_flag(Flag::Sign, (value as i8) < 0);
                self.set_flag(Flag::Zero, value == 0);
                self.set_flag(Flag::HalfCarry, (left & 0xf) < (right & 0xf));
                self.set_flag(Flag::ParityOverflow, counter != 0);
                self.set_flag(Flag::AddSubtract, true);

                if counter == 0 || value == 0 {
                    self.registers
                        .write_word(&Register16::PC, next_address as u16);
                    timing_in_nops = 5; // not having to adjust the PC saves time
                }
            }
            Instruction::Cpl => {
                let value = self.registers.read_byte(&Register8::A);
                self.registers.write_byte(&Register8::A, !value);

                self.set_flag(Flag::HalfCarry, true);
                self.set_flag(Flag::AddSubtract, true);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Daa => {
                let mut value = self.registers.read_byte(&Register8::A);
                let mut half_carry = self.check_flag(Flag::HalfCarry);
                let mut carry = self.check_flag(Flag::Carry);
                let mut correction = 0;

                if (value & 0xf) > 0x9 || half_carry {
                    half_carry = half_carry && (value & 0xf) < 0x6;
                    correction += 0x6;
                }

                if value > 0x99 || carry {
                    correction += 0x60;
                    carry = true;
                }

                if self.check_flag(Flag::AddSubtract) {
                    half_carry = half_carry && (value & 0xf) < 0x6;
                    value -= correction;
                } else {
                    half_carry = (value & 0xf) > 0x9;
                    value += correction;
                }

                self.registers.write_byte(&Register8::A, value);

                self.set_flag(Flag::Sign, (value as i8) < 0);
                self.set_flag(Flag::Zero, value == 0);
                self.set_flag(Flag::HalfCarry, half_carry);
                self.set_flag(Flag::ParityOverflow, (value.count_ones() & 1) == 0);
                self.set_flag(Flag::Carry, carry);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Dec(destination) => {
                match destination {
                    Operand::Register16(register) => {
                        let value = self.registers.read_word(register);
                        self.registers.write_word(register, value.wrapping_sub(1));
                    }
                    _ => {
                        let old_value = self.load_byte(memory, destination);
                        self.set_flag(Flag::ParityOverflow, old_value == 0x80);
                        let (value, _) = old_value.overflowing_sub(1);
                        self.store_byte(memory, destination, value);

                        self.set_flag(Flag::Sign, (value as i8) < 0);
                        self.set_flag(Flag::Zero, value == 0);
                        self.set_flag(Flag::HalfCarry, (old_value & 0xf) < 1);
                        self.set_flag(Flag::AddSubtract, true);
                    }
                }

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Defb(_operand) => {
                prevent_interrupt = true;
                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Defw(_operand) => {
                // TODO: disable interrupts??? or not???
                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Di => {
                self.iff1 = false;
                self.iff2 = false;

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Djnz(target) => {
                let value = self.registers.read_byte(&Register8::B).wrapping_sub(1);
                self.registers.write_byte(&Register8::B, value);

                if value != 0 {
                    let address = match target {
                        Operand::Immediate16(address) => address,
                        _ => {
                            unimplemented!();
                        }
                    };
                    self.registers.write_word(&Register16::PC, *address);
                    timing_in_nops = 4;
                } else {
                    self.registers
                        .write_word(&Register16::PC, next_address as u16);
                }
            }
            Instruction::Ei => {
                self.enable_interrupt = true;

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Ex(left, right) => {
                match (left, right) {
                    (Operand::Register16(Register16::AF), Operand::Register16(Register16::AF)) => {
                        // TODO: test if this match arm works with references
                        self.registers.swap_word(&Register16::AF);
                    }
                    (left, right) => {
                        let left_value = self.load_word(memory, left);
                        let right_value = self.load_word(memory, right);

                        self.store_word(memory, left, right_value);
                        self.store_word(memory, right, left_value);
                    }
                }

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Exx => {
                self.registers.swap_word(&Register16::BC);
                self.registers.swap_word(&Register16::DE);
                self.registers.swap_word(&Register16::HL);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Halt => {
                self.halted = true;

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Im(mode) => {
                self.interrupt_mode = *mode;

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::In(Operand::Register8(Register8::A), Operand::Direct8(port_low)) => {
                let port_high = self.registers.read_byte(&Register8::A);
                let port = (port_high as u16) << 8 | (*port_low as u16);

                let value = bus.read_byte(port);
                self.registers.write_byte(&Register8::A, value);

                // TODO: do we need to set flags?

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::In(
                Operand::Register8(destination),
                Operand::RegisterIndirect(Register16::BC),
            ) => {
                // TODO: make this a special case of the other IN instruction above?
                let port = self.registers.read_word(&Register16::BC);

                let value = bus.read_byte(port);
                // TODO: can this write affect the flags register in unintended ways?
                self.registers.write_byte(destination, value);

                self.set_flag(Flag::Sign, (value as i8) < 0);
                self.set_flag(Flag::Zero, value == 0);
                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::ParityOverflow, (value.count_ones() & 1) == 0);
                self.set_flag(Flag::AddSubtract, false);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::In(_, _) => {
                unreachable!();
            }
            Instruction::Inc(destination) => {
                match destination {
                    Operand::Register16(register) => {
                        let value = self.registers.read_word(register);
                        self.registers.write_word(register, value.wrapping_add(1));
                    }
                    _ => {
                        let old_value = self.load_byte(memory, destination);
                        self.set_flag(Flag::ParityOverflow, old_value == 0x7f);
                        let (value, _) = old_value.overflowing_add(1);
                        self.store_byte(memory, destination, value);

                        self.set_flag(Flag::Sign, (value as i8) < 0);
                        self.set_flag(Flag::Zero, value == 0);
                        self.set_flag(Flag::HalfCarry, (((old_value & 0xf) + 1) & 0x10) != 0);
                        self.set_flag(Flag::AddSubtract, false);
                    }
                }

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Ind => {
                let address = self.registers.read_word(&Register16::BC);

                let value = bus.read_byte(address);
                self.store_byte(memory, &Operand::RegisterIndirect(Register16::HL), value);

                let address = self.registers.read_word(&Register16::HL).wrapping_sub(1);
                self.registers.write_word(&Register16::HL, address);

                let counter = self.registers.read_byte(&Register8::B).wrapping_sub(1);
                self.registers.write_byte(&Register8::B, counter);

                self.set_flag(Flag::Zero, counter == 0);
                self.set_flag(Flag::AddSubtract, true);
                // TODO: parity, sign, half carry flags (according to http://www.z80.info/z80sflag.htm)

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Indr => {
                let address = self.registers.read_word(&Register16::BC);

                let value = bus.read_byte(address);
                self.store_byte(memory, &Operand::RegisterIndirect(Register16::HL), value);

                let address = self.registers.read_word(&Register16::HL).wrapping_sub(1);
                self.registers.write_word(&Register16::HL, address);

                let counter = self.registers.read_byte(&Register8::B).wrapping_sub(1);
                self.registers.write_byte(&Register8::B, counter);

                self.set_flag(Flag::Zero, counter == 0);
                self.set_flag(Flag::AddSubtract, true);
                // TODO: parity, sign, half carry flags (according to http://www.z80.info/z80sflag.htm)

                if counter == 0 {
                    self.registers
                        .write_word(&Register16::PC, next_address as u16);
                    timing_in_nops = 5; // not having to adjust the PC saves time
                }
            }
            Instruction::Ini => {
                let address = self.registers.read_word(&Register16::BC);

                let value = bus.read_byte(address);
                self.store_byte(memory, &Operand::RegisterIndirect(Register16::HL), value);

                let address = self.registers.read_word(&Register16::HL).wrapping_add(1);
                self.registers.write_word(&Register16::HL, address);

                let counter = self.registers.read_byte(&Register8::B).wrapping_sub(1);
                self.registers.write_byte(&Register8::B, counter);

                self.set_flag(Flag::Zero, counter == 0);
                self.set_flag(Flag::AddSubtract, true);
                // TODO: parity, sign, half carry flags (according to http://www.z80.info/z80sflag.htm)

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Inir => {
                let address = self.registers.read_word(&Register16::BC);

                let value = bus.read_byte(address);
                self.store_byte(memory, &Operand::RegisterIndirect(Register16::HL), value);

                let address = self.registers.read_word(&Register16::HL).wrapping_add(1);
                self.registers.write_word(&Register16::HL, address);

                let counter = self.registers.read_byte(&Register8::B).wrapping_sub(1);
                self.registers.write_byte(&Register8::B, counter);

                self.set_flag(Flag::Zero, counter == 0);
                self.set_flag(Flag::AddSubtract, true);
                // TODO: parity, sign, half carry flags (according to http://www.z80.info/z80sflag.htm)

                if counter == 0 {
                    self.registers
                        .write_word(&Register16::PC, next_address as u16);
                    timing_in_nops = 5; // not having to adjust the PC saves time
                }
            }
            Instruction::Jp(jump_test, target) => {
                if self.check_jump(jump_test) {
                    let address = match target {
                        Operand::RegisterIndirect(register) => {
                            self.registers.read_word(register) // special case for JP (HL) and friends
                        }
                        _ => self.load_word(memory, target),
                    };

                    self.registers.write_word(&Register16::PC, address);
                } else {
                    self.registers
                        .write_word(&Register16::PC, next_address as u16);
                }
            }
            Instruction::Jr(jump_test, target) => {
                if self.check_jump(jump_test) {
                    let address = match target {
                        Operand::Immediate16(address) => address,
                        _ => {
                            unimplemented!();
                        }
                    };
                    self.registers.write_word(&Register16::PC, *address);
                } else {
                    self.registers
                        .write_word(&Register16::PC, next_address as u16);
                    timing_in_nops = 2;
                }
            }
            Instruction::Ld(destination, source) => {
                match (&destination, &source) {
                    (Operand::Register16(_), _) => {
                        let value = self.load_word(memory, source);
                        self.store_word(memory, destination, value);
                    }
                    (_, Operand::Register16(_)) => {
                        let value = self.load_word(memory, source);
                        self.store_word(memory, destination, value);
                    }
                    _ => {
                        // TODO: store iff2 in parity flag if instruction is ld a,i or ld a,r
                        let value = self.load_byte(memory, source);
                        self.store_byte(memory, destination, value);
                    }
                }

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::LdDirect16(destination, source) => {
                let value = self.load_word(memory, source);
                self.store_word(memory, destination, value);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Ldd => {
                let value = self.load_byte(memory, &Operand::RegisterIndirect(Register16::HL));
                self.store_byte(memory, &Operand::RegisterIndirect(Register16::DE), value);

                let address = self.registers.read_word(&Register16::DE).wrapping_sub(1);
                self.registers.write_word(&Register16::DE, address);

                let address = self.registers.read_word(&Register16::HL).wrapping_sub(1);
                self.registers.write_word(&Register16::HL, address);

                let counter = self.registers.read_word(&Register16::BC).wrapping_sub(1);
                self.registers.write_word(&Register16::BC, counter);

                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::ParityOverflow, counter != 0);
                self.set_flag(Flag::AddSubtract, false);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Lddr => {
                let value = self.load_byte(memory, &Operand::RegisterIndirect(Register16::HL));
                self.store_byte(memory, &Operand::RegisterIndirect(Register16::DE), value);

                let address = self.registers.read_word(&Register16::DE).wrapping_sub(1);
                self.registers.write_word(&Register16::DE, address);

                let address = self.registers.read_word(&Register16::HL).wrapping_sub(1);
                self.registers.write_word(&Register16::HL, address);

                let counter = self.registers.read_word(&Register16::BC).wrapping_sub(1);
                self.registers.write_word(&Register16::BC, counter);

                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::ParityOverflow, counter != 0);
                self.set_flag(Flag::AddSubtract, false);

                if counter == 0 {
                    self.registers
                        .write_word(&Register16::PC, next_address as u16);
                    timing_in_nops = 5; // not having to adjust the PC saves time
                }
            }
            Instruction::Ldi => {
                let value = self.load_byte(memory, &Operand::RegisterIndirect(Register16::HL));
                self.store_byte(memory, &Operand::RegisterIndirect(Register16::DE), value);

                let address = self.registers.read_word(&Register16::DE).wrapping_add(1);
                self.registers.write_word(&Register16::DE, address);

                let address = self.registers.read_word(&Register16::HL).wrapping_add(1);
                self.registers.write_word(&Register16::HL, address);

                let counter = self.registers.read_word(&Register16::BC).wrapping_sub(1);
                self.registers.write_word(&Register16::BC, counter);

                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::ParityOverflow, counter != 0);
                self.set_flag(Flag::AddSubtract, false);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Ldir => {
                let value = self.load_byte(memory, &Operand::RegisterIndirect(Register16::HL));
                self.store_byte(memory, &Operand::RegisterIndirect(Register16::DE), value);

                let address = self.registers.read_word(&Register16::DE).wrapping_add(1);
                self.registers.write_word(&Register16::DE, address);

                let address = self.registers.read_word(&Register16::HL).wrapping_add(1);
                self.registers.write_word(&Register16::HL, address);

                let counter = self.registers.read_word(&Register16::BC).wrapping_sub(1);
                self.registers.write_word(&Register16::BC, counter);

                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::ParityOverflow, counter != 0);
                self.set_flag(Flag::AddSubtract, false);

                if counter == 0 {
                    self.registers
                        .write_word(&Register16::PC, next_address as u16);
                    timing_in_nops = 5; // not having to adjust the PC saves time
                }
            }
            Instruction::Neg => {
                let right = self.registers.read_byte(&Register8::A);
                let (value, carry) = (0_u8).overflowing_sub(right);
                let (_, overflow) = (0_i8).overflowing_sub(right as i8);
                self.registers.write_byte(&Register8::A, value);

                self.set_flag(Flag::Sign, (value as i8) < 0); // TODO: make this reusable?
                self.set_flag(Flag::Zero, value == 0);
                self.set_flag(Flag::HalfCarry, 0 < (right & 0xf));
                self.set_flag(Flag::ParityOverflow, overflow);
                self.set_flag(Flag::AddSubtract, true);
                self.set_flag(Flag::Carry, carry);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Nop => {
                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Or(operand) => {
                let value = self.load_byte(memory, operand);

                let result = self.registers.read_byte(&Register8::A) | value;
                self.registers.write_byte(&Register8::A, result);

                self.set_flag(Flag::Sign, (result as i8) < 0); // TODO: make this reusable?
                self.set_flag(Flag::Zero, result == 0);
                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::ParityOverflow, (result.count_ones() & 1) == 0);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, false);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Out(Operand::Direct8(port_low), Operand::Register8(Register8::A)) => {
                let value = self.registers.read_byte(&Register8::A);
                let address = ((value as u16) << 8) + *port_low as u16;
                bus.write_byte(memory, address, value);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Out(Operand::RegisterIndirect(port), Operand::Register8(source)) => {
                let address = self.registers.read_word(port);
                let value = self.registers.read_byte(source);
                bus.write_byte(memory, address, value);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Out(Operand::RegisterIndirect(port), Operand::Immediate8(value)) => {
                // TODO: make this a special case of the other OUT instruction above
                let address = self.registers.read_word(port);
                bus.write_byte(memory, address, *value);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Out(_, _) => {
                unreachable!();
            }
            Instruction::Outd => {
                let value = self.load_byte(memory, &Operand::RegisterIndirect(Register16::HL));

                let address = self.registers.read_word(&Register16::HL).wrapping_sub(1);
                self.registers.write_word(&Register16::HL, address);

                let counter = self.registers.read_byte(&Register8::B).wrapping_sub(1);
                self.registers.write_byte(&Register8::B, counter);

                let address = self.registers.read_word(&Register16::BC);

                bus.write_byte(memory, address, value);

                self.set_flag(Flag::Zero, counter == 0);
                self.set_flag(Flag::AddSubtract, true);
                // TODO: parity, sign, half carry flags (according to http://www.z80.info/z80sflag.htm)

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Outi => {
                let value = self.load_byte(memory, &Operand::RegisterIndirect(Register16::HL));

                let address = self.registers.read_word(&Register16::HL).wrapping_add(1);
                self.registers.write_word(&Register16::HL, address);

                let counter = self.registers.read_byte(&Register8::B).wrapping_sub(1);
                self.registers.write_byte(&Register8::B, counter);

                let address = self.registers.read_word(&Register16::BC);

                bus.write_byte(memory, address, value);

                self.set_flag(Flag::Zero, counter == 0);
                self.set_flag(Flag::AddSubtract, true);
                // TODO: parity, sign, half carry flags (according to http://www.z80.info/z80sflag.htm)

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Otdr => {
                let value = self.load_byte(memory, &Operand::RegisterIndirect(Register16::HL));

                let address = self.registers.read_word(&Register16::HL).wrapping_sub(1);
                self.registers.write_word(&Register16::HL, address);

                let counter = self.registers.read_byte(&Register8::B).wrapping_sub(1);
                self.registers.write_byte(&Register8::B, counter);

                let address = self.registers.read_word(&Register16::BC);

                bus.write_byte(memory, address, value);

                self.set_flag(Flag::Zero, counter == 0);
                self.set_flag(Flag::AddSubtract, true);
                // TODO: parity, sign, half carry flags (according to http://www.z80.info/z80sflag.htm)

                if counter == 0 {
                    self.registers
                        .write_word(&Register16::PC, next_address as u16);
                    timing_in_nops = 5; // not having to adjust the PC saves time
                }
            }
            Instruction::Otir => {
                let value = self.load_byte(memory, &Operand::RegisterIndirect(Register16::HL));

                let address = self.registers.read_word(&Register16::HL).wrapping_add(1);
                self.registers.write_word(&Register16::HL, address);

                let counter = self.registers.read_byte(&Register8::B).wrapping_sub(1);
                self.registers.write_byte(&Register8::B, counter);

                let address = self.registers.read_word(&Register16::BC);

                bus.write_byte(memory, address, value);

                self.set_flag(Flag::Zero, counter == 0);
                self.set_flag(Flag::AddSubtract, true);
                // TODO: parity, sign, half carry flags (according to http://www.z80.info/z80sflag.htm)

                if counter == 0 {
                    self.registers
                        .write_word(&Register16::PC, next_address as u16);
                    timing_in_nops = 5; // not having to adjust the PC saves time
                }
            }
            Instruction::Pop(Operand::Register16(destination)) => {
                let old_sp = self.registers.read_word(&Register16::SP);
                self.registers.write_word(&Register16::SP, old_sp + 2);
                self.registers
                    .write_word(destination, memory.read_word(old_sp as usize));
                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Pop(_) => {
                unreachable!();
            }
            Instruction::Push(Operand::Register16(source)) => {
                let new_sp = self.registers.read_word(&Register16::SP) - 2;
                self.registers.write_word(&Register16::SP, new_sp);
                memory.write_word(new_sp as usize, self.registers.read_word(source));
                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Push(_) => {
                unreachable!();
            }
            Instruction::Res(destination, Operand::Bit(bit), operand) => {
                let value = self.load_byte(memory, operand) & (!(1 << bit));
                self.store_byte(memory, destination, value); // copy for undocumented instructions
                self.store_byte(memory, operand, value);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Res(_, _, _) => {
                unreachable!();
            }
            Instruction::Ret(jump_test) => {
                if self.check_jump(jump_test) {
                    let old_sp = self.registers.read_word(&Register16::SP);
                    self.registers.write_word(&Register16::SP, old_sp + 2);
                    self.registers
                        .write_word(&Register16::PC, memory.read_word(old_sp as usize));
                } else {
                    self.registers
                        .write_word(&Register16::PC, next_address as u16);
                }
            }
            Instruction::Reti => {
                let old_sp = self.registers.read_word(&Register16::SP);
                self.registers.write_word(&Register16::SP, old_sp + 2);
                self.registers
                    .write_word(&Register16::PC, memory.read_word(old_sp as usize));
            }
            Instruction::Retn => {
                self.iff1 = self.iff2;
                let old_sp = self.registers.read_word(&Register16::SP);
                self.registers.write_word(&Register16::SP, old_sp + 2);
                self.registers
                    .write_word(&Register16::PC, memory.read_word(old_sp as usize));
            }
            Instruction::Rl(destination, operand) => {
                let value = self.load_byte(memory, operand);
                let carry_value = if self.check_flag(Flag::Carry) { 1 } else { 0 };
                let carry = (value >> 7) != 0;
                let value = (value.rotate_left(1) & 0xfe) | carry_value;
                self.store_byte(memory, destination, value); // copy for undocumented instructions
                self.store_byte(memory, operand, value);

                self.set_flag(Flag::Sign, (value as i8) < 0);
                self.set_flag(Flag::Zero, value == 0);
                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::ParityOverflow, (value.count_ones() & 1) == 0);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, carry);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Rla => {
                let value = self.registers.read_byte(&Register8::A);
                let carry_value = if self.check_flag(Flag::Carry) { 1 } else { 0 };
                let carry = (value >> 7) != 0;
                let result = (value.rotate_left(1) & 0xfe) | carry_value;
                self.registers.write_byte(&Register8::A, result);

                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, carry);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Rlc(destination, operand) => {
                let value = self.load_byte(memory, operand).rotate_left(1);
                self.store_byte(memory, destination, value); // copy for undocumented instructions
                self.store_byte(memory, operand, value);

                self.set_flag(Flag::Sign, (value as i8) < 0);
                self.set_flag(Flag::Zero, value == 0);
                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::ParityOverflow, (value.count_ones() & 1) == 0);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, (value & 1) != 0);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Rlca => {
                let value = self.registers.read_byte(&Register8::A);
                let result = value.rotate_left(1);
                self.registers.write_byte(&Register8::A, result);

                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, (result & 1) != 0);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Rr(destination, operand) => {
                let value = self.load_byte(memory, operand);
                let carry_value = if self.check_flag(Flag::Carry) { 1 } else { 0 };
                let carry = (value & 1) != 0;
                let value = (value.rotate_right(1) & 0x7f) | (carry_value << 7);
                self.store_byte(memory, destination, value); // copy for undocumented instructions
                self.store_byte(memory, operand, value);

                self.set_flag(Flag::Sign, (value as i8) < 0);
                self.set_flag(Flag::Zero, value == 0);
                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::ParityOverflow, (value.count_ones() & 1) == 0);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, carry);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Rra => {
                let value = self.registers.read_byte(&Register8::A);
                let carry_value = if self.check_flag(Flag::Carry) { 1 } else { 0 };
                let carry = (value & 1) != 0;
                let result = (value.rotate_right(1) & 0x7f) | (carry_value << 7);
                self.registers.write_byte(&Register8::A, result);

                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, carry);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Rrc(destination, operand) => {
                let value = self.load_byte(memory, operand).rotate_right(1);
                self.store_byte(memory, destination, value); // copy for undocumented instructions
                self.store_byte(memory, operand, value);

                self.set_flag(Flag::Sign, (value as i8) < 0);
                self.set_flag(Flag::Zero, value == 0);
                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::ParityOverflow, (value.count_ones() & 1) == 0);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, (value & 0x80) != 0);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Rrca => {
                let value = self.registers.read_byte(&Register8::A);
                let result = value.rotate_right(1);
                self.registers.write_byte(&Register8::A, result);

                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, (result & 0x80) != 0);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Rld => {
                let accumulator = self.registers.read_byte(&Register8::A);
                let memory_hl = self.load_byte(memory, &Operand::RegisterIndirect(Register16::HL));
                let new_accumulator = (accumulator & 0xf0) | (memory_hl >> 4);
                self.registers.write_byte(&Register8::A, new_accumulator);
                let new_memory_hl = (memory_hl << 4) | (accumulator & 0xf);
                self.store_byte(
                    memory,
                    &Operand::RegisterIndirect(Register16::HL),
                    new_memory_hl,
                );

                self.set_flag(Flag::Sign, (new_accumulator as i8) < 0);
                self.set_flag(Flag::Zero, new_accumulator == 0);
                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(
                    Flag::ParityOverflow,
                    (new_accumulator.count_ones() & 1) == 0,
                );
                self.set_flag(Flag::AddSubtract, false);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Rrd => {
                let accumulator = self.registers.read_byte(&Register8::A);
                let memory_hl = self.load_byte(memory, &Operand::RegisterIndirect(Register16::HL));
                let new_accumulator = (accumulator & 0xf0) | (memory_hl & 0xf);
                self.registers.write_byte(&Register8::A, new_accumulator);
                let new_memory_hl = ((accumulator & 0xf) << 4) | (memory_hl >> 4);
                self.store_byte(
                    memory,
                    &Operand::RegisterIndirect(Register16::HL),
                    new_memory_hl,
                );

                self.set_flag(Flag::Sign, (new_accumulator as i8) < 0);
                self.set_flag(Flag::Zero, new_accumulator == 0);
                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(
                    Flag::ParityOverflow,
                    (new_accumulator.count_ones() & 1) == 0,
                );
                self.set_flag(Flag::AddSubtract, false);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Rst(target) => {
                let new_sp = self.registers.read_word(&Register16::SP) - 2;
                self.registers.write_word(&Register16::SP, new_sp);
                memory.write_word(new_sp as usize, next_address as u16);
                let address_lower = self.load_byte(memory, target);

                self.registers
                    .write_word(&Register16::PC, address_lower as u16);
            }
            Instruction::Sbc(destination, source) => {
                match destination {
                    Operand::Register8(Register8::A) => {
                        let left = self.load_byte(memory, destination);
                        let right = self.load_byte(memory, source);
                        let carry_value = if self.check_flag(Flag::Carry) { 1 } else { 0 };
                        let (value, carry1) = right.overflowing_add(carry_value);
                        let (value, carry2) = left.overflowing_sub(value);
                        let overflow =
                            (left & 0x80) != (right & 0x80) && (right & 0x80) == (value & 0x80);
                        self.store_byte(memory, destination, value);

                        self.set_flag(Flag::Sign, (value as i8) < 0);
                        self.set_flag(Flag::Zero, value == 0);
                        self.set_flag(Flag::HalfCarry, (left & 0xf) < (right & 0xf) + carry_value);
                        self.set_flag(Flag::ParityOverflow, overflow);
                        self.set_flag(Flag::AddSubtract, true);
                        self.set_flag(Flag::Carry, carry1 || carry2);
                    }
                    Operand::Register16(Register16::HL) => {
                        let left = self.load_word(memory, destination);
                        let right = self.load_word(memory, source);
                        let carry_value = if self.check_flag(Flag::Carry) { 1 } else { 0 };
                        let (value, carry1) = right.overflowing_add(carry_value);
                        let (value, carry2) = left.overflowing_sub(value);
                        let overflow = (left & 0x8000) != (right & 0x8000)
                            && (right & 0x8000) == (value & 0x8000);
                        self.store_word(memory, destination, value);

                        self.set_flag(Flag::Sign, (value as i16) < 0);
                        self.set_flag(Flag::Zero, value == 0);
                        self.set_flag(
                            Flag::HalfCarry,
                            (left & 0xfff) < (right & 0xfff) + carry_value,
                        );
                        self.set_flag(Flag::ParityOverflow, overflow);
                        self.set_flag(Flag::AddSubtract, true);
                        self.set_flag(Flag::Carry, carry1 || carry2);
                    }
                    _ => unreachable!(),
                }

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Scf => {
                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, true);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Set(destination, Operand::Bit(bit), operand) => {
                let value = self.load_byte(memory, operand) | (1 << bit);
                self.store_byte(memory, destination, value); // copy for undocumented instructions
                self.store_byte(memory, operand, value);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Set(_, _, _) => {
                unreachable!();
            }
            Instruction::Sla(destination, operand) => {
                let value = self.load_byte(memory, operand);
                let carry = (value >> 7) != 0;
                let result = value << 1;
                self.store_byte(memory, destination, result); // copy for undocumented instructions
                self.store_byte(memory, operand, result);

                self.set_flag(Flag::Sign, (result as i8) < 0); // TODO: make this reusable?
                self.set_flag(Flag::Zero, result == 0);
                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::ParityOverflow, (result.count_ones() & 1) == 0);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, carry);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Sll(destination, operand) => {
                let value = self.load_byte(memory, operand);
                let carry = (value >> 7) != 0;
                let result = (value << 1) | 1;
                self.store_byte(memory, destination, result); // copy for undocumented instructions
                self.store_byte(memory, operand, result);

                self.set_flag(Flag::Sign, (result as i8) < 0); // TODO: make this reusable?
                self.set_flag(Flag::Zero, result == 0);
                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::ParityOverflow, (result.count_ones() & 1) == 0);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, carry);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Sra(destination, operand) => {
                let value = self.load_byte(memory, operand);
                let sign = value & 0x80;
                let carry = (value & 1) != 0;
                let result = sign | (value >> 1);
                self.store_byte(memory, destination, result); // copy for undocumented instructions
                self.store_byte(memory, operand, result);

                self.set_flag(Flag::Sign, (result as i8) < 0); // TODO: make this reusable?
                self.set_flag(Flag::Zero, result == 0);
                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::ParityOverflow, (result.count_ones() & 1) == 0);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, carry);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Srl(destination, operand) => {
                let value = self.load_byte(memory, operand);
                let carry = (value & 1) != 0;
                let result = value >> 1;
                self.store_byte(memory, destination, result); // copy for undocumented instructions
                self.store_byte(memory, operand, result);

                self.set_flag(Flag::Sign, (result as i8) < 0); // TODO: make this reusable?
                self.set_flag(Flag::Zero, result == 0);
                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::ParityOverflow, (result.count_ones() & 1) == 0);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, carry);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Sub(operand) => {
                // ALUOP / checked
                let left = self.registers.read_byte(&Register8::A);
                let right = self.load_byte(memory, operand);
                let (value, carry) = left.overflowing_sub(right);
                let (_, overflow) = (left as i8).overflowing_sub(right as i8);
                self.registers.write_byte(&Register8::A, value);

                self.set_flag(Flag::Sign, (value as i8) < 0); // TODO: make this reusable?
                self.set_flag(Flag::Zero, value == 0);
                self.set_flag(Flag::HalfCarry, (left & 0xf) < (right & 0xf));
                self.set_flag(Flag::ParityOverflow, overflow);
                self.set_flag(Flag::AddSubtract, true);
                self.set_flag(Flag::Carry, carry);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Xor(operand) => {
                let value = self.load_byte(memory, operand);

                let result = self.registers.read_byte(&Register8::A) ^ value;
                self.registers.write_byte(&Register8::A, result);

                self.set_flag(Flag::Sign, (result as i8) < 0); // TODO: make this reusable?
                self.set_flag(Flag::Zero, result == 0);
                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::ParityOverflow, (result.count_ones() & 1) == 0);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, false);

                self.registers
                    .write_word(&Register16::PC, next_address as u16);
            }
        }

        if !prevent_interrupt && self.handle_interrupt(memory) {
            return (timing_in_nops + 4, true);
        }

        (timing_in_nops, false)
    }

    fn request_interrupt(&mut self) {
        self.irq_received = true;
    }

    fn disassemble(
        &mut self,
        memory: &mut (impl MemRead + MemWrite + MemManage),
        count: usize,
    ) -> Vec<(u16, String)> {
        let mut address = self.registers.read_word(&Register16::PC);

        let mut assembly = Vec::with_capacity(count);
        for _ in 0..count {
            let (instruction, next_address) = self.decoder.decode(memory, address as usize);
            assembly.push((address, format!("{instruction}")));
            address = next_address as u16;
        }

        assembly
    }
}

impl<D> Snapshotable for ZilogZ80<D>
where
    D: Decoder,
{
    type View = CpuDebugView;

    fn debug_view(&self) -> Self::View {
        let registers = self.registers.debug_view();
        let iff1 = self.iff1;
        let iff2 = self.iff2;
        let halted = self.halted;
        let interrupt_mode = self.interrupt_mode;
        let enable_interrupt = self.enable_interrupt;
        let irq_received = self.irq_received;

        Self::View {
            register_a: registers.register_a,
            register_f: registers.register_f,
            register_b: registers.register_b,
            register_c: registers.register_c,
            register_d: registers.register_d,
            register_e: registers.register_e,
            register_h: registers.register_h,
            register_l: registers.register_l,
            shadow_register_a: registers.shadow_register_a,
            shadow_register_f: registers.shadow_register_f,
            shadow_register_b: registers.shadow_register_b,
            shadow_register_c: registers.shadow_register_c,
            shadow_register_d: registers.shadow_register_d,
            shadow_register_e: registers.shadow_register_e,
            shadow_register_h: registers.shadow_register_h,
            shadow_register_l: registers.shadow_register_l,
            register_i: registers.register_i,
            register_r: registers.register_r,
            register_ixh: registers.register_ixh,
            register_ixl: registers.register_ixl,
            register_iyh: registers.register_iyh,
            register_iyl: registers.register_iyl,
            register_sp: registers.register_sp,
            register_pc: registers.register_pc,
            iff1,
            iff2,
            halted,
            interrupt_mode,
            enable_interrupt,
            irq_received,
        }
    }
}

impl<D> Debuggable for ZilogZ80<D>
where
    D: Decoder,
{
    const SOURCE: DebugSource = DebugSource::Cpu;
    type Event = CpuDebugEvent;
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        system::{
            bus::Bus,
            instruction::AlgorithmicDecoder,
            memory::{MemManage, Ram},
        },
        AudioSink, VideoSink,
    };

    use super::*;

    struct ZexHarness {
        cpu: ZilogZ80<AlgorithmicDecoder>,
        memory: Ram,
        bus: BlackHole,
    }

    impl ZexHarness {
        pub fn new(rom: &[u8]) -> ZexHarness {
            let mut memory = Ram::from_bytes(0x10000, rom, 0x100);
            memory.write_byte(0x0005, 0xc9); // patch with RET instruction
            memory.write_word(0x0006, 0xe400); // patch with initial SP

            ZexHarness {
                cpu: ZilogZ80::new(0x100),
                memory,
                bus: BlackHole::new(),
            }
        }

        pub fn emulate(&mut self) -> usize {
            let mut output = String::new();

            let start = std::time::Instant::now();
            let mut total_cycles = 0;

            loop {
                match self.cpu.registers.read_word(&Register16::PC) {
                    0x0000 => break,
                    0x0005 => {
                        match self.cpu.registers.read_byte(&Register8::C) {
                            2 => {
                                print!("{}", self.cpu.registers.read_byte(&Register8::E) as char);
                                output.push(self.cpu.registers.read_byte(&Register8::E) as char);
                            }
                            9 => {
                                let mut address =
                                    self.cpu.registers.read_word(&Register16::DE) as usize;
                                loop {
                                    let character = self.memory.read_byte(address) as char;
                                    if character == '$' {
                                        break;
                                    } else {
                                        print!("{character}");
                                        output.push(character);
                                    }
                                    address += 1;
                                }
                            }
                            _ => unreachable!(),
                        }
                        let (cycles, _) =
                            self.cpu.fetch_and_execute(&mut self.memory, &mut self.bus);
                        total_cycles += cycles as usize;
                    }
                    _ => {
                        let (cycles, _) =
                            self.cpu.fetch_and_execute(&mut self.memory, &mut self.bus);
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

            output.matches("OK").count()
        }
    }

    #[derive(Default)]
    struct BlackHole {}

    impl BlackHole {
        pub fn new() -> BlackHole {
            BlackHole {}
        }
    }

    impl Bus for BlackHole {
        fn read_byte(&mut self, _port: u16) -> u8 {
            unimplemented!()
        }

        fn write_byte(&mut self, _memory: &mut impl MemManage, _port: u16, _value: u8) {
            unimplemented!();
        }

        fn step(
            &mut self,
            _memory: &mut impl MemManage,
            _video: &mut impl VideoSink,
            _audio: &mut impl AudioSink,
        ) -> bool {
            unimplemented!();
        }

        fn acknowledge_interrupt(&mut self) {
            unimplemented!();
        }

        fn set_key(&mut self, _line: usize, _bit: u8) {
            unimplemented!();
        }

        fn unset_key(&mut self, _line: usize, _bit: u8) {
            unimplemented!();
        }

        fn load_disk(&mut self, _drive: usize, _rom: Vec<u8>, _path: PathBuf) {
            unimplemented!();
        }
    }

    #[test]
    #[ignore = "this is an extremely slow test"]
    fn test_zexdoc_slow() {
        let rom = include_bytes!("../../rom/zexdoc.rom");
        let mut harness = ZexHarness::new(rom);
        assert_eq!(harness.emulate(), 67);
    }
}
