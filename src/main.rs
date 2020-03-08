mod cpu;
mod instruction;
mod memory;

use memory::{ Read, Write };

fn main() {
    exercise("rom/zexall.rom");
    decode("rom/amsdos_0.5.rom");
}

fn exercise(path: &str) {
    let mut memory = memory::RAM::from_file(0x10000, path, 0x100);
    memory.write_byte(0x0005, 0xc9); // patch with RET instruction
    memory.write_word(0x0006, 0xe400); // patch with initial SP
    let mut cpu = cpu::CPU::new(0x100);

    loop {
        match cpu.registers.read_word(&cpu::Register::PC) {
            0x0000 => break,
            0x0005 => {
                match cpu.registers.read_byte(&cpu::Register::C) {
                    5 => print!("{}", cpu.registers.read_byte(&cpu::Register::E) as char),
                    9 => {
                        let mut address = cpu.registers.read_word(&cpu::Register::DE) as usize;
                        loop {
                            let character = memory.read_byte(address) as char;
                            if character == '$' {
                                break;
                            }
                            else {
                                print!("{}", character);
                            }
                            address += 1;
                        }
                    }
                    _ => unreachable!(),
                }
                cpu.fetch_and_execute(&mut memory);
            }
            _ => cpu.fetch_and_execute(&mut memory),
        }
    }
}

fn decode(path: &str) {
    let rom = memory::ROM::from_file(path);
    let mut decoder = instruction::Decoder::new();
    let mut current_address = 0;
    while current_address < 0x4000 {
        let (instruction, next_address) = decoder.decode_next(&rom);
        println!("{}", instruction);
        current_address = next_address;
    }
}
