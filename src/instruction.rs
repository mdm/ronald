use std::fmt;

use crate::memory::{Memory, Read};
use crate::cpu;


enum Prefix {
    None,
    CB,
    ED,
    DD,
    FD,
    DDCB(u8),
    FDCB(u8),
}

enum Operand {
    Immediate8(u8),
    Immediate16(u16),
    Register(cpu::Register),
    RegisterIndirect(cpu::Register),
    Direct8(u8),
    Direct16(u16),
    Indexed(cpu::Register, i8),
    Bit(u8), // TODO: use Immediate8? or rename to Immediate1?
}

impl Operand {
    fn decode_register(encoded: u8) -> Operand {
        match encoded {
            0 => Operand::Register(cpu::Register::B),
            1 => Operand::Register(cpu::Register::C),
            2 => Operand::Register(cpu::Register::D),
            3 => Operand::Register(cpu::Register::E),
            4 => Operand::Register(cpu::Register::H),
            5 => Operand::Register(cpu::Register::L),
            6 => Operand::RegisterIndirect(cpu::Register::HL),
            7 => Operand::Register(cpu::Register::A),
            _ => unreachable!(),
        }
    }

    fn decode_register_pair(encoded: u8, alternate: bool) -> Operand {
        match encoded {
            0 => Operand::Register(cpu::Register::BC),
            1 => Operand::Register(cpu::Register::DE),
            2 => Operand::Register(cpu::Register::HL),
            3 => if alternate {
                Operand::Register(cpu::Register::AF)
            } else {
                Operand::Register(cpu::Register::SP)
            }
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Immediate8(value) => write!(f, "{:#04x}", value),
            Operand::Immediate16(value) => write!(f, "{:#06x}", value),
            Operand::Register(register) => match register { // TODO: remove unused match arms
                cpu::Register::A => write!(f, "a"),
                cpu::Register::F => write!(f, "f"),
                cpu::Register::B => write!(f, "b"),
                cpu::Register::C => write!(f, "c"),
                cpu::Register::D => write!(f, "d"),
                cpu::Register::E => write!(f, "e"),
                cpu::Register::H => write!(f, "h"),
                cpu::Register::L => write!(f, "l"),
                cpu::Register::AF => write!(f, "af"),
                cpu::Register::BC => write!(f, "bc"),
                cpu::Register::DE => write!(f, "de"),
                cpu::Register::HL => write!(f, "hl"),
                cpu::Register::I => write!(f, "i"),
                cpu::Register::R => write!(f, "r"),
                cpu::Register::IX => write!(f, "ix"),
                cpu::Register::IXH => write!(f, "ixh"),
                cpu::Register::IXL => write!(f, "ixl"),
                cpu::Register::IY => write!(f, "iy"),
                cpu::Register::IYH => write!(f, "iyh"),
                cpu::Register::IYL => write!(f, "iyl"),
                cpu::Register::SP => write!(f, "sp"),
                cpu::Register::PC => write!(f, "pc"),
            }
            Operand::RegisterIndirect(register) => match register { // TODO: remove unused match arms
                cpu::Register::A => write!(f, "(a)"),
                cpu::Register::F => write!(f, "(f)"),
                cpu::Register::B => write!(f, "(b)"),
                cpu::Register::C => write!(f, "(c)"),
                cpu::Register::D => write!(f, "(d)"),
                cpu::Register::E => write!(f, "(e)"),
                cpu::Register::H => write!(f, "(h)"),
                cpu::Register::L => write!(f, "(l)"),
                cpu::Register::AF => write!(f, "(af)"),
                cpu::Register::BC => write!(f, "(bc)"),
                cpu::Register::DE => write!(f, "(de)"),
                cpu::Register::HL => write!(f, "(hl)"),
                cpu::Register::I => write!(f, "(i)"),
                cpu::Register::R => write!(f, "(r)"),
                cpu::Register::IX => write!(f, "(ix)"),
                cpu::Register::IXH => write!(f, "(ixh)"),
                cpu::Register::IXL => write!(f, "(ixl)"),
                cpu::Register::IY => write!(f, "(iy)"),
                cpu::Register::IYH => write!(f, "(iyh)"),
                cpu::Register::IYL => write!(f, "(iyl)"),
                cpu::Register::SP => write!(f, "(sp)"),
                cpu::Register::PC => write!(f, "(pc)"),
            }
            Operand::Direct8(address) => write!(f, "({:#04x})", address),
            Operand::Direct16(address) => write!(f, "({:#06x})", address),
            Operand::Indexed(register, displacement) => match register {
                cpu::Register::IX => write!(f, "(ix{:+})", displacement),
                cpu::Register::IY => write!(f, "(iy{:+})", displacement),
                _ => unreachable!(),
            }
            Operand::Bit(bit) => write!(f, "{}", bit),
        }
    }
}

enum InterruptMode { // TODO: move to CPU?
    Mode0,
    Mode1,
    Mode2,
}

enum JumpTest {
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

enum Instruction {
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
    fn decode(memory: &Memory, address: usize) -> (Instruction, usize) {
        // based on http://z80.info/decoding.htm

        let mut opcode = memory.read_byte(address);
        let mut next_address = address + 1;

        let prefix = match opcode {
            0xcb => {
                opcode = memory.read_byte(next_address);
                next_address += 1;
                Prefix::CB
            }
            0xed => {
                opcode = memory.read_byte(next_address);
                next_address += 1;
                Prefix::ED
            }
            0xdd => {
                opcode = memory.read_byte(next_address);
                next_address += 1;
                if opcode == 0xcb {
                    opcode = memory.read_byte(next_address);
                    let displacement = memory.read_byte(next_address + 1); // TODO: as i8?
                    next_address += 2;
                    Prefix::DDCB(displacement)
                } else {
                    Prefix::DD
                }
            }
            0xfd => {
                opcode = memory.read_byte(next_address);
                next_address += 1;
                if opcode == 0xcb {
                    opcode = memory.read_byte(next_address);
                    let displacement = memory.read_byte(next_address + 1); // TODO: as i8?
                    next_address += 2;
                    Prefix::FDCB(displacement)
                } else {
                    Prefix::FD
                }
            }
            _ => Prefix::None
        };  

        let x = opcode >> 6;
        let y = (opcode >> 3) & 7;
        let z = opcode & 7;

        let instruction = match (prefix, x, y, z) {
            (Prefix::None, 0, 0, 0) =>
                Instruction::Nop,
            (Prefix::None, 0, 1, 0) =>
                Instruction::Ex(
                    Operand::Register(cpu::Register::AF),
                    // this instruction swaps AF and AF', but using AF below uniquely identifies the instruction
                    Operand::Register(cpu::Register::AF),
                ),
            (Prefix::None, 0, 2, 0) => {
                let displacement = memory.read_byte(next_address) as i8;
                next_address += 1;
                Instruction::Djnz(
                    Operand::Immediate16((next_address as u16).wrapping_add(displacement as u16))
                )
            }
            (Prefix::None, 0, _, 0) => {
                let jump_test = if y == 3 {
                    JumpTest::Unconditional
                } else {
                    JumpTest::decode(y - 4)
                };
                let displacement = memory.read_byte(next_address) as i8;
                next_address += 1;
                Instruction::Jr(
                    jump_test,
                    Operand::Immediate16((next_address as u16).wrapping_add(displacement as u16))
                )
            }
            (Prefix::None, 0, _, 1) => {
                let p = y >> 1;
                let q = y & 1;

                let register_pair = Operand::decode_register_pair(p, false);

                match q {
                    0 => {
                        let value = Operand::Immediate16(memory.read_word(next_address));
                        next_address += 2;
                        Instruction::Ld(register_pair, value)
                    }
                    1 => {
                        Instruction::Add(Operand::Register(cpu::Register::HL), register_pair)
                    }
                    _ => unreachable!(),
                }
            }
            (Prefix::None, 0, _, 2) => {
                let p = y >> 1;
                let q = y & 1;

                let address = match p {
                    0 => Operand::RegisterIndirect(cpu::Register::BC),
                    1 => Operand::RegisterIndirect(cpu::Register::DE),
                    _ => {
                        let address = memory.read_word(next_address);
                        next_address += 2;
                        Operand::Direct16(address)
                    }
                };

                let register = match p {
                    2 => Operand::Register(cpu::Register::HL),
                    _ => Operand::Register(cpu::Register::A),
                };

                match q {
                    0 => Instruction::Ld(address, register),
                    1 => Instruction::Ld(register, address),
                    _ => unreachable!(),
                }
            }
            (Prefix::None, 0, _, 3) => {
                let p = y >> 1;
                let q = y & 1;

                let register_pair = Operand::decode_register_pair(p, false);

                match q {
                    0 => {
                        Instruction::Inc(register_pair)
                    }
                    1 => {
                        Instruction::Dec(register_pair)
                    }
                    _ => unreachable!(),
                }
            }
            (Prefix::None, 0, _, 4) => {
                let register = Operand::decode_register(y);
                Instruction::Inc(register)
            }
            (Prefix::None, 0, _, 5) => {
                let register = Operand::decode_register(y);
                Instruction::Dec(register)
            }
            (Prefix::None, 0, _, 6) => {
                let register = Operand::decode_register(y);

                let value = memory.read_byte(next_address);
                next_address += 1;

                Instruction::Ld(register, Operand::Immediate8(value))
            }
            (Prefix::None, 0, _, 7) => {
                match y {
                    0 => Instruction::Rlca,
                    1 => Instruction::Rrca,
                    2 => Instruction::Rla,
                    3 => Instruction::Rra,
                    4 => Instruction::Daa,
                    5 => Instruction::Cpl,
                    6 => Instruction::Scf,
                    7 => Instruction::Ccf,
                    _ => unreachable!(),
                }
            }
            (Prefix::None, 1, _, _) => {
                if y == 6 && z == 6 {
                    Instruction::Halt
                } else {
                    let destination = Operand::decode_register(y);
                    let source = Operand::decode_register(z);
                    Instruction::Ld(destination, source)
                }
            }
            (Prefix::None, 2, _, _) => {
                let register = Operand::decode_register(z);
                Instruction::decode_alu(y, register)
            }
            (Prefix::None, 3, _, 0) => {
                let jump_test = JumpTest::decode(y);
                Instruction::Ret(jump_test)
            }
            (Prefix::None, 3, _, 1) => {
                let p = y >> 1;
                let q = y & 1;

                match q {
                    0 => {
                        let register_pair = Operand::decode_register_pair(p, true);
                        Instruction::Pop(register_pair)
                    }
                    1 => match p {
                        0 => Instruction::Ret(JumpTest::Unconditional),
                        1 => Instruction::Exx,
                        2 => Instruction::Jp(JumpTest::Unconditional, Operand::Register(cpu::Register::HL)),
                        3 => Instruction::Ld(Operand::Register(cpu::Register::SP), Operand::Register(cpu::Register::HL)),
                        _ => unreachable!(),
                    }
                    _ => unreachable!(),
                }
            }
            (Prefix::None, 3, _, 2) => {
                let jump_test = JumpTest::decode(y);

                let target = Operand::Immediate16(memory.read_word(next_address));
                next_address += 2;

                Instruction::Jp(jump_test, target)
            }
            (Prefix::None, 3, _, 3) => {
                match y {
                    0 => {
                        let target = Operand::Immediate16(memory.read_word(next_address));
                        next_address += 2;

                        Instruction::Jp(JumpTest::Unconditional, target)
                    }
                    1 => unreachable!(),
                    2 => {
                        let port = Operand::Direct8(memory.read_byte(next_address));
                        next_address += 1;

                        Instruction::Out(port, Operand::Register(cpu::Register::A))
                    }
                    3 => {
                        let port = Operand::Direct8(memory.read_byte(next_address));
                        next_address += 1;

                        Instruction::In(Operand::Register(cpu::Register::A), port)
                    }
                    4 => Instruction::Ex(Operand::RegisterIndirect(cpu::Register::SP), Operand::Register(cpu::Register::HL)),
                    5 => Instruction::Ex(Operand::Register(cpu::Register::DE), Operand::Register(cpu::Register::HL)),
                    6 => Instruction::Di,
                    7 => Instruction::Ei,
                    _ => unreachable!(),
                }
            }
            (Prefix::None, 3, _, 4) => {
                let jump_test = JumpTest::decode(y);

                let target = Operand::Immediate16(memory.read_word(next_address));
                next_address += 2;

                Instruction::Call(jump_test, target)
            }
            (Prefix::None, 3, _, 5) => {
                let p = y >> 1;
                let q = y & 1;

                match q {
                    0 => {
                        let source = Operand::decode_register_pair(p, true);
                        Instruction::Push(source)
                    }
                    1 => if p == 0 {
                        let target = Operand::Immediate16(memory.read_word(next_address));
                        next_address += 2;

                        Instruction::Call(JumpTest::Unconditional, target)
                    } else {
                        unreachable!()
                    }
                    _ => unreachable!(),
                }
            }
            (Prefix::None, 3, _, 6) => {
                let value = Operand::Immediate8(memory.read_byte(next_address));
                next_address += 1;

                Instruction::decode_alu(y, value)
            }
            (Prefix::None, 3, _, 7) => {
                Instruction::Rst(Operand::Immediate8(y * 8))
            }
            (Prefix::CB, 0, _, _) => {
                let destination = Operand::decode_register(z);
                let operand = Operand::decode_register(z);

                Instruction::decode_bitshift(y, destination, operand)
            }
            (Prefix::CB, 1, _, _) => {
                let bit = Operand::Bit(y);
                let operand = Operand::decode_register(z);

                Instruction::Bit(bit, operand)
            }
            (Prefix::CB, 2, _, _) => {
                let bit = Operand::Bit(y);
                let destination = Operand::decode_register(z);
                let operand = Operand::decode_register(z);

                Instruction::Res(destination, bit, operand)
            }
            (Prefix::CB, 3, _, _) => {
                let bit = Operand::Bit(y);
                let destination = Operand::decode_register(z);
                let operand = Operand::decode_register(z);

                Instruction::Set(destination, bit, operand)
            }
            (Prefix::ED, 1, _, 0) => {
                let destination = Operand::decode_register(y);
                let port = Operand::RegisterIndirect(cpu::Register::C);

                Instruction::In(destination, port)
            }
            (Prefix::ED, 1, _, 1) => {
                let port = Operand::RegisterIndirect(cpu::Register::C);
                let source = Operand::decode_register(y);

                Instruction::Out(port, source)
            }
            (Prefix::ED, 1, _, 2) => {
                let p = y >> 1;
                let q = y & 1;

                let destination = Operand::Register(cpu::Register::HL);
                let source = Operand::decode_register_pair(p, false);

                match q {
                    0 => Instruction::Sbc(destination, source),
                    1 => Instruction::Adc(destination, source),
                    _ => unreachable!(),
                }
            }
            (Prefix::ED, 1, _, 3) => {
                let p = y >> 1;
                let q = y & 1;

                let address = Operand::Direct16(memory.read_word(next_address));
                next_address += 2;

                let register = Operand::decode_register_pair(p, false);

                match q {
                    0 => Instruction::Ld(address, register),
                    1 => Instruction::Ld(register, address),
                    _ => unreachable!(),
                }
            }
            (Prefix::ED, 1, _, 4) => {
                Instruction::Neg
            }
            (Prefix::ED, 1, _, 5) => {
                match y {
                    1 => Instruction::Reti,
                    _ => Instruction::Retn,
                }
            }
            (Prefix::ED, 1, _, 6) => {
                let mode = match y {
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
            (Prefix::ED, 1, _, 7) => {
                match y {
                    0 => Instruction::Ld(Operand::Register(cpu::Register::I), Operand::Register(cpu::Register::A)),
                    1 => Instruction::Ld(Operand::Register(cpu::Register::R), Operand::Register(cpu::Register::A)),
                    2 => Instruction::Ld(Operand::Register(cpu::Register::A), Operand::Register(cpu::Register::I)),
                    3 => Instruction::Ld(Operand::Register(cpu::Register::A), Operand::Register(cpu::Register::R)),
                    4 => Instruction::Rrd,
                    5 => Instruction::Rld,
                    6 => Instruction::Nop,
                    7 => Instruction::Nop,
                    _ => unreachable!(),
                }
            }
            (Prefix::ED, 2, _, _) => {
                match z {
                    0 => match y {
                        4 => Instruction::Ldi,
                        5 => Instruction::Ldd,
                        6 => Instruction::Ldir,
                        7 => Instruction::Lddr,
                        _ => Instruction::Defw(Operand::Immediate16(u16::from_le_bytes([0xed, opcode])))
                    }
                    1 => match y {
                        4 => Instruction::Cpi,
                        5 => Instruction::Cpd,
                        6 => Instruction::Cpir,
                        7 => Instruction::Cpdr,
                        _ => Instruction::Defw(Operand::Immediate16(u16::from_le_bytes([0xed, opcode])))
                    }
                    2 => match y {
                        4 => Instruction::Ini,
                        5 => Instruction::Ind,
                        6 => Instruction::Inir,
                        7 => Instruction::Indr,
                        _ => Instruction::Defw(Operand::Immediate16(u16::from_le_bytes([0xed, opcode])))
                    }
                    3 => match y {
                        4 => Instruction::Outi,
                        5 => Instruction::Outd,
                        6 => Instruction::Otir,
                        7 => Instruction::Otdr,
                        _ => Instruction::Defw(Operand::Immediate16(u16::from_le_bytes([0xed, opcode])))
                    }
                    _ => Instruction::Defw(Operand::Immediate16(u16::from_le_bytes([0xed, opcode])))
                }
            }
            (Prefix::ED, _, _, _) => {
                Instruction::Defw(Operand::Immediate16(u16::from_le_bytes([0xed, opcode])))
            }
            (Prefix::DD, 0, _, 1) => {
                let p = y >> 1;
                let q = y & 1;

                match q {
                    0 => {
                        let destination = Operand::decode_register_pair(p, false);
                        match destination {
                            Operand::Register(cpu::Register::HL) => {
                                let value = memory.read_word(next_address);
                                next_address += 2;

                                Instruction::Ld(Operand::Register(cpu::Register::IX), Operand::Immediate16(value))
                            }
                            _ => {
                                next_address -= 1;
                                Instruction::Defb(Operand::Immediate8(0xdd))
                            }
                        }
                    }
                    1 => {
                        let source = Operand::decode_register_pair(p, false);
                        match source {
                            Operand::Register(cpu::Register::HL) => {
                                Instruction::Add(Operand::Register(cpu::Register::IX), Operand::Register(cpu::Register::IX))
                            }
                            register => {
                                Instruction::Add(Operand::Register(cpu::Register::IX), register)
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }
            (Prefix::DD, 0, _, 2) => {
                let p = y >> 1;
                let q = y & 1;

                match p {
                    2 => {
                        let address = memory.read_word(next_address);
                        next_address += 2;

                        match q {
                            0 => Instruction::Ld(Operand::Direct16(address), Operand::Register(cpu::Register::IX)),
                            1 => Instruction::Ld(Operand::Register(cpu::Register::IX), Operand::Direct16(address)),
                            _ => unreachable!(),
                        }
                    }
                    _ => {
                            next_address -= 1;
                            Instruction::Defb(Operand::Immediate8(0xdd))
                    }
                }
            }
            (Prefix::DD, 0, _, 3) => {
                let p = y >> 1;
                let q = y & 1;

                match p {
                    2 => { // TODO: use decoded HL register?
                        match q {
                            0 => Instruction::Inc(Operand::Register(cpu::Register::IX)),
                            1 => Instruction::Dec(Operand::Register(cpu::Register::IX)),
                            _ => unreachable!(),
                        }
                    }
                    _ => {
                            next_address -= 1;
                            Instruction::Defb(Operand::Immediate8(0xdd))
                    }
                }
            }
            (Prefix::DD, 0, _, 4) => {
                let register = Operand::decode_register(y);

                match register {
                    Operand::Register(cpu::Register::H) => {
                        Instruction::Inc(Operand::Register(cpu::Register::IXH))
                    }
                    Operand::Register(cpu::Register::L) => {
                        Instruction::Inc(Operand::Register(cpu::Register::IXL))
                    }
                    Operand::RegisterIndirect(cpu::Register::HL) => {
                        let displacement = memory.read_byte(next_address) as i8;
                        next_address += 1;

                        Instruction::Inc(Operand::Indexed(cpu::Register::IX, displacement))
                    }
                    _ => {
                            next_address -= 1;
                            Instruction::Defb(Operand::Immediate8(0xdd))
                    }
                }
            }
            (Prefix::DD, 0, _, 5) => {
                let register = Operand::decode_register(y);

                match register {
                    Operand::Register(cpu::Register::H) => {
                        Instruction::Dec(Operand::Register(cpu::Register::IXH))
                    }
                    Operand::Register(cpu::Register::L) => {
                        Instruction::Dec(Operand::Register(cpu::Register::IXL))
                    }
                    Operand::RegisterIndirect(cpu::Register::HL) => {
                        let displacement = memory.read_byte(next_address) as i8;
                        next_address += 1;

                        Instruction::Dec(Operand::Indexed(cpu::Register::IX, displacement))
                    }
                    _ => {
                            next_address -= 1;
                            Instruction::Defb(Operand::Immediate8(0xdd))
                    }
                }
            }
            (Prefix::DD, 0, _, 6) => {
                let register = Operand::decode_register(y);

                match register {
                    Operand::Register(cpu::Register::H) => {
                        let value = memory.read_byte(next_address);
                        next_address += 1;

                        Instruction::Ld(Operand::Register(cpu::Register::IXH), Operand::Immediate8(value))
                    }
                    Operand::Register(cpu::Register::L) => {
                        let value = memory.read_byte(next_address);
                        next_address += 1;

                        Instruction::Ld(Operand::Register(cpu::Register::IXL), Operand::Immediate8(value))
                    }
                    Operand::RegisterIndirect(cpu::Register::HL) => {
                        let displacement = memory.read_byte(next_address) as i8;
                        next_address += 1;

                        let value = memory.read_byte(next_address);
                        next_address += 1;

                        Instruction::Ld(Operand::Indexed(cpu::Register::IX, displacement), Operand::Immediate8(value))
                    }
                    _ => {
                            next_address -= 1;
                            Instruction::Defb(Operand::Immediate8(0xdd))
                    }
                }
            }
            (Prefix::DD, 1, _, _) => {
                let destination = Operand::decode_register(y);
                let source = Operand::decode_register(y);

                match (destination, source) {
                    (Operand::RegisterIndirect(cpu::Register::HL), Operand::RegisterIndirect(cpu::Register::HL)) => {
                        unimplemented!() // defb
                    }
                    (Operand::RegisterIndirect(cpu::Register::HL), _) => {

                    }
                    (_, Operand::RegisterIndirect(cpu::Register::HL)) => {

                    }
                }
            }
            (Prefix::DD, _, _, _) => {
                next_address -= 1;
                Instruction::Defb(Operand::Immediate8(0xdd))
            }
            _ => panic!("Illegal instruction: {:x}", opcode), // TODO: id instruction more accurately
        };

        (instruction, next_address)
    }

    fn decode_alu(encoded: u8, operand: Operand) -> Instruction {
        match encoded {
            0 => Instruction::Add(Operand::Register(cpu::Register::A), operand),
            1 => Instruction::Adc(Operand::Register(cpu::Register::A), operand),
            2 => Instruction::Sub(operand),
            3 => Instruction::Sbc(Operand::Register(cpu::Register::A), operand),
            4 => Instruction::And(operand),
            5 => Instruction::Xor(operand),
            6 => Instruction::Or(operand),
            7 => Instruction::Cp(operand),
            _ => unreachable!(),
        }
    }

    fn decode_bitshift(encoded: u8, destination: Operand, operand: Operand) -> Instruction {
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

    fn timing(&self) -> usize {
        match self {
            Instruction::Adc(_, _) => unimplemented!(),
            Instruction::Add(_, _) => unimplemented!(),
            Instruction::And(_) => unimplemented!(),
            Instruction::Bit(_, _) => unimplemented!(),
            Instruction::Call(_, _) => unimplemented!(),
            Instruction::Ccf => unimplemented!(),
            Instruction::Cp(_) => unimplemented!(),
            Instruction::Cpd => unimplemented!(),
            Instruction::Cpdr => unimplemented!(),
            Instruction::Cpi => unimplemented!(),
            Instruction::Cpir => unimplemented!(),
            Instruction::Cpl => unimplemented!(),
            Instruction::Daa => unimplemented!(),
            Instruction::Dec(_) => unimplemented!(),
            Instruction::Defb(_) => unimplemented!(),
            Instruction::Defw(_) => unimplemented!(),
            Instruction::Di => unimplemented!(),
            Instruction::Djnz(_) => unimplemented!(),
            Instruction::Ei => unimplemented!(),
            Instruction::Ex(_, _) => unimplemented!(),
            Instruction::Exx => unimplemented!(),
            Instruction::Halt => unimplemented!(),
            Instruction::Im(_) => unimplemented!(),
            Instruction::In(_, _) => unimplemented!(),
            Instruction::Inc(_) => unimplemented!(),
            Instruction::Ind => unimplemented!(),
            Instruction::Indr => unimplemented!(),
            Instruction::Ini => unimplemented!(),
            Instruction::Inir => unimplemented!(),
            Instruction::Jp(_, _) => unimplemented!(),
            Instruction::Jr(_, _) => unimplemented!(),
            Instruction::Ld(_, _) => unimplemented!(),
            Instruction::Ldd => unimplemented!(),
            Instruction::Lddr => unimplemented!(),
            Instruction::Ldi => unimplemented!(),
            Instruction::Ldir => unimplemented!(),
            Instruction::Neg => unimplemented!(),
            Instruction::Nop => 4,
            Instruction::Or(_) => unimplemented!(),
            Instruction::Out(_, _) => unimplemented!(),
            Instruction::Otdr => unimplemented!(),
            Instruction::Otir => unimplemented!(),
            Instruction::Outd => unimplemented!(),
            Instruction::Outi => unimplemented!(),
            Instruction::Pop(_) => unimplemented!(),
            Instruction::Push(_) => unimplemented!(),
            Instruction::Res(_, _, _) => unimplemented!(),
            Instruction::Ret(_) => unimplemented!(),
            Instruction::Reti => unimplemented!(),
            Instruction::Retn => unimplemented!(),
            Instruction::Rl(_, _) => unimplemented!(),
            Instruction::Rla => unimplemented!(),
            Instruction::Rlc(_, _) => unimplemented!(),
            Instruction::Rlca => unimplemented!(),
            Instruction::Rld => unimplemented!(),
            Instruction::Rr(_, _) => unimplemented!(),
            Instruction::Rrc(_, _) => unimplemented!(),
            Instruction::Rra => unimplemented!(),
            Instruction::Rrca => unimplemented!(),
            Instruction::Rrd => unimplemented!(),
            Instruction::Rst(_) => unimplemented!(),
            Instruction::Sbc(_, _) => unimplemented!(),
            Instruction::Scf => unimplemented!(),
            Instruction::Set(_, _, _) => unimplemented!(),
            Instruction::Sla(_, _) => unimplemented!(),
            Instruction::Sll(_, _) => unimplemented!(),
            Instruction::Sra(_, _) => unimplemented!(),
            Instruction::Srl(_, _) => unimplemented!(),
            Instruction::Sub(_) => unimplemented!(),
            Instruction::Xor(_) => unimplemented!(),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Adc(destination, value) => write!(f, "adc {},{}", destination, value),
            Instruction::Add(destination, value) => write!(f, "add {},{}", destination, value),
            Instruction::And(value) => write!(f, "and {}", value),
            Instruction::Bit(_, _) => unimplemented!(),
            Instruction::Call(jump_test, target) => match jump_test {
                JumpTest::Unconditional => write!(f, "call {}", target),
                _ => write!(f, "call {},{}", jump_test, target),
            }
            Instruction::Ccf => write!(f, "ccf"),
            Instruction::Cp(value) => write!(f, "cp {}", value),
            Instruction::Cpd => unimplemented!(),
            Instruction::Cpdr => unimplemented!(),
            Instruction::Cpi => unimplemented!(),
            Instruction::Cpir => unimplemented!(),
            Instruction::Cpl => write!(f, "cpl"),
            Instruction::Daa => write!(f, "daa"),
            Instruction::Dec(destination) => write!(f, "dec {}", destination),
            Instruction::Defb(value) => write!(f, "defb {}", value),
            Instruction::Defw(value) => write!(f, "defw {}", value),
            Instruction::Di => write!(f, "di"),
            Instruction::Djnz(target) => write!(f, "djnz {}", target),
            Instruction::Ei => write!(f, "ei"),
            Instruction::Ex(Operand::Register(cpu::Register::AF), _) => write!(f, "ex af,af'"),
            Instruction::Ex(left, right) => write!(f, "ex {},{}", left, right),
            Instruction::Exx => write!(f, "exx"),
            Instruction::Halt => write!(f, "halt"),
            Instruction::Im(mode) => match mode {
                InterruptMode::Mode0 => write!(f, "im 0"),
                InterruptMode::Mode1 => write!(f, "im 1"),
                InterruptMode::Mode2 => write!(f, "im 2"),
            }
            Instruction::In(destination, port) => write!(f, "in {},{}", destination, port),
            Instruction::Inc(destination) => write!(f, "inc {}", destination),
            Instruction::Ind => unimplemented!(),
            Instruction::Indr => unimplemented!(),
            Instruction::Ini => unimplemented!(),
            Instruction::Inir => unimplemented!(),
            Instruction::Jp(jump_test, target) => match jump_test {
                JumpTest::Unconditional => write!(f, "jp {}", target),
                _ => write!(f, "jp {},{}", jump_test, target),
            }
            Instruction::Jr(jump_test, target) => match jump_test {
                JumpTest::Unconditional => write!(f, "jr {}", target),
                _ => write!(f, "jr {},{}", jump_test, target),
            }
            Instruction::Ld(destination, source) => write!(f, "ld {},{}", destination, source),
            Instruction::Ldd => unimplemented!(),
            Instruction::Lddr => unimplemented!(),
            Instruction::Ldi => unimplemented!(),
            Instruction::Ldir => unimplemented!(),
            Instruction::Neg => write!(f, "neg"),
            Instruction::Nop => write!(f, "nop"),
            Instruction::Or(value) => write!(f, "or {}", value),
            Instruction::Out(port, source) => write!(f, "out {},{}", port, source),
            Instruction::Otdr => unimplemented!(),
            Instruction::Otir => unimplemented!(),
            Instruction::Outd => unimplemented!(),
            Instruction::Outi => unimplemented!(),
            Instruction::Pop(destination) => write!(f, "pop {}", destination),
            Instruction::Push(source) => write!(f, "push {}", source),
            Instruction::Res(destination, bit, operand) => match operand {
                Operand::Indexed(_, _) => match destination {
                    Operand::RegisterIndirect(cpu::Register::HL) => write!(f, "res {},{}", bit, operand),
                    _ => write!(f, "res {},{}->{}", bit, operand, destination),
                }
                _ => write!(f, "res {},{}", bit, operand),
            }
            Instruction::Ret(jump_test) => match jump_test {
                JumpTest::Unconditional => write!(f, "ret"),
                _ => write!(f, "ret {}", jump_test),
            }
            Instruction::Reti => write!(f, "reti"),
            Instruction::Retn => write!(f, "retn"),
            Instruction::Rl(destination, operand) => match operand {
                Operand::Indexed(_, _) => match destination {
                    Operand::RegisterIndirect(cpu::Register::HL) => write!(f, "rl {}", operand),
                    _ => write!(f, "rl {}->{}", operand, destination),
                }
                _ => write!(f, "rl {}", operand),
            }
            Instruction::Rla => write!(f, "rla"),
            Instruction::Rlc(destination, operand) => match operand { // TODO: extract this into reusable method. how to handle helpers in traits?
                Operand::Indexed(_, _) => match destination {
                    Operand::RegisterIndirect(cpu::Register::HL) => write!(f, "rlc {}", operand),
                    _ => write!(f, "rlc {}->{}", operand, destination),
                }
                _ => write!(f, "rlc {}", operand),
            }
            Instruction::Rlca => write!(f, "rlca"),
            Instruction::Rld => write!(f, "rld"),
            Instruction::Rr(destination, operand) => match operand {
                Operand::Indexed(_, _) => match destination {
                    Operand::RegisterIndirect(cpu::Register::HL) => write!(f, "rr {}", operand),
                    _ => write!(f, "rr {}->{}", operand, destination),
                }
                _ => write!(f, "rr {}", operand),
            }
            Instruction::Rrc(destination, operand) => match operand {
                Operand::Indexed(_, _) => match destination {
                    Operand::RegisterIndirect(cpu::Register::HL) => write!(f, "rrc {}", operand),
                    _ => write!(f, "rrc {}->{}", operand, destination),
                }
                _ => write!(f, "rrc {}", operand),
            }
            Instruction::Rra => write!(f, "rra"),
            Instruction::Rrca => write!(f, "rrca"),
            Instruction::Rrd => write!(f, "rrd"),
            Instruction::Rst(target) => write!(f, "rst {}", target),
            Instruction::Sbc(destination, value) => write!(f, "sbc {},{}", destination, value),
            Instruction::Scf => write!(f, "scf"),
            Instruction::Set(destination, bit, operand) => match operand {
                Operand::Indexed(_, _) => match destination {
                    Operand::RegisterIndirect(cpu::Register::HL) => write!(f, "set {},{}", bit, operand),
                    _ => write!(f, "set {},{}->{}", bit, operand, destination),
                }
                _ => write!(f, "set {},{}", bit, operand),
            }
            Instruction::Sla(destination, operand) => match operand {
                Operand::Indexed(_, _) => match destination {
                    Operand::RegisterIndirect(cpu::Register::HL) => write!(f, "sla {}", operand),
                    _ => write!(f, "sla {}->{}", operand, destination),
                }
                _ => write!(f, "sla {}", operand),
            }
            Instruction::Sll(destination, operand) => match operand {
                Operand::Indexed(_, _) => match destination {
                    Operand::RegisterIndirect(cpu::Register::HL) => write!(f, "sll {}", operand),
                    _ => write!(f, "sll {}->{}", operand, destination),
                }
                _ => write!(f, "sll {}", operand),
            }
            Instruction::Sra(destination, operand) => match operand {
                Operand::Indexed(_, _) => match destination {
                    Operand::RegisterIndirect(cpu::Register::HL) => write!(f, "sra {}", operand),
                    _ => write!(f, "sra {}->{}", operand, destination),
                }
                _ => write!(f, "sra {}", operand),
            }
            Instruction::Srl(destination, operand) => match operand {
                Operand::Indexed(_, _) => match destination {
                    Operand::RegisterIndirect(cpu::Register::HL) => write!(f, "srl {}", operand),
                    _ => write!(f, "srl {}->{}", operand, destination),
                }
                _ => write!(f, "srl {}", operand),
            }
            Instruction::Sub(value) => write!(f, "sub {}", value),
            Instruction::Xor(value) => write!(f, "xor {}", value),
        }
    }
}

enum DecoderMode {
    Default,
    PatchIX,
    PatchIY,
}

impl DecoderMode {
    fn into_instruction(&self) -> Instruction {
        match self {
            DecoderMode::PatchIX => Instruction::Defb(Operand::Immediate8(0xdd)),
            DecoderMode::PatchIY => Instruction::Defb(Operand::Immediate8(0xfd)),
            _ => unreachable!(),
        }
    }
}

struct Decoder<'m, M> {
    memory: &'m M,
    next_address: usize,
    mode: DecoderMode,
    patched: bool,
}

impl<'m, M> Decoder<'m, M>
where M: Read
{ // based on http://z80.info/decoding.htm
    fn new(memory: &M) -> Decoder<M> {
        Decoder {
            memory,
            next_address: 0,
            mode: DecoderMode::Default,
            patched: false
        }
    }

    fn decode_at(&self, address: usize) -> (Instruction, usize) {
        self.next_address = address;

        let opcode = self.memory.read_byte(self.next_address);
        self.next_address += 1;

        match opcode {
            0xcb => (self.decode_cb_instruction(), self.next_address),
            0xed => (self.decode_ed_instruction(), self.next_address),
            0xdd => (self.decode_prefixed(DecoderMode::PatchIX), self.next_address),
            0xfd => (self.decode_prefixed(DecoderMode::PatchIY), self.next_address),
            _ => (self.decode_instruction(opcode), self.next_address),
        }
    }

    fn decode_next(&self) -> (Instruction, usize) {
        self.decode_at(self.next_address)
    }

    fn decode_cb_instruction(&self) -> Instruction {
        Instruction::Nop
    }

    fn decode_ed_instruction(&self) -> Instruction {
        Instruction::Nop
    }

    fn decode_prefixed(&self, mode: DecoderMode) -> Instruction {
        self.mode = mode;

        let opcode = self.memory.read_byte(self.next_address);

        match opcode {
            0xcb => {
                self.next_address += 1;
                self.decode_cb_instruction()
            }
            0xed => self.mode.into_instruction(),
            0xdd => self.mode.into_instruction(),
            0xfd => self.mode.into_instruction(),
            _ => {
                self.next_address += 1;
                self.patched = false;

                let start = self.next_address;
                let instruction = self.decode_instruction(opcode);

                if self.patched {
                    instruction
                } else {
                    self.next_address = start;
                    self.mode.into_instruction()
                }
            }
        }
    }

    fn decode_instruction(&self, opcode: u8) -> Instruction {
        let x = opcode >> 6;
        let y = (opcode >> 3) & 7;
        let z = opcode & 7;

        match (x, y, z) {
            (0, 0, 0) =>
                Instruction::Nop,
            (0, 1, 0) =>
                Instruction::Ex(
                    Operand::Register(cpu::Register::AF),
                    // this instruction swaps AF and AF', but using AF below uniquely identifies the instruction
                    Operand::Register(cpu::Register::AF),
                ),
            (0, 2, 0) => {
                let displacement = self.memory.read_byte(self.next_address) as i8;
                self.next_address += 1;
                Instruction::Djnz(
                    Operand::Immediate16((self.next_address as u16).wrapping_add(displacement as u16))
                )
            }
            (0, _, 0) => {
                let jump_test = if y == 3 {
                    JumpTest::Unconditional
                } else {
                    JumpTest::decode(y - 4)
                };
                let displacement = self.memory.read_byte(self.next_address) as i8;
                self.next_address += 1;
                Instruction::Jr(
                    jump_test,
                    Operand::Immediate16((self.next_address as u16).wrapping_add(displacement as u16))
                )
            }
            (0, _, 1) => {
                let p = y >> 1;
                let q = y & 1;

                let register_pair = Operand::decode_register_pair(p, false);

                match q {
                    0 => {
                        let value = Operand::Immediate16(self.memory.read_word(self.next_address));
                        self.next_address += 2;
                        Instruction::Ld(register_pair, value)
                    }
                    1 => {
                        Instruction::Add(Operand::Register(cpu::Register::HL), register_pair)
                    }
                    _ => unreachable!(),
                }
            }
            (0, _, 2) => {
                let p = y >> 1;
                let q = y & 1;

                let address = match p {
                    0 => Operand::RegisterIndirect(cpu::Register::BC),
                    1 => Operand::RegisterIndirect(cpu::Register::DE),
                    _ => {
                        let address = self.memory.read_word(self.next_address);
                        self.next_address += 2;
                        Operand::Direct16(address)
                    }
                };

                let register = match p {
                    2 => Operand::Register(cpu::Register::HL),
                    _ => Operand::Register(cpu::Register::A),
                };

                match q {
                    0 => Instruction::Ld(address, register),
                    1 => Instruction::Ld(register, address),
                    _ => unreachable!(),
                }
            }
            (0, _, 3) => {
                let p = y >> 1;
                let q = y & 1;

                let register_pair = Operand::decode_register_pair(p, false);

                match q {
                    0 => {
                        Instruction::Inc(register_pair)
                    }
                    1 => {
                        Instruction::Dec(register_pair)
                    }
                    _ => unreachable!(),
                }
            }
            (0, _, 4) => {
                let register = Operand::decode_register(y);
                Instruction::Inc(register)
            }
            (0, _, 5) => {
                let register = Operand::decode_register(y);
                Instruction::Dec(register)
            }
            (0, _, 6) => {
                let register = Operand::decode_register(y);

                let value = self.memory.read_byte(self.next_address);
                self.next_address += 1;

                Instruction::Ld(register, Operand::Immediate8(value))
            }
            (0, _, 7) => {
                match y {
                    0 => Instruction::Rlca,
                    1 => Instruction::Rrca,
                    2 => Instruction::Rla,
                    3 => Instruction::Rra,
                    4 => Instruction::Daa,
                    5 => Instruction::Cpl,
                    6 => Instruction::Scf,
                    7 => Instruction::Ccf,
                    _ => unreachable!(),
                }
            }
            (1, _, _) => {
                if y == 6 && z == 6 {
                    Instruction::Halt
                } else {
                    let destination = Operand::decode_register(y);
                    let source = Operand::decode_register(z);
                    Instruction::Ld(destination, source)
                }
            }
            (2, _, _) => {
                let register = Operand::decode_register(z);
                Instruction::decode_alu(y, register)
            }
            (3, _, 0) => {
                let jump_test = JumpTest::decode(y);
                Instruction::Ret(jump_test)
            }
            (3, _, 1) => {
                let p = y >> 1;
                let q = y & 1;

                match q {
                    0 => {
                        let register_pair = Operand::decode_register_pair(p, true);
                        Instruction::Pop(register_pair)
                    }
                    1 => match p {
                        0 => Instruction::Ret(JumpTest::Unconditional),
                        1 => Instruction::Exx,
                        2 => Instruction::Jp(JumpTest::Unconditional, Operand::Register(cpu::Register::HL)),
                        3 => Instruction::Ld(Operand::Register(cpu::Register::SP), Operand::Register(cpu::Register::HL)),
                        _ => unreachable!(),
                    }
                    _ => unreachable!(),
                }
            }
            (3, _, 2) => {
                let jump_test = JumpTest::decode(y);

                let target = Operand::Immediate16(self.memory.read_word(self.next_address));
                self.next_address += 2;

                Instruction::Jp(jump_test, target)
            }
            (3, _, 3) => {
                match y {
                    0 => {
                        let target = Operand::Immediate16(self.memory.read_word(self.next_address));
                        self.next_address += 2;

                        Instruction::Jp(JumpTest::Unconditional, target)
                    }
                    1 => unreachable!(),
                    2 => {
                        let port = Operand::Direct8(self.memory.read_byte(self.next_address));
                        self.next_address += 1;

                        Instruction::Out(port, Operand::Register(cpu::Register::A))
                    }
                    3 => {
                        let port = Operand::Direct8(self.memory.read_byte(self.next_address));
                        self.next_address += 1;

                        Instruction::In(Operand::Register(cpu::Register::A), port)
                    }
                    4 => Instruction::Ex(Operand::RegisterIndirect(cpu::Register::SP), Operand::Register(cpu::Register::HL)),
                    5 => Instruction::Ex(Operand::Register(cpu::Register::DE), Operand::Register(cpu::Register::HL)),
                    6 => Instruction::Di,
                    7 => Instruction::Ei,
                    _ => unreachable!(),
                }
            }
            (3, _, 4) => {
                let jump_test = JumpTest::decode(y);

                let target = Operand::Immediate16(self.memory.read_word(self.next_address));
                self.next_address += 2;

                Instruction::Call(jump_test, target)
            }
            (3, _, 5) => {
                let p = y >> 1;
                let q = y & 1;

                match q {
                    0 => {
                        let source = Operand::decode_register_pair(p, true);
                        Instruction::Push(source)
                    }
                    1 => if p == 0 {
                        let target = Operand::Immediate16(self.memory.read_word(self.next_address));
                        self.next_address += 2;

                        Instruction::Call(JumpTest::Unconditional, target)
                    } else {
                        unreachable!()
                    }
                    _ => unreachable!(),
                }
            }
            (3, _, 6) => {
                let value = Operand::Immediate8(self.memory.read_byte(self.next_address));
                self.next_address += 1;

                Instruction::decode_alu(y, value)
            }
            (3, _, 7) => {
                Instruction::Rst(Operand::Immediate8(y * 8))
            }
            _ => panic!("Illegal instruction: {:x}", opcode), // TODO: id instruction more accurately
        }
    }
}
