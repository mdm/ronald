mod cpu;
mod instruction;
mod memory;
mod crtc;
mod system;

use system::System;

fn main() {
    let mut zex_harness = system::ZexHarness::new("rom/zexdoc.rom");
    zex_harness.emulate(None);
    // decode("rom/amsdos_0.5.rom");
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
