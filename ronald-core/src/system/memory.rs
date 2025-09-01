use std::collections::HashMap;
use std::fs::*;
use std::io;

use serde::{Deserialize, Serialize};

pub trait MemRead {
    fn read_byte(&self, address: usize) -> u8;

    fn read_word(&self, address: usize) -> u16 {
        let low_byte = self.read_byte(address);
        let high_byte = self.read_byte(address + 1);
        u16::from_le_bytes([low_byte, high_byte])
    }
}

pub trait MemWrite {
    fn write_byte(&mut self, address: usize, value: u8);

    fn write_word(&mut self, address: usize, value: u16) {
        let bytes = value.to_le_bytes();
        self.write_byte(address, bytes[0]);
        self.write_byte(address + 1, bytes[1]);
    }
}

pub trait MemManage {
    fn enable_lower_rom(&mut self, enable: bool);

    fn enable_upper_rom(&mut self, enable: bool);

    fn select_upper_rom(&mut self, upper_rom_nr: u8);

    fn force_ram_read(&mut self, force: bool);
}

#[derive(Serialize, Deserialize)]
pub struct Rom {
    #[serde(rename = "rom")]
    data: Vec<u8>,
}

impl Rom {
    pub fn from_bytes(bytes: &[u8]) -> Rom {
        // TODO: better error handling
        // TODO: check ROM size (should be 16k)
        Rom {
            data: bytes.to_vec(),
        }
    }
}

impl MemRead for Rom {
    fn read_byte(&self, address: usize) -> u8 {
        self.data[address]
    }
}

#[derive(Serialize, Deserialize)]
pub struct Ram {
    #[serde(rename = "ram")]
    data: Vec<u8>,
}

impl Ram {
    pub fn new(size: usize) -> Ram {
        Ram {
            data: vec![0; size],
        }
    }

    pub fn from_file(size: usize, path: &str, offset: usize) -> Ram {
        // TODO: better error handling
        // TODO: check if ROM fits
        let mut ram = Ram::new(size);
        let rom = read(path).unwrap_or_else(|_| panic!("ROM file \"{path}\" could not be read."));

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

impl MemRead for Ram {
    fn read_byte(&self, address: usize) -> u8 {
        self.data[address]
    }
}

impl MemWrite for Ram {
    fn write_byte(&mut self, address: usize, value: u8) {
        self.data[address] = value;
    }
}

impl MemManage for Ram {
    fn enable_lower_rom(&mut self, enable: bool) {}

    fn enable_upper_rom(&mut self, enable: bool) {}

    fn select_upper_rom(&mut self, upper_rom_nr: u8) {}

    fn force_ram_read(&mut self, force: bool) {}
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryCpcX64 {
    #[serde(flatten)]
    ram: Ram,
    #[serde(flatten)]
    lower_rom: Rom,
    lower_rom_enabled: bool,
    upper_roms: HashMap<u8, Rom>,
    selected_upper_rom: u8,
    upper_rom_enabled: bool,
    ram_read_forced: bool,
}

impl MemoryCpcX64 {
    pub fn new() -> Self {
        // TODO: receive rom paths as parameters
        let mut upper_roms = HashMap::new();
        upper_roms.insert(
            0,
            Rom::from_bytes(include_bytes!("../../rom/basic_1.0.rom")),
        );
        upper_roms.insert(
            7,
            Rom::from_bytes(include_bytes!("../../rom/amsdos_0.5.rom")),
        );

        MemoryCpcX64 {
            ram: Ram::new(0x10000),
            lower_rom: Rom::from_bytes(include_bytes!("../../rom/os_464.rom")),
            lower_rom_enabled: true,
            upper_roms,
            selected_upper_rom: 0,
            upper_rom_enabled: true,
            ram_read_forced: false,
        }
    }
}

impl Default for MemoryCpcX64 {
    fn default() -> Self {
        Self::new()
    }
}

impl MemRead for MemoryCpcX64 {
    fn read_byte(&self, address: usize) -> u8 {
        if self.ram_read_forced {
            return self.ram.read_byte(address);
        }

        // TODO: define proper constants
        if self.lower_rom_enabled && address < 0x4000 {
            return self.lower_rom.read_byte(address);
        }

        if self.upper_rom_enabled && address >= 0xc000 {
            match self.upper_roms.get(&self.selected_upper_rom) {
                Some(upper_rom) => {
                    return upper_rom.read_byte(address - 0xc000);
                }
                None => {
                    return self.ram.read_byte(address);
                }
            }
        }

        self.ram.read_byte(address)
    }
}

impl MemWrite for MemoryCpcX64 {
    fn write_byte(&mut self, address: usize, value: u8) {
        self.ram.write_byte(address, value);
    }
}

impl MemManage for MemoryCpcX64 {
    fn enable_lower_rom(&mut self, enable: bool) {
        self.lower_rom_enabled = enable;
    }

    fn enable_upper_rom(&mut self, enable: bool) {
        self.upper_rom_enabled = enable;
    }

    fn select_upper_rom(&mut self, upper_rom_nr: u8) {
        self.selected_upper_rom = upper_rom_nr;
    }

    fn force_ram_read(&mut self, force: bool) {
        self.ram_read_forced = force;
    }
}

#[derive(Serialize, Deserialize)]
pub struct MemoryCpc6128 {
    memory: MemoryCpcX64,
}

impl MemoryCpc6128 {
    pub fn new() -> Self {
        MemoryCpc6128 {
            memory: MemoryCpcX64::new(),
        }
    }
}

impl Default for MemoryCpc6128 {
    fn default() -> Self {
        Self::new()
    }
}

impl MemRead for MemoryCpc6128 {
    fn read_byte(&self, address: usize) -> u8 {
        self.memory.read_byte(address)
    }

    fn read_word(&self, address: usize) -> u16 {
        self.memory.read_word(address)
    }
}

impl MemWrite for MemoryCpc6128 {
    fn write_byte(&mut self, address: usize, value: u8) {
        self.memory.write_byte(address, value);
    }

    fn write_word(&mut self, address: usize, value: u16) {
        self.memory.write_word(address, value);
    }
}

impl MemManage for MemoryCpc6128 {
    fn enable_lower_rom(&mut self, enable: bool) {
        self.memory.enable_lower_rom(enable);
    }

    fn enable_upper_rom(&mut self, enable: bool) {
        self.memory.enable_upper_rom(enable);
    }

    fn select_upper_rom(&mut self, upper_rom_nr: u8) {
        self.memory.select_upper_rom(upper_rom_nr);
    }

    fn force_ram_read(&mut self, force: bool) {
        self.memory.force_ram_read(force);
    }
}
