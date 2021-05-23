use std::collections::HashMap;
use std::fs::*;
use std::io;

use std::cell::RefCell;
use std::rc::Rc;

pub type MemoryShared = Rc<RefCell<Memory>>;

pub trait Read {
    fn read_byte(&self, address: usize) -> u8;

    fn read_word(&self, address: usize) -> u16 {
        let low_byte = self.read_byte(address);
        let high_byte = self.read_byte(address + 1);
        u16::from_le_bytes([low_byte, high_byte])
    }
}

pub trait Write {
    fn write_byte(&mut self, address: usize, value: u8);

    fn write_word(&mut self, address: usize, value: u16) {
        let bytes = value.to_le_bytes();
        self.write_byte(address, bytes[0]);
        self.write_byte(address + 1, bytes[1]);
    }
}

pub struct ROM {
    data: Vec<u8>,
}

impl ROM {
    pub fn from_file(path: &str) -> ROM {
        // TODO: better error handling
        // TODO: check ROM size (should be 16k)
        ROM { data: std::fs::read(path).expect(&format!("ROM file \"{}\" could not be read.", path)) }
    }
}

impl Read for ROM {
    fn read_byte(&self, address: usize) -> u8 {
        self.data[address]
    }
}

pub struct RAM {
    data: Vec<u8>,
}

impl RAM {
    pub fn new(size: usize) -> RAM {
        RAM { data: vec![0; size] }
    }

    pub fn from_file(size: usize, path: &str, offset: usize) -> RAM {
        // TODO: better error handling
        // TODO: check if ROM fits
        let mut ram = RAM::new(size);
        let rom = read(path).expect(&format!("ROM file \"{}\" could not be read.", path));

        for (i, byte) in rom.into_iter().enumerate() {
            ram.write_byte(offset + i, byte);
        }

        ram
    }

    pub fn write_to_file(&self, file: &mut File) -> io::Result<()> {
        // TODO: better error handling
        io::Write::write_all(file, &self.data)
    }
}

impl Read for RAM {
    fn read_byte(&self, address: usize) -> u8 {
        self.data[address]
    }
}

impl Write for RAM {
    fn write_byte(&mut self, address: usize, value: u8) {
        self.data[address] = value;
    }
}

pub struct Memory {
    ram: RAM,
    lower_rom: ROM,
    lower_rom_enabled: bool,
    upper_roms: HashMap<u8, ROM>,
    selected_upper_rom: u8,
    upper_rom_enabled: bool,
}

impl Memory {
    pub fn new_shared() -> MemoryShared {
        // TODO: receive rom paths as parameters
        let mut upper_roms = HashMap::new();
        upper_roms.insert(0, ROM::from_file("rom/basic_1.0.rom"));
        upper_roms.insert(7, ROM::from_file("rom/amsdos_0.5.rom"));

        let memory = Memory {
            ram: RAM::new(0x10000),
            lower_rom: ROM::from_file("rom/os_464.rom"),
            lower_rom_enabled: true,
            upper_roms,
            selected_upper_rom: 0,
            upper_rom_enabled: true,
        };

        Rc::new(RefCell::new(memory))
    }

    pub fn enable_lower_rom(&mut self, enable: bool) {
        self.lower_rom_enabled = enable;
    }

    pub fn enable_upper_rom(&mut self, enable: bool) {
        self.upper_rom_enabled = enable;
    }
}

impl Read for Memory {
    fn read_byte(&self, address: usize) -> u8 {
        // TODO: define proper constants
        if self.lower_rom_enabled && address < 0x4000 {
            return self.lower_rom.read_byte(address);
        }

        if self.upper_rom_enabled && address >= 0xc000 {
            match self.upper_roms.get(&self.selected_upper_rom) {
                Some(upper_rom) => {
                    return upper_rom.read_byte(address);
                }
                None => {
                    panic!("No upper ROM loaded in slot {}", self.selected_upper_rom);
                }
            }
        }

        self.ram.read_byte(address)
    }
}

impl Write for Memory {
    fn write_byte(&mut self, address: usize, value: u8) {
        self.ram.write_byte(address, value);
    }
}
