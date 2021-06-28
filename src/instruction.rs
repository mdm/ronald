use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::cpu;
use crate::memory::Read;

pub enum Operand {
    Immediate8(u8),
    Immediate16(u16),
    Register8(cpu::Register8),
    Register16(cpu::Register16),
    RegisterIndirect(cpu::Register16),
    Direct8(u8),
    Direct16(u16),
    Indexed(cpu::Register16, i8),
    Bit(u8), // TODO: use Immediate8? or rename to Immediate1?
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Immediate8(value) => write!(f, "{:#04x}", value),
            Operand::Immediate16(value) => write!(f, "{:#06x}", value),
            Operand::Register8(register) => match register {
                cpu::Register8::A => write!(f, "a"),
                cpu::Register8::F => write!(f, "f"),
                cpu::Register8::B => write!(f, "b"),
                cpu::Register8::C => write!(f, "c"),
                cpu::Register8::D => write!(f, "d"),
                cpu::Register8::E => write!(f, "e"),
                cpu::Register8::H => write!(f, "h"),
                cpu::Register8::L => write!(f, "l"),
                cpu::Register8::I => write!(f, "i"),
                cpu::Register8::R => write!(f, "r"),
                cpu::Register8::IXH => write!(f, "ixh"),
                cpu::Register8::IXL => write!(f, "ixl"),
                cpu::Register8::IYH => write!(f, "iyh"),
                cpu::Register8::IYL => write!(f, "iyl"),
            },
            Operand::Register16(register) => match register {
                cpu::Register16::AF => write!(f, "af"),
                cpu::Register16::BC => write!(f, "bc"),
                cpu::Register16::DE => write!(f, "de"),
                cpu::Register16::HL => write!(f, "hl"),
                cpu::Register16::IX => write!(f, "ix"),
                cpu::Register16::IY => write!(f, "iy"),
                cpu::Register16::SP => write!(f, "sp"),
                cpu::Register16::PC => write!(f, "pc"),
            },
            Operand::RegisterIndirect(register) => match register {
                // TODO: remove unused match arms
                cpu::Register16::AF => write!(f, "(af)"),
                cpu::Register16::BC => write!(f, "(bc)"),
                cpu::Register16::DE => write!(f, "(de)"),
                cpu::Register16::HL => write!(f, "(hl)"),
                cpu::Register16::IX => write!(f, "(ix)"),
                cpu::Register16::IY => write!(f, "(iy)"),
                cpu::Register16::SP => write!(f, "(sp)"),
                cpu::Register16::PC => write!(f, "(pc)"),
            },
            Operand::Direct8(address) => write!(f, "({:#04x})", address),
            Operand::Direct16(address) => write!(f, "({:#06x})", address),
            Operand::Indexed(register, displacement) => match register {
                cpu::Register16::IX => write!(f, "(ix{:+})", displacement),
                cpu::Register16::IY => write!(f, "(iy{:+})", displacement),
                _ => unreachable!(),
            },
            Operand::Bit(bit) => write!(f, "{}", bit),
        }
    }
}

pub enum InterruptMode {
    // TODO: move to CPU?
    Mode0,
    Mode1,
    Mode2,
}

pub enum JumpTest {
    Unconditional,
    NonZero,
    Zero,
    NoCarry,
    Carry,
    ParityOdd,
    ParityEven,
    SignPositive,
    SignNegative,
}

impl JumpTest {
    // TODO: move to decoder?
    fn decode(encoded: u8) -> JumpTest {
        match encoded {
            0 => JumpTest::NonZero,
            1 => JumpTest::Zero,
            2 => JumpTest::NoCarry,
            3 => JumpTest::Carry,
            4 => JumpTest::ParityOdd,
            5 => JumpTest::ParityEven,
            6 => JumpTest::SignPositive,
            7 => JumpTest::SignNegative,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for JumpTest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JumpTest::Unconditional => write!(f, ""),
            JumpTest::NonZero => write!(f, "nz"),
            JumpTest::Zero => write!(f, "z"),
            JumpTest::NoCarry => write!(f, "nc"),
            JumpTest::Carry => write!(f, "c"),
            JumpTest::ParityOdd => write!(f, "po"),
            JumpTest::ParityEven => write!(f, "pe"),
            JumpTest::SignPositive => write!(f, "p"),
            JumpTest::SignNegative => write!(f, "m"),
        }
    }
}

pub enum Instruction {
    Adc(Operand, Operand),
    Add(Operand, Operand),
    And(Operand),
    Bit(Operand, Operand),
    Call(JumpTest, Operand),
    Ccf,
    Cp(Operand),
    Cpd,
    Cpdr,
    Cpi,
    Cpir,
    Cpl,
    Daa,
    Dec(Operand),
    Defb(Operand),
    Defw(Operand),
    Di,
    Djnz(Operand),
    Ei,
    Ex(Operand, Operand),
    Exx,
    Halt,
    Im(InterruptMode),
    In(Operand, Operand),
    Ind,
    Indr,
    Ini,
    Inir,
    Inc(Operand),
    Jp(JumpTest, Operand),
    Jr(JumpTest, Operand),
    Ld(Operand, Operand),
    LdDirect16(Operand, Operand), // this is treated special because the timing is different
    Ldd,
    Lddr,
    Ldi,
    Ldir,
    Neg,
    Nop,
    Or(Operand),
    Out(Operand, Operand),
    Otdr,
    Otir,
    Outd,
    Outi,
    Pop(Operand),
    Push(Operand),
    Res(Operand, Operand, Operand),
    Ret(JumpTest),
    Reti,
    Retn,
    Rl(Operand, Operand),
    Rla,
    Rlc(Operand, Operand),
    Rlca,
    Rld,
    Rr(Operand, Operand),
    Rrc(Operand, Operand),
    Rra,
    Rrca,
    Rrd,
    Rst(Operand),
    Sbc(Operand, Operand),
    Scf,
    Set(Operand, Operand, Operand),
    Sla(Operand, Operand),
    Sll(Operand, Operand),
    Sra(Operand, Operand),
    Srl(Operand, Operand),
    Sub(Operand),
    Xor(Operand),
}

impl Instruction {
    pub fn timing(&self) -> u8 {
        match self {
            Instruction::Adc(
                Operand::Register8(cpu::Register8::A),
                Operand::Register8(cpu::Register8::IXH),
            ) => 2,
            Instruction::Adc(
                Operand::Register8(cpu::Register8::A),
                Operand::Register8(cpu::Register8::IXL),
            ) => 2,
            Instruction::Adc(
                Operand::Register8(cpu::Register8::A),
                Operand::Register8(cpu::Register8::IYH),
            ) => 2,
            Instruction::Adc(
                Operand::Register8(cpu::Register8::A),
                Operand::Register8(cpu::Register8::IYL),
            ) => 2,
            Instruction::Adc(Operand::Register8(cpu::Register8::A), Operand::Register8(_)) => 1,
            Instruction::Adc(Operand::Register8(cpu::Register8::A), Operand::Immediate8(_)) => 2,
            Instruction::Adc(
                Operand::Register8(cpu::Register8::A),
                Operand::RegisterIndirect(cpu::Register16::HL),
            ) => 2,
            Instruction::Adc(Operand::Register8(cpu::Register8::A), Operand::Indexed(_, _)) => 5,
            Instruction::Adc(Operand::Register16(cpu::Register16::HL), Operand::Register16(_)) => 4,
            Instruction::Adc(_, _) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::Add(Operand::Register8(cpu::Register8::A), Operand::Register8(_)) => 1,
            Instruction::Add(Operand::Register8(cpu::Register8::A), Operand::Immediate8(_)) => 2,
            Instruction::Add(
                Operand::Register8(cpu::Register8::A),
                Operand::RegisterIndirect(cpu::Register16::HL),
            ) => 2,
            Instruction::Add(Operand::Register8(cpu::Register8::A), Operand::Indexed(_, _)) => 5,
            Instruction::Add(Operand::Register16(cpu::Register16::HL), Operand::Register16(_)) => 3,
            Instruction::Add(Operand::Register16(cpu::Register16::IX), Operand::Register16(_)) => 4,
            Instruction::Add(Operand::Register16(cpu::Register16::IY), Operand::Register16(_)) => 4,
            Instruction::Add(_, _) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::And(Operand::Register8(cpu::Register8::IXH)) => 2,
            Instruction::And(Operand::Register8(cpu::Register8::IXL)) => 2,
            Instruction::And(Operand::Register8(cpu::Register8::IYH)) => 2,
            Instruction::And(Operand::Register8(cpu::Register8::IYL)) => 2,
            Instruction::And(Operand::Register8(_)) => 1,
            Instruction::And(Operand::Immediate8(_)) => 2,
            Instruction::And(Operand::RegisterIndirect(cpu::Register16::HL)) => 2,
            Instruction::And(Operand::Indexed(_, _)) => 5,
            Instruction::And(_) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::Bit(_, Operand::Register8(_)) => 2,
            Instruction::Bit(_, Operand::RegisterIndirect(cpu::Register16::HL)) => 3,
            Instruction::Bit(_, Operand::Indexed(_, _)) => 6, // TODO: check why this is not 5 as per official docs
            Instruction::Bit(_, _) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::Call(JumpTest::Unconditional, _) => 5,
            Instruction::Call(JumpTest::NonZero, _) => 3,
            Instruction::Call(JumpTest::Zero, _) => 5,
            Instruction::Call(JumpTest::NoCarry, _) => 3,
            Instruction::Call(JumpTest::Carry, _) => 5,
            Instruction::Call(JumpTest::ParityOdd, _) => 3,
            Instruction::Call(JumpTest::ParityEven, _) => 5,
            Instruction::Call(JumpTest::SignPositive, _) => 3,
            Instruction::Call(JumpTest::SignNegative, _) => 5,
            Instruction::Ccf => 1,
            Instruction::Cp(Operand::Register8(_)) => 1,
            Instruction::Cp(Operand::Immediate8(_)) => 2,
            Instruction::Cp(Operand::RegisterIndirect(cpu::Register16::HL)) => 2,
            Instruction::Cp(Operand::Indexed(_, _)) => 5,
            Instruction::Cp(_) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::Cpd => 5,
            Instruction::Cpdr => 6,
            Instruction::Cpi => 5,
            Instruction::Cpir => 6,
            Instruction::Cpl => 1,
            Instruction::Daa => 1,
            Instruction::Dec(Operand::Register8(_)) => 1,
            Instruction::Dec(Operand::RegisterIndirect(cpu::Register16::HL)) => 3,
            Instruction::Dec(Operand::Indexed(_, _)) => 6,
            Instruction::Dec(Operand::Register16(cpu::Register16::IX)) => 3,
            Instruction::Dec(Operand::Register16(cpu::Register16::IY)) => 3,
            Instruction::Dec(Operand::Register16(_)) => 2,
            Instruction::Dec(_) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::Defb(_) => 1,
            Instruction::Defw(_) => 2,
            Instruction::Di => 1,
            Instruction::Djnz(_) => 3,
            Instruction::Ei => 1,
            Instruction::Ex(
                Operand::Register16(cpu::Register16::AF),
                Operand::Register16(cpu::Register16::AF),
            ) => 1,
            Instruction::Ex(
                Operand::Register16(cpu::Register16::DE),
                Operand::Register16(cpu::Register16::HL),
            ) => 1,
            Instruction::Ex(
                Operand::RegisterIndirect(cpu::Register16::SP),
                Operand::Register16(cpu::Register16::HL),
            ) => 6,
            Instruction::Ex(
                Operand::RegisterIndirect(cpu::Register16::SP),
                Operand::Register16(cpu::Register16::IX),
            ) => 7,
            Instruction::Ex(
                Operand::RegisterIndirect(cpu::Register16::SP),
                Operand::Register16(cpu::Register16::IY),
            ) => 7,
            Instruction::Ex(_, _) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::Exx => 1,
            Instruction::Halt => 1,
            Instruction::Im(_) => 2,
            Instruction::In(_, Operand::RegisterIndirect(cpu::Register16::BC)) => 4,
            Instruction::In(Operand::Register8(_), _) => 3,
            Instruction::In(_, _) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::Inc(Operand::Register8(_)) => 1,
            Instruction::Inc(Operand::RegisterIndirect(cpu::Register16::HL)) => 3,
            Instruction::Inc(Operand::Indexed(_, _)) => 6,
            Instruction::Inc(Operand::Register16(cpu::Register16::IX)) => 3,
            Instruction::Inc(Operand::Register16(cpu::Register16::IY)) => 3,
            Instruction::Inc(Operand::Register16(_)) => 2,
            Instruction::Inc(_) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::Ind => 5,
            Instruction::Indr => 6,
            Instruction::Ini => 5,
            Instruction::Inir => 6,
            Instruction::Jp(
                JumpTest::Unconditional,
                Operand::RegisterIndirect(cpu::Register16::HL),
            ) => 1,
            Instruction::Jp(
                JumpTest::Unconditional,
                Operand::RegisterIndirect(cpu::Register16::IX),
            ) => 2,
            Instruction::Jp(
                JumpTest::Unconditional,
                Operand::RegisterIndirect(cpu::Register16::IY),
            ) => 2,
            Instruction::Jp(_, Operand::Immediate16(_)) => 3,
            Instruction::Jp(_, _) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::Jr(JumpTest::Unconditional, _) => 3,
            Instruction::Jr(JumpTest::NonZero, _) => 3,
            Instruction::Jr(JumpTest::Zero, _) => 3,
            Instruction::Jr(JumpTest::NoCarry, _) => 3,
            Instruction::Jr(JumpTest::Carry, _) => 3,
            Instruction::Jr(_, _) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::Ld(
                Operand::Register8(cpu::Register8::A),
                Operand::Register8(cpu::Register8::I),
            ) => 3,
            Instruction::Ld(
                Operand::Register8(cpu::Register8::I),
                Operand::Register8(cpu::Register8::A),
            ) => 3,
            Instruction::Ld(
                Operand::Register8(cpu::Register8::A),
                Operand::Register8(cpu::Register8::R),
            ) => 3,
            Instruction::Ld(
                Operand::Register8(cpu::Register8::R),
                Operand::Register8(cpu::Register8::A),
            ) => 3,
            Instruction::Ld(Operand::Register8(_), Operand::Register8(cpu::Register8::IXL)) => 2,
            Instruction::Ld(Operand::Register8(cpu::Register8::IXL), Operand::Register8(_)) => 2,
            Instruction::Ld(Operand::Register8(_), Operand::Register8(cpu::Register8::IXH)) => 2,
            Instruction::Ld(Operand::Register8(cpu::Register8::IXH), Operand::Register8(_)) => 2,
            Instruction::Ld(Operand::Register8(_), Operand::Register8(cpu::Register8::IYL)) => 2,
            Instruction::Ld(Operand::Register8(cpu::Register8::IYL), Operand::Register8(_)) => 2,
            Instruction::Ld(Operand::Register8(_), Operand::Register8(cpu::Register8::IYH)) => 2,
            Instruction::Ld(Operand::Register8(cpu::Register8::IYH), Operand::Register8(_)) => 2,
            Instruction::Ld(Operand::Register8(_), Operand::Register8(_)) => 1,
            Instruction::Ld(
                Operand::Register8(cpu::Register8::A),
                Operand::RegisterIndirect(cpu::Register16::BC),
            ) => 2,
            Instruction::Ld(
                Operand::RegisterIndirect(cpu::Register16::BC),
                Operand::Register8(cpu::Register8::A),
            ) => 2,
            Instruction::Ld(
                Operand::Register8(cpu::Register8::A),
                Operand::RegisterIndirect(cpu::Register16::DE),
            ) => 2,
            Instruction::Ld(
                Operand::RegisterIndirect(cpu::Register16::DE),
                Operand::Register8(cpu::Register8::A),
            ) => 2,
            Instruction::Ld(
                Operand::Register8(_),
                Operand::RegisterIndirect(cpu::Register16::HL),
            ) => 2,
            Instruction::Ld(
                Operand::RegisterIndirect(cpu::Register16::HL),
                Operand::Register8(_),
            ) => 2,
            Instruction::Ld(Operand::Register8(cpu::Register8::IXL), Operand::Immediate8(_)) => 3,
            Instruction::Ld(Operand::Register8(cpu::Register8::IXH), Operand::Immediate8(_)) => 3,
            Instruction::Ld(Operand::Register8(cpu::Register8::IYL), Operand::Immediate8(_)) => 3,
            Instruction::Ld(Operand::Register8(cpu::Register8::IYH), Operand::Immediate8(_)) => 3,
            Instruction::Ld(Operand::Register8(_), Operand::Immediate8(_)) => 2,
            Instruction::Ld(
                Operand::RegisterIndirect(cpu::Register16::HL),
                Operand::Immediate8(_),
            ) => 3,
            Instruction::Ld(Operand::Register16(cpu::Register16::IX), Operand::Immediate16(_)) => 4,
            Instruction::Ld(Operand::Register16(cpu::Register16::IY), Operand::Immediate16(_)) => 4,
            Instruction::Ld(Operand::Register16(_), Operand::Immediate16(_)) => 3,
            Instruction::Ld(Operand::Register8(cpu::Register8::A), Operand::Direct16(_)) => 4,
            Instruction::Ld(Operand::Direct16(_), Operand::Register8(cpu::Register8::A)) => 4,
            Instruction::Ld(Operand::Register16(cpu::Register16::HL), Operand::Direct16(_)) => 5,
            Instruction::Ld(Operand::Direct16(_), Operand::Register16(cpu::Register16::HL)) => 5,
            Instruction::Ld(Operand::Register16(cpu::Register16::IX), Operand::Direct16(_)) => 5,
            Instruction::Ld(Operand::Direct16(_), Operand::Register16(cpu::Register16::IX)) => 5,
            Instruction::Ld(Operand::Register16(cpu::Register16::IY), Operand::Direct16(_)) => 5,
            Instruction::Ld(Operand::Direct16(_), Operand::Register16(cpu::Register16::IY)) => 5,
            Instruction::Ld(Operand::Register8(_), Operand::Indexed(_, _)) => 5,
            Instruction::Ld(Operand::Indexed(_, _), Operand::Register8(_)) => 5,
            Instruction::Ld(Operand::Indexed(_, _), Operand::Immediate8(_)) => 6,
            Instruction::Ld(Operand::Register16(cpu::Register16::SP), Operand::Register16(cpu::Register16::HL)) => 2,
            Instruction::Ld(_, _) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::LdDirect16(
                Operand::Register16(cpu::Register16::IX),
                Operand::Direct16(_),
            ) => 6,
            Instruction::LdDirect16(
                Operand::Direct16(_),
                Operand::Register16(cpu::Register16::IX),
            ) => 6,
            Instruction::LdDirect16(
                Operand::Register16(cpu::Register16::IY),
                Operand::Direct16(_),
            ) => 6,
            Instruction::LdDirect16(
                Operand::Direct16(_),
                Operand::Register16(cpu::Register16::IY),
            ) => 6,
            Instruction::LdDirect16(Operand::Register16(_), Operand::Direct16(_)) => 6,
            Instruction::LdDirect16(Operand::Direct16(_), Operand::Register16(_)) => 6,
            Instruction::LdDirect16(_, _) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::Ldd => 5,
            Instruction::Lddr => 6,
            Instruction::Ldi => 5,
            Instruction::Ldir => 6,
            Instruction::Neg => 2,
            Instruction::Nop => 4,
            Instruction::Or(Operand::Register8(cpu::Register8::IXH)) => 2,
            Instruction::Or(Operand::Register8(cpu::Register8::IXL)) => 2,
            Instruction::Or(Operand::Register8(cpu::Register8::IYH)) => 2,
            Instruction::Or(Operand::Register8(cpu::Register8::IYL)) => 2,
            Instruction::Or(Operand::Register8(_)) => 1,
            Instruction::Or(Operand::Immediate8(_)) => 2,
            Instruction::Or(Operand::RegisterIndirect(cpu::Register16::HL)) => 2,
            Instruction::Or(Operand::Indexed(_, _)) => 5,
            Instruction::Or(_) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::Out(Operand::Direct8(_), Operand::Register8(cpu::Register8::A)) => 3,
            Instruction::Out(
                Operand::RegisterIndirect(cpu::Register16::BC),
                Operand::Register8(_),
            ) => 4,
            Instruction::Out(
                Operand::RegisterIndirect(cpu::Register16::BC),
                Operand::Immediate8(_),
            ) => 4,
            Instruction::Out(_, _) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::Otdr => 6,
            Instruction::Otir => 6,
            Instruction::Outd => 5,
            Instruction::Outi => 5,
            Instruction::Pop(Operand::Register16(cpu::Register16::IX)) => 5,
            Instruction::Pop(Operand::Register16(cpu::Register16::IY)) => 5,
            Instruction::Pop(Operand::Register16(_)) => 3,
            Instruction::Pop(_) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::Push(Operand::Register16(cpu::Register16::IX)) => 5,
            Instruction::Push(Operand::Register16(cpu::Register16::IY)) => 5,
            Instruction::Push(Operand::Register16(_)) => 4,
            Instruction::Push(_) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::Res(_, _, Operand::Register8(_)) => 2,
            Instruction::Res(_, _, Operand::RegisterIndirect(cpu::Register16::HL)) => 4,
            Instruction::Res(_, _, Operand::Indexed(_, _)) => 7,
            Instruction::Res(_, _, _) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::Ret(JumpTest::Unconditional) => 3,
            Instruction::Ret(JumpTest::NonZero) => 2,
            Instruction::Ret(JumpTest::Zero) => 4,
            Instruction::Ret(JumpTest::NoCarry) => 2,
            Instruction::Ret(JumpTest::Carry) => 4,
            Instruction::Ret(JumpTest::ParityOdd) => 2,
            Instruction::Ret(JumpTest::ParityEven) => 4,
            Instruction::Ret(JumpTest::SignPositive) => 2,
            Instruction::Ret(JumpTest::SignNegative) => 4,
            Instruction::Reti => 4,
            Instruction::Retn => 4,
            Instruction::Rl(_, Operand::Register8(_)) => 2,
            Instruction::Rl(_, Operand::RegisterIndirect(cpu::Register16::HL)) => 4,
            Instruction::Rl(_, Operand::Indexed(_, _)) => 7,
            Instruction::Rl(_, _) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::Rla => 1,
            Instruction::Rlc(_, Operand::Register8(_)) => 2,
            Instruction::Rlc(_, Operand::RegisterIndirect(cpu::Register16::HL)) => 4,
            Instruction::Rlc(_, Operand::Indexed(_, _)) => 7,
            Instruction::Rlc(_, _) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::Rlca => 1,
            Instruction::Rld => 5,
            Instruction::Rr(_, Operand::Register8(_)) => 2,
            Instruction::Rr(_, Operand::RegisterIndirect(cpu::Register16::HL)) => 4,
            Instruction::Rr(_, Operand::Indexed(_, _)) => 7,
            Instruction::Rr(_, _) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::Rra => 1,
            Instruction::Rrc(_, Operand::Register8(_)) => 2,
            Instruction::Rrc(_, Operand::RegisterIndirect(cpu::Register16::HL)) => 4,
            Instruction::Rrc(_, Operand::Indexed(_, _)) => 7,
            Instruction::Rrc(_, _) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::Rrca => 1,
            Instruction::Rrd => 5,
            Instruction::Rst(Operand::Immediate8(_)) => 4,
            Instruction::Rst(_) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::Sbc(
                Operand::Register8(cpu::Register8::A),
                Operand::Register8(cpu::Register8::IXH),
            ) => 2,
            Instruction::Sbc(
                Operand::Register8(cpu::Register8::A),
                Operand::Register8(cpu::Register8::IXL),
            ) => 2,
            Instruction::Sbc(
                Operand::Register8(cpu::Register8::A),
                Operand::Register8(cpu::Register8::IYH),
            ) => 2,
            Instruction::Sbc(
                Operand::Register8(cpu::Register8::A),
                Operand::Register8(cpu::Register8::IYL),
            ) => 2,
            Instruction::Sbc(Operand::Register8(cpu::Register8::A), Operand::Register8(_)) => 1,
            Instruction::Sbc(Operand::Register8(cpu::Register8::A), Operand::Immediate8(_)) => 2,
            Instruction::Sbc(
                Operand::Register8(cpu::Register8::A),
                Operand::RegisterIndirect(cpu::Register16::HL),
            ) => 2,
            Instruction::Sbc(Operand::Register8(cpu::Register8::A), Operand::Indexed(_, _)) => 5,
            Instruction::Sbc(Operand::Register16(cpu::Register16::HL), Operand::Register16(_)) => 4,
            Instruction::Sbc(_, _) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::Scf => 1,
            Instruction::Set(_, _, Operand::Register8(_)) => 2,
            Instruction::Set(_, _, Operand::RegisterIndirect(cpu::Register16::HL)) => 4,
            Instruction::Set(_, _, Operand::Indexed(_, _)) => 7,
            Instruction::Set(_, _, _) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::Sla(_, Operand::Register8(_)) => 2,
            Instruction::Sla(_, Operand::RegisterIndirect(cpu::Register16::HL)) => 4,
            Instruction::Sla(_, Operand::Indexed(_, _)) => 7,
            Instruction::Sla(_, _) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::Sll(_, Operand::Register8(_)) => 2,
            Instruction::Sll(_, Operand::RegisterIndirect(cpu::Register16::HL)) => 4,
            Instruction::Sll(_, Operand::Indexed(_, _)) => 7,
            Instruction::Sll(_, _) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::Sra(_, Operand::Register8(_)) => 2,
            Instruction::Sra(_, Operand::RegisterIndirect(cpu::Register16::HL)) => 4,
            Instruction::Sra(_, Operand::Indexed(_, _)) => 7,
            Instruction::Sra(_, _) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::Srl(_, Operand::Register8(_)) => 2,
            Instruction::Srl(_, Operand::RegisterIndirect(cpu::Register16::HL)) => 4,
            Instruction::Srl(_, Operand::Indexed(_, _)) => 7,
            Instruction::Srl(_, _) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::Sub(Operand::Register8(cpu::Register8::IXH)) => 2,
            Instruction::Sub(Operand::Register8(cpu::Register8::IXL)) => 2,
            Instruction::Sub(Operand::Register8(cpu::Register8::IYH)) => 2,
            Instruction::Sub(Operand::Register8(cpu::Register8::IYL)) => 2,
            Instruction::Sub(Operand::Register8(_)) => 1,
            Instruction::Sub(Operand::Immediate8(_)) => 2,
            Instruction::Sub(Operand::RegisterIndirect(cpu::Register16::HL)) => 2,
            Instruction::Sub(Operand::Indexed(_, _)) => 5,
            Instruction::Sub(_) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
            Instruction::Xor(Operand::Register8(cpu::Register8::IXH)) => 2,
            Instruction::Xor(Operand::Register8(cpu::Register8::IXL)) => 2,
            Instruction::Xor(Operand::Register8(cpu::Register8::IYH)) => 2,
            Instruction::Xor(Operand::Register8(cpu::Register8::IYL)) => 2,
            Instruction::Xor(Operand::Register8(_)) => 1,
            Instruction::Xor(Operand::Immediate8(_)) => 2,
            Instruction::Xor(Operand::RegisterIndirect(cpu::Register16::HL)) => 2,
            Instruction::Xor(Operand::Indexed(_, _)) => 5,
            Instruction::Xor(_) => {
                println!("No timing for {}", self);
                unimplemented!()
            }
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Adc(destination, value) => write!(f, "adc {},{}", destination, value),
            Instruction::Add(destination, value) => write!(f, "add {},{}", destination, value),
            Instruction::And(value) => write!(f, "and {}", value),
            Instruction::Bit(bit, operand) => write!(f, "bit {},{}", bit, operand),
            Instruction::Call(jump_test, target) => match jump_test {
                JumpTest::Unconditional => write!(f, "call {}", target),
                _ => write!(f, "call {},{}", jump_test, target),
            },
            Instruction::Ccf => write!(f, "ccf"),
            Instruction::Cp(value) => write!(f, "cp {}", value),
            Instruction::Cpd => write!(f, "cpd"),
            Instruction::Cpdr => write!(f, "cpdr"),
            Instruction::Cpi => write!(f, "cpi"),
            Instruction::Cpir => write!(f, "cpir"),
            Instruction::Cpl => write!(f, "cpl"),
            Instruction::Daa => write!(f, "daa"),
            Instruction::Dec(destination) => write!(f, "dec {}", destination),
            Instruction::Defb(value) => write!(f, "defb {}", value),
            Instruction::Defw(value) => write!(f, "defw {}", value),
            Instruction::Di => write!(f, "di"),
            Instruction::Djnz(target) => write!(f, "djnz {}", target),
            Instruction::Ei => write!(f, "ei"),
            Instruction::Ex(Operand::Register16(cpu::Register16::AF), _) => write!(f, "ex af,af'"),
            Instruction::Ex(left, right) => write!(f, "ex {},{}", left, right),
            Instruction::Exx => write!(f, "exx"),
            Instruction::Halt => write!(f, "halt"),
            Instruction::Im(mode) => match mode {
                InterruptMode::Mode0 => write!(f, "im 0"),
                InterruptMode::Mode1 => write!(f, "im 1"),
                InterruptMode::Mode2 => write!(f, "im 2"),
            },
            Instruction::In(destination, port) => write!(f, "in {},{}", destination, port), // TODO: account for special case where (BC) is printed as (C)
            Instruction::Inc(destination) => write!(f, "inc {}", destination),
            Instruction::Ind => write!(f, "ind"),
            Instruction::Indr => write!(f, "indr"),
            Instruction::Ini => write!(f, "ini"),
            Instruction::Inir => write!(f, "inir"),
            Instruction::Jp(jump_test, target) => match jump_test {
                JumpTest::Unconditional => write!(f, "jp {}", target),
                _ => write!(f, "jp {},{}", jump_test, target),
            },
            Instruction::Jr(jump_test, target) => match jump_test {
                JumpTest::Unconditional => write!(f, "jr {}", target),
                _ => write!(f, "jr {},{}", jump_test, target),
            },
            Instruction::Ld(destination, source) => write!(f, "ld {},{}", destination, source),
            Instruction::LdDirect16(destination, source) => {
                write!(f, "ld {},{}", destination, source)
            }
            Instruction::Ldd => write!(f, "ldd"),
            Instruction::Lddr => write!(f, "lddr"),
            Instruction::Ldi => write!(f, "ldi"),
            Instruction::Ldir => write!(f, "ldir"),
            Instruction::Neg => write!(f, "neg"),
            Instruction::Nop => write!(f, "nop"),
            Instruction::Or(value) => write!(f, "or {}", value),
            Instruction::Out(port, source) => write!(f, "out {},{}", port, source), // TODO: account for special case where (BC) is printed as (C)
            Instruction::Otdr => write!(f, "otdr"),
            Instruction::Otir => write!(f, "otir"),
            Instruction::Outd => write!(f, "outd"),
            Instruction::Outi => write!(f, "outi"),
            Instruction::Pop(destination) => write!(f, "pop {}", destination),
            Instruction::Push(source) => write!(f, "push {}", source),
            Instruction::Res(destination, bit, operand) => match operand {
                Operand::Indexed(_, _) => match destination {
                    Operand::Indexed(_, _) => write!(f, "res {},{}", bit, operand),
                    _ => write!(f, "res {},{}->{}", bit, operand, destination),
                },
                _ => write!(f, "res {},{}", bit, operand),
            },
            Instruction::Ret(jump_test) => match jump_test {
                JumpTest::Unconditional => write!(f, "ret"),
                _ => write!(f, "ret {}", jump_test),
            },
            Instruction::Reti => write!(f, "reti"),
            Instruction::Retn => write!(f, "retn"),
            Instruction::Rl(destination, operand) => match operand {
                Operand::Indexed(_, _) => match destination {
                    Operand::Indexed(_, _) => write!(f, "rl {}", operand),
                    _ => write!(f, "rl {}->{}", operand, destination),
                },
                _ => write!(f, "rl {}", operand),
            },
            Instruction::Rla => write!(f, "rla"),
            Instruction::Rlc(destination, operand) => match operand {
                // TODO: extract this into reusable method. how to handle helpers in traits?
                Operand::Indexed(_, _) => match destination {
                    Operand::Indexed(_, _) => write!(f, "rlc {}", operand),
                    _ => write!(f, "rlc {}->{}", operand, destination),
                },
                _ => write!(f, "rlc {}", operand),
            },
            Instruction::Rlca => write!(f, "rlca"),
            Instruction::Rld => write!(f, "rld"),
            Instruction::Rr(destination, operand) => match operand {
                Operand::Indexed(_, _) => match destination {
                    Operand::Indexed(_, _) => write!(f, "rr {}", operand),
                    _ => write!(f, "rr {}->{}", operand, destination),
                },
                _ => write!(f, "rr {}", operand),
            },
            Instruction::Rrc(destination, operand) => match operand {
                Operand::Indexed(_, _) => match destination {
                    Operand::Indexed(_, _) => write!(f, "rrc {}", operand),
                    _ => write!(f, "rrc {}->{}", operand, destination),
                },
                _ => write!(f, "rrc {}", operand),
            },
            Instruction::Rra => write!(f, "rra"),
            Instruction::Rrca => write!(f, "rrca"),
            Instruction::Rrd => write!(f, "rrd"),
            Instruction::Rst(target) => write!(f, "rst {}", target),
            Instruction::Sbc(destination, value) => write!(f, "sbc {},{}", destination, value),
            Instruction::Scf => write!(f, "scf"),
            Instruction::Set(destination, bit, operand) => match operand {
                Operand::Indexed(_, _) => match destination {
                    Operand::Indexed(_, _) => write!(f, "set {},{}", bit, operand),
                    _ => write!(f, "set {},{}->{}", bit, operand, destination),
                },
                _ => write!(f, "set {},{}", bit, operand),
            },
            Instruction::Sla(destination, operand) => match operand {
                Operand::Indexed(_, _) => match destination {
                    Operand::Indexed(_, _) => write!(f, "sla {}", operand),
                    _ => write!(f, "sla {}->{}", operand, destination),
                },
                _ => write!(f, "sla {}", operand),
            },
            Instruction::Sll(destination, operand) => match operand {
                Operand::Indexed(_, _) => match destination {
                    Operand::Indexed(_, _) => write!(f, "sll {}", operand),
                    _ => write!(f, "sll {}->{}", operand, destination),
                },
                _ => write!(f, "sll {}", operand),
            },
            Instruction::Sra(destination, operand) => match operand {
                Operand::Indexed(_, _) => match destination {
                    Operand::Indexed(_, _) => write!(f, "sra {}", operand),
                    _ => write!(f, "sra {}->{}", operand, destination),
                },
                _ => write!(f, "sra {}", operand),
            },
            Instruction::Srl(destination, operand) => match operand {
                Operand::Indexed(_, _) => match destination {
                    Operand::Indexed(_, _) => write!(f, "srl {}", operand),
                    _ => write!(f, "srl {}->{}", operand, destination),
                },
                _ => write!(f, "srl {}", operand),
            },
            Instruction::Sub(value) => write!(f, "sub {}", value),
            Instruction::Xor(value) => write!(f, "xor {}", value),
        }
    }
}

#[derive(Clone, Copy)]
enum DecoderMode {
    Default,
    PatchIX,
    PatchIY,
}

impl DecoderMode {
    fn as_instruction(&self) -> Instruction {
        match self {
            DecoderMode::PatchIX => Instruction::Defb(Operand::Immediate8(0xdd)),
            DecoderMode::PatchIY => Instruction::Defb(Operand::Immediate8(0xfd)),
            _ => unreachable!(),
        }
    }
}

pub struct Decoder<M> {
    memory: Rc<RefCell<M>>,
    next_address: usize,
    mode: DecoderMode,
    patched: bool,
}

impl<M> Decoder<M>
where
    M: Read,
{
    // based on http://z80.info/decoding.htm
    pub fn new(memory: Rc<RefCell<M>>) -> Decoder<M> {
        Decoder {
            memory,
            next_address: 0,
            mode: DecoderMode::Default,
            patched: false,
        }
    }

    pub fn decode_at(&mut self, address: usize) -> (Instruction, usize) {
        self.next_address = address;

        let opcode = self.memory.borrow().read_byte(self.next_address);
        self.next_address += 1;

        match opcode {
            0xcb => (self.decode_cb_instruction(), self.next_address),
            0xed => (self.decode_ed_instruction(), self.next_address),
            0xdd => (
                self.decode_prefixed(DecoderMode::PatchIX),
                self.next_address,
            ),
            0xfd => (
                self.decode_prefixed(DecoderMode::PatchIY),
                self.next_address,
            ),
            _ => (self.decode_instruction(opcode), self.next_address),
        }
    }

    pub fn decode_next(&mut self) -> (Instruction, usize) {
        self.decode_at(self.next_address)
    }

    fn decode_cb_instruction(&mut self) -> Instruction {
        match self.mode {
            DecoderMode::PatchIX => {
                let displacement = self.memory.borrow().read_byte(self.next_address) as i8;
                self.next_address += 1;

                let opcode = self.memory.borrow().read_byte(self.next_address);
                self.next_address += 1;

                let opcode_x = opcode >> 6;
                let opcode_y = (opcode >> 3) & 7;
                let opcode_z = opcode & 7;

                let old_mode = self.mode;
                self.mode = DecoderMode::Default;
                let destination = self.decode_register(opcode_z);
                self.mode = old_mode;

                match (opcode_x, opcode_y, opcode_z) {
                    (0, _, 6) => {
                        let destination = Operand::Indexed(cpu::Register16::IX, displacement);
                        let operand = Operand::Indexed(cpu::Register16::IX, displacement);
                        self.decode_bitshift(opcode_y, destination, operand)
                    }
                    (0, _, _) => {
                        let operand = Operand::Indexed(cpu::Register16::IX, displacement);
                        self.decode_bitshift(opcode_y, destination, operand)
                    }
                    (1, _, _) => {
                        let bit = Operand::Bit(opcode_y);
                        let operand = Operand::Indexed(cpu::Register16::IX, displacement);
                        Instruction::Bit(bit, operand)
                    }
                    (2, _, 6) => {
                        let bit = Operand::Bit(opcode_y);
                        let destination = Operand::Indexed(cpu::Register16::IX, displacement);
                        let operand = Operand::Indexed(cpu::Register16::IX, displacement);
                        Instruction::Res(destination, bit, operand)
                    }
                    (2, _, _) => {
                        let bit = Operand::Bit(opcode_y);
                        let operand = Operand::Indexed(cpu::Register16::IX, displacement);
                        Instruction::Res(destination, bit, operand)
                    }
                    (3, _, 6) => {
                        let bit = Operand::Bit(opcode_y);
                        let destination = Operand::Indexed(cpu::Register16::IX, displacement);
                        let operand = Operand::Indexed(cpu::Register16::IX, displacement);
                        Instruction::Set(destination, bit, operand)
                    }
                    (3, _, _) => {
                        let bit = Operand::Bit(opcode_y);
                        let operand = Operand::Indexed(cpu::Register16::IX, displacement);
                        Instruction::Set(destination, bit, operand)
                    }
                    _ => unreachable!(),
                }
            }
            DecoderMode::PatchIY => {
                let displacement = self.memory.borrow().read_byte(self.next_address) as i8;
                self.next_address += 1;

                let opcode = self.memory.borrow().read_byte(self.next_address);
                self.next_address += 1;

                let opcode_x = opcode >> 6;
                let opcode_y = (opcode >> 3) & 7;
                let opcode_z = opcode & 7;

                let old_mode = self.mode;
                self.mode = DecoderMode::Default;
                let destination = self.decode_register(opcode_z);
                self.mode = old_mode;

                match (opcode_x, opcode_y, opcode_z) {
                    (0, _, 6) => {
                        let destination = Operand::Indexed(cpu::Register16::IY, displacement);
                        let operand = Operand::Indexed(cpu::Register16::IY, displacement);
                        self.decode_bitshift(opcode_y, destination, operand)
                    }
                    (0, _, _) => {
                        let operand = Operand::Indexed(cpu::Register16::IY, displacement);
                        self.decode_bitshift(opcode_y, destination, operand)
                    }
                    (1, _, _) => {
                        let bit = Operand::Bit(opcode_y);
                        let operand = Operand::Indexed(cpu::Register16::IY, displacement);
                        Instruction::Bit(bit, operand)
                    }
                    (2, _, 6) => {
                        let bit = Operand::Bit(opcode_y);
                        let destination = Operand::Indexed(cpu::Register16::IY, displacement);
                        let operand = Operand::Indexed(cpu::Register16::IY, displacement);
                        Instruction::Res(destination, bit, operand)
                    }
                    (2, _, _) => {
                        let bit = Operand::Bit(opcode_y);
                        let operand = Operand::Indexed(cpu::Register16::IY, displacement);
                        Instruction::Res(destination, bit, operand)
                    }
                    (3, _, 6) => {
                        let bit = Operand::Bit(opcode_y);
                        let destination = Operand::Indexed(cpu::Register16::IY, displacement);
                        let operand = Operand::Indexed(cpu::Register16::IY, displacement);
                        Instruction::Set(destination, bit, operand)
                    }
                    (3, _, _) => {
                        let bit = Operand::Bit(opcode_y);
                        let operand = Operand::Indexed(cpu::Register16::IY, displacement);
                        Instruction::Set(destination, bit, operand)
                    }
                    _ => unreachable!(),
                }
            }
            _ => {
                let opcode = self.memory.borrow().read_byte(self.next_address);
                self.next_address += 1;

                let opcode_x = opcode >> 6;
                let opcode_y = (opcode >> 3) & 7;
                let opcode_z = opcode & 7;

                match (opcode_x, opcode_y, opcode_z) {
                    (0, _, _) => {
                        let destination = self.decode_register(opcode_z);
                        let operand = self.decode_register(opcode_z);
                        self.decode_bitshift(opcode_y, destination, operand)
                    }
                    (1, _, _) => {
                        let bit = Operand::Bit(opcode_y);
                        let operand = self.decode_register(opcode_z);

                        Instruction::Bit(bit, operand)
                    }
                    (2, _, _) => {
                        let bit = Operand::Bit(opcode_y);
                        let destination = self.decode_register(opcode_z);
                        let operand = self.decode_register(opcode_z);

                        Instruction::Res(destination, bit, operand)
                    }
                    (3, _, _) => {
                        let bit = Operand::Bit(opcode_y);
                        let destination = self.decode_register(opcode_z);
                        let operand = self.decode_register(opcode_z);

                        Instruction::Set(destination, bit, operand)
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    fn decode_ed_instruction(&mut self) -> Instruction {
        let opcode = self.memory.borrow().read_byte(self.next_address);
        self.next_address += 1;

        let opcode_x = opcode >> 6;
        let opcode_y = (opcode >> 3) & 7;
        let opcode_z = opcode & 7;

        match (opcode_x, opcode_y, opcode_z) {
            (1, _, 0) => {
                let destination = self.decode_register(opcode_y);
                let port = Operand::RegisterIndirect(cpu::Register16::BC);

                Instruction::In(destination, port) // TODO: what if destination is flags register?
            }
            (1, _, 1) => {
                let port = Operand::RegisterIndirect(cpu::Register16::BC);

                if opcode_y == 6 {
                    Instruction::Out(port, Operand::Immediate8(0))
                } else {
                    let source = self.decode_register(opcode_y);
                    Instruction::Out(port, source)
                }
            }
            (1, _, 2) => {
                let opcode_p = opcode_y >> 1;
                let opcode_q = opcode_y & 1;

                let destination = Operand::Register16(cpu::Register16::HL);
                let source = self.decode_register_pair(opcode_p, false);

                match opcode_q {
                    0 => Instruction::Sbc(destination, source),
                    1 => Instruction::Adc(destination, source),
                    _ => unreachable!(),
                }
            }
            (1, _, 3) => {
                let opcode_p = opcode_y >> 1;
                let opcode_q = opcode_y & 1;

                let address = Operand::Direct16(self.memory.borrow().read_word(self.next_address));
                self.next_address += 2;

                let register = self.decode_register_pair(opcode_p, false);

                match opcode_q {
                    0 => Instruction::LdDirect16(address, register), // TODO: does this include LD IX,(nnnn)?
                    1 => Instruction::LdDirect16(register, address),
                    _ => unreachable!(),
                }
            }
            (1, _, 4) => Instruction::Neg,
            (1, _, 5) => match opcode_y {
                1 => Instruction::Reti,
                _ => Instruction::Retn,
            },
            (1, _, 6) => {
                let mode = match opcode_y {
                    0 => InterruptMode::Mode0,
                    1 => InterruptMode::Mode0, // TODO: verify this. could be mode 1
                    2 => InterruptMode::Mode1,
                    3 => InterruptMode::Mode2,
                    4 => InterruptMode::Mode0,
                    5 => InterruptMode::Mode0, // TODO: verify this. could be mode 1
                    6 => InterruptMode::Mode1,
                    7 => InterruptMode::Mode2,
                    _ => unreachable!(),
                };

                Instruction::Im(mode)
            }
            (1, _, 7) => match opcode_y {
                0 => Instruction::Ld(
                    Operand::Register8(cpu::Register8::I),
                    Operand::Register8(cpu::Register8::A),
                ),
                1 => Instruction::Ld(
                    Operand::Register8(cpu::Register8::R),
                    Operand::Register8(cpu::Register8::A),
                ),
                2 => Instruction::Ld(
                    Operand::Register8(cpu::Register8::A),
                    Operand::Register8(cpu::Register8::I),
                ),
                3 => Instruction::Ld(
                    Operand::Register8(cpu::Register8::A),
                    Operand::Register8(cpu::Register8::R),
                ),
                4 => Instruction::Rrd,
                5 => Instruction::Rld,
                6 => Instruction::Nop,
                7 => Instruction::Nop,
                _ => unreachable!(),
            },
            (2, _, _) => match opcode_z {
                0 => match opcode_y {
                    4 => Instruction::Ldi,
                    5 => Instruction::Ldd,
                    6 => Instruction::Ldir,
                    7 => Instruction::Lddr,
                    _ => {
                        Instruction::Defw(Operand::Immediate16(u16::from_le_bytes([0xed, opcode])))
                    }
                },
                1 => match opcode_y {
                    4 => Instruction::Cpi,
                    5 => Instruction::Cpd,
                    6 => Instruction::Cpir,
                    7 => Instruction::Cpdr,
                    _ => {
                        Instruction::Defw(Operand::Immediate16(u16::from_le_bytes([0xed, opcode])))
                    }
                },
                2 => match opcode_y {
                    4 => Instruction::Ini,
                    5 => Instruction::Ind,
                    6 => Instruction::Inir,
                    7 => Instruction::Indr,
                    _ => {
                        Instruction::Defw(Operand::Immediate16(u16::from_le_bytes([0xed, opcode])))
                    }
                },
                3 => match opcode_y {
                    4 => Instruction::Outi,
                    5 => Instruction::Outd,
                    6 => Instruction::Otir,
                    7 => Instruction::Otdr,
                    _ => {
                        Instruction::Defw(Operand::Immediate16(u16::from_le_bytes([0xed, opcode])))
                    }
                },
                _ => Instruction::Defw(Operand::Immediate16(u16::from_le_bytes([0xed, opcode]))),
            },
            _ => Instruction::Defw(Operand::Immediate16(u16::from_le_bytes([0xed, opcode]))),
        }
    }

    fn decode_prefixed(&mut self, mode: DecoderMode) -> Instruction {
        self.mode = mode;

        let opcode = self.memory.borrow().read_byte(self.next_address);

        let instruction = match opcode {
            0xcb => {
                self.next_address += 1;
                self.decode_cb_instruction()
            }
            0xed => self.mode.as_instruction(),
            0xdd => self.mode.as_instruction(),
            0xfd => self.mode.as_instruction(),
            _ => {
                self.patched = false;

                let start = self.next_address;
                self.next_address += 1;
                let instruction = self.decode_instruction(opcode);

                if self.patched {
                    instruction
                } else {
                    self.next_address = start;
                    self.mode.as_instruction()
                }
            }
        };

        self.mode = DecoderMode::Default;

        instruction
    }

    fn decode_instruction(&mut self, opcode: u8) -> Instruction {
        let opcode_x = opcode >> 6;
        let opcode_y = (opcode >> 3) & 7;
        let opcode_z = opcode & 7;

        match (opcode_x, opcode_y, opcode_z) {
            (0, 0, 0) => Instruction::Nop,
            (0, 1, 0) => Instruction::Ex(
                Operand::Register16(cpu::Register16::AF),
                // this instruction swaps AF and AF', but using AF below uniquely identifies the instruction
                Operand::Register16(cpu::Register16::AF),
            ),
            (0, 2, 0) => {
                let displacement = self.memory.borrow().read_byte(self.next_address) as i8;
                self.next_address += 1;
                Instruction::Djnz(Operand::Immediate16(
                    (self.next_address as u16).wrapping_add(displacement as u16),
                ))
            }
            (0, _, 0) => {
                let jump_test = if opcode_y == 3 {
                    JumpTest::Unconditional
                } else {
                    JumpTest::decode(opcode_y - 4)
                };
                let displacement = self.memory.borrow().read_byte(self.next_address) as i8;
                self.next_address += 1;
                Instruction::Jr(
                    jump_test,
                    Operand::Immediate16(
                        (self.next_address as u16).wrapping_add(displacement as u16),
                    ),
                )
            }
            (0, _, 1) => {
                let opcode_p = opcode_y >> 1;
                let opcode_q = opcode_y & 1;

                let register_pair = self.decode_register_pair(opcode_p, false);

                match opcode_q {
                    0 => {
                        let value =
                            Operand::Immediate16(self.memory.borrow().read_word(self.next_address));
                        self.next_address += 2;
                        Instruction::Ld(register_pair, value)
                    }
                    1 => {
                        let destination = match self.mode {
                            DecoderMode::PatchIX => {
                                self.patched = true;
                                Operand::Register16(cpu::Register16::IX)
                            }
                            DecoderMode::PatchIY => {
                                self.patched = true;
                                Operand::Register16(cpu::Register16::IY)
                            }
                            _ => Operand::Register16(cpu::Register16::HL),
                        };
                        Instruction::Add(destination, register_pair)
                    }
                    _ => unreachable!(),
                }
            }
            (0, _, 2) => {
                let opcode_p = opcode_y >> 1;
                let opcode_q = opcode_y & 1;

                let address = match opcode_p {
                    0 => Operand::RegisterIndirect(cpu::Register16::BC),
                    1 => Operand::RegisterIndirect(cpu::Register16::DE),
                    _ => {
                        let address = self.memory.borrow().read_word(self.next_address);
                        self.next_address += 2;
                        Operand::Direct16(address)
                    }
                };

                let register = match opcode_p {
                    2 => match self.mode {
                        DecoderMode::PatchIX => {
                            self.patched = true;
                            Operand::Register16(cpu::Register16::IX)
                        }
                        DecoderMode::PatchIY => {
                            self.patched = true;
                            Operand::Register16(cpu::Register16::IY)
                        }
                        _ => Operand::Register16(cpu::Register16::HL),
                    },
                    _ => Operand::Register8(cpu::Register8::A),
                };

                match opcode_q {
                    0 => match register {
                        Operand::Register8(cpu::Register8::A) => Instruction::Ld(address, register),
                        _ => Instruction::Ld(address, register),
                    },
                    1 => match register {
                        Operand::Register8(cpu::Register8::A) => Instruction::Ld(register, address),
                        _ => Instruction::Ld(register, address),
                    },
                    _ => unreachable!(),
                }
            }
            (0, _, 3) => {
                let opcode_p = opcode_y >> 1;
                let opcode_q = opcode_y & 1;

                let register_pair = self.decode_register_pair(opcode_p, false);

                match opcode_q {
                    0 => Instruction::Inc(register_pair),
                    1 => Instruction::Dec(register_pair),
                    _ => unreachable!(),
                }
            }
            (0, _, 4) => {
                let register = self.decode_register(opcode_y);
                Instruction::Inc(register)
            }
            (0, _, 5) => {
                let register = self.decode_register(opcode_y);
                Instruction::Dec(register)
            }
            (0, _, 6) => {
                let register = self.decode_register(opcode_y);

                let value = self.memory.borrow().read_byte(self.next_address);
                self.next_address += 1;

                Instruction::Ld(register, Operand::Immediate8(value))
            }
            (0, _, 7) => match opcode_y {
                0 => Instruction::Rlca,
                1 => Instruction::Rrca,
                2 => Instruction::Rla,
                3 => Instruction::Rra,
                4 => Instruction::Daa,
                5 => Instruction::Cpl,
                6 => Instruction::Scf,
                7 => Instruction::Ccf,
                _ => unreachable!(),
            },
            (1, 6, 6) => Instruction::Halt,
            (1, 6, _) => {
                let destination = self.decode_register(opcode_y);

                let old_mode = self.mode;
                self.mode = DecoderMode::Default;
                let source = self.decode_register(opcode_z);
                self.mode = old_mode;

                Instruction::Ld(destination, source)
            }
            (1, _, 6) => {
                let old_mode = self.mode;
                self.mode = DecoderMode::Default;
                let destination = self.decode_register(opcode_y);
                self.mode = old_mode;

                let source = self.decode_register(opcode_z);

                Instruction::Ld(destination, source)
            }
            (1, _, _) => {
                let destination = self.decode_register(opcode_y);
                let source = self.decode_register(opcode_z);
                Instruction::Ld(destination, source)
            }
            (2, _, _) => {
                let register = self.decode_register(opcode_z);
                self.decode_alu(opcode_y, register)
            }
            (3, _, 0) => {
                let jump_test = JumpTest::decode(opcode_y);
                Instruction::Ret(jump_test)
            }
            (3, _, 1) => {
                let opcode_p = opcode_y >> 1;
                let opcode_q = opcode_y & 1;

                match opcode_q {
                    0 => {
                        let register_pair = self.decode_register_pair(opcode_p, true);
                        Instruction::Pop(register_pair)
                    }
                    1 => match opcode_p {
                        0 => Instruction::Ret(JumpTest::Unconditional),
                        1 => Instruction::Exx,
                        2 => Instruction::Jp(
                            JumpTest::Unconditional,
                            Operand::RegisterIndirect(cpu::Register16::HL),
                        ), // TODO: decode Jp (IX) as well
                        3 => Instruction::Ld(
                            Operand::Register16(cpu::Register16::SP),
                            Operand::Register16(cpu::Register16::HL),
                        ),
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            }
            (3, _, 2) => {
                let jump_test = JumpTest::decode(opcode_y);

                let target =
                    Operand::Immediate16(self.memory.borrow().read_word(self.next_address));
                self.next_address += 2;

                Instruction::Jp(jump_test, target)
            }
            (3, _, 3) => match opcode_y {
                0 => {
                    let target =
                        Operand::Immediate16(self.memory.borrow().read_word(self.next_address));
                    self.next_address += 2;

                    Instruction::Jp(JumpTest::Unconditional, target)
                }
                1 => unreachable!(),
                2 => {
                    let port = Operand::Direct8(self.memory.borrow().read_byte(self.next_address));
                    self.next_address += 1;

                    Instruction::Out(port, Operand::Register8(cpu::Register8::A))
                }
                3 => {
                    let port = Operand::Direct8(self.memory.borrow().read_byte(self.next_address));
                    self.next_address += 1;

                    Instruction::In(Operand::Register8(cpu::Register8::A), port)
                }
                4 => {
                    let right = match self.mode {
                        DecoderMode::PatchIX => {
                            self.patched = true;
                            Operand::Register16(cpu::Register16::IX)
                        }
                        DecoderMode::PatchIY => {
                            self.patched = true;
                            Operand::Register16(cpu::Register16::IY)
                        }
                        _ => Operand::Register16(cpu::Register16::HL),
                    };
                    Instruction::Ex(Operand::RegisterIndirect(cpu::Register16::SP), right)
                }
                5 => Instruction::Ex(
                    Operand::Register16(cpu::Register16::DE),
                    Operand::Register16(cpu::Register16::HL),
                ),
                6 => Instruction::Di,
                7 => Instruction::Ei,
                _ => unreachable!(),
            },
            (3, _, 4) => {
                let jump_test = JumpTest::decode(opcode_y);

                let target =
                    Operand::Immediate16(self.memory.borrow().read_word(self.next_address));
                self.next_address += 2;

                Instruction::Call(jump_test, target)
            }
            (3, _, 5) => {
                let opcode_p = opcode_y >> 1;
                let opcode_q = opcode_y & 1;

                match opcode_q {
                    0 => {
                        let source = self.decode_register_pair(opcode_p, true);
                        Instruction::Push(source)
                    }
                    1 => {
                        if opcode_p == 0 {
                            let target = Operand::Immediate16(
                                self.memory.borrow().read_word(self.next_address),
                            );
                            self.next_address += 2;

                            Instruction::Call(JumpTest::Unconditional, target)
                        } else {
                            unreachable!()
                        }
                    }
                    _ => unreachable!(),
                }
            }
            (3, _, 6) => {
                let value = Operand::Immediate8(self.memory.borrow().read_byte(self.next_address));
                self.next_address += 1;

                self.decode_alu(opcode_y, value)
            }
            (3, _, 7) => Instruction::Rst(Operand::Immediate8(opcode_y * 8)),
            _ => panic!("Illegal instruction: {:x}", opcode), // TODO: id instruction more accurately
        }
    }

    fn decode_alu(&self, encoded: u8, operand: Operand) -> Instruction {
        match encoded {
            0 => Instruction::Add(Operand::Register8(cpu::Register8::A), operand),
            1 => Instruction::Adc(Operand::Register8(cpu::Register8::A), operand),
            2 => Instruction::Sub(operand),
            3 => Instruction::Sbc(Operand::Register8(cpu::Register8::A), operand),
            4 => Instruction::And(operand),
            5 => Instruction::Xor(operand),
            6 => Instruction::Or(operand),
            7 => Instruction::Cp(operand),
            _ => unreachable!(),
        }
    }

    fn decode_bitshift(&self, encoded: u8, destination: Operand, operand: Operand) -> Instruction {
        match encoded {
            0 => Instruction::Rlc(destination, operand),
            1 => Instruction::Rrc(destination, operand),
            2 => Instruction::Rl(destination, operand),
            3 => Instruction::Rr(destination, operand),
            4 => Instruction::Sla(destination, operand),
            5 => Instruction::Sra(destination, operand),
            6 => Instruction::Sll(destination, operand),
            7 => Instruction::Srl(destination, operand),
            _ => unreachable!(),
        }
    }

    fn decode_register(&mut self, encoded: u8) -> Operand {
        match encoded {
            0 => Operand::Register8(cpu::Register8::B),
            1 => Operand::Register8(cpu::Register8::C),
            2 => Operand::Register8(cpu::Register8::D),
            3 => Operand::Register8(cpu::Register8::E),
            4 => match self.mode {
                DecoderMode::PatchIX => {
                    self.patched = true;
                    Operand::Register8(cpu::Register8::IXH)
                }
                DecoderMode::PatchIY => {
                    self.patched = true;
                    Operand::Register8(cpu::Register8::IYH)
                }
                _ => Operand::Register8(cpu::Register8::H),
            },
            5 => match self.mode {
                DecoderMode::PatchIX => {
                    self.patched = true;
                    Operand::Register8(cpu::Register8::IXL)
                }
                DecoderMode::PatchIY => {
                    self.patched = true;
                    Operand::Register8(cpu::Register8::IYL)
                }
                _ => Operand::Register8(cpu::Register8::L),
            },
            6 => match self.mode {
                DecoderMode::PatchIX => {
                    self.patched = true;
                    let displacement = self.memory.borrow().read_byte(self.next_address) as i8;
                    self.next_address += 1;
                    Operand::Indexed(cpu::Register16::IX, displacement)
                }
                DecoderMode::PatchIY => {
                    self.patched = true;
                    let displacement = self.memory.borrow().read_byte(self.next_address) as i8;
                    self.next_address += 1;
                    Operand::Indexed(cpu::Register16::IY, displacement)
                }
                _ => Operand::RegisterIndirect(cpu::Register16::HL),
            },
            7 => Operand::Register8(cpu::Register8::A),
            _ => unreachable!(),
        }
    }

    fn decode_register_pair(&mut self, encoded: u8, alternate: bool) -> Operand {
        match encoded {
            0 => Operand::Register16(cpu::Register16::BC),
            1 => Operand::Register16(cpu::Register16::DE),
            2 => match self.mode {
                DecoderMode::PatchIX => {
                    self.patched = true;
                    Operand::Register16(cpu::Register16::IX)
                }
                DecoderMode::PatchIY => {
                    self.patched = true;
                    Operand::Register16(cpu::Register16::IY)
                }
                _ => Operand::Register16(cpu::Register16::HL),
            },
            3 => {
                if alternate {
                    Operand::Register16(cpu::Register16::AF)
                } else {
                    Operand::Register16(cpu::Register16::SP)
                }
            }
            _ => unreachable!(),
        }
    }
}
