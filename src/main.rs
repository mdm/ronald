mod memory;
mod instruction;
mod cpu;

fn main() {
    let rom = memory::ROM::from_file("rom/os_464.rom");
    let mut decoder = instruction::Decoder::new(&rom);
    let mut current_address = 0;
    while current_address < 0x4000 {
        let (instruction, next_address) = decoder.decode_next();
        println!("{}", instruction);
        current_address = next_address;
    }
}
