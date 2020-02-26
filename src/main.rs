mod memory;
mod instruction;
mod cpu;

fn main() {
    exercise("rom/zexall.com");
    decode("rom/amsdos_0.5.rom");
}

fn exercise(path: &str) {
    let memory = memory::RAM::from_file(0x10000, path, 0x100);
    let mut cpu = CPU::new(memory, 0x100);
    while cpu.fetch_and_execute() {}
}

fn decode(path: &str) {
    let rom = memory::ROM::from_file(path);
    let mut decoder = instruction::Decoder::new(&rom);
    let mut current_address = 0;
    while current_address < 0x4000 {
        let (instruction, next_address) = decoder.decode_next();
        println!("{}", instruction);
        current_address = next_address;
    }
}