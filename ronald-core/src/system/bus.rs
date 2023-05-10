use crate::system::memory::{Memory, Mmu};
use crate::{AudioSink, VideoSink};

mod crtc;
mod fdc;
mod gate_array;
pub mod keyboard; // TODO: refactor to not use pub
mod ppi;
mod psg;
pub mod screen; // TODO: refactor to not use pub
mod tape;

use crtc::CRTController;
use fdc::FloppyDiskController;
use gate_array::GateArray;
use keyboard::Keyboard;
use ppi::PeripheralInterface;
use self::psg::SoundGenerator;
use screen::Screen;
use tape::TapeController;

pub trait Bus {
    fn read_byte(&mut self, port: u16) -> u8;
    fn write_byte(&mut self, memory: &mut impl Mmu, port: u16, value: u8);
}

pub struct DummyBus {}

impl DummyBus {
    pub fn new() -> DummyBus {
        DummyBus {}
    }
}

impl Bus for DummyBus {
    fn read_byte(&mut self, _port: u16) -> u8 {
        unimplemented!()
    }

    fn write_byte(&mut self, _memory: &mut impl Mmu, _port: u16, _value: u8) {
        unimplemented!();
    }
}

pub struct StandardBus {
    crtc: CRTController,
    fdc: FloppyDiskController,
    gate_array: GateArray,
    keyboard: Keyboard,
    ppi: PeripheralInterface,
    psg: SoundGenerator,
    screen: Screen,
    tape: TapeController,
}

impl StandardBus {
    pub fn new() -> Self {
        let crtc = CRTController::new();
        let fdc = FloppyDiskController::new();
        let gate_array = GateArray::new();
        let keyboard = Keyboard::new();
        let ppi = PeripheralInterface::new();
        let psg = SoundGenerator::new();
        let screen = Screen::new();
        let tape = TapeController::new();

        StandardBus {
            crtc,
            fdc,
            gate_array,
            keyboard,
            ppi,
            psg,
            screen,
            tape,
        }
    }

    pub fn step(&mut self, memory: &mut Memory, video: &mut impl VideoSink, audio: &mut impl AudioSink) -> bool {
        self.psg.step(audio);
        self.crtc.step();
        self.gate_array.step(&self.crtc, memory, &mut self.screen, video)
    }

    pub fn acknowledge_interrupt(&mut self) {
        self.gate_array.acknowledge_interrupt();
    }

    pub fn set_key(&mut self, line: usize, bit: u8) {
        self.keyboard.set_key(line, bit)
    }

    pub fn unset_key(&mut self, line: usize, bit: u8) {
        self.keyboard.unset_key(line, bit)
    }

    pub fn load_disk(&mut self, drive: usize, rom: Vec<u8>) {
        self.fdc.load_disk(drive, rom);
    }
}

impl Bus for StandardBus {
    fn read_byte(&mut self, port: u16) -> u8 {
        match port {
            _ if port & 0x4000 == 0 => self.crtc.read_byte(port),
            _ if port & 0x0800 == 0 => self.ppi.read_byte(&self.crtc, &self.psg, &self.tape, port),
            0xfb7e | 0xfb7f => self.fdc.read_byte(port),
            _ => {
                log::error!("Unhandled read from port {:#06x}", port);
                unimplemented!();
            }
        }
    }

    fn write_byte(&mut self, memory: &mut impl Mmu, port: u16, value: u8) {
        // TODO: do we need "value" or is it always the lower half of "port"?
        match port {
            _ if port & 0x8000 == 0 && port & 0x4000 != 0 => {
                self.gate_array.write_byte(memory, port, value)
            }
            _ if port & 0x4000 == 0 => self.crtc.write_byte(port, value),
            _ if port & 0xdf00 == 0xdf00 => memory.select_upper_rom(value),
            _ if port & 0xef00 == 0xef00 => (), // printer port (unsupported)
            _ if port & 0x0800 == 0 => self.ppi.write_byte(&mut self.keyboard, &mut self.psg, &mut self.tape, port, value),
            0xfa7e | 0xfb7f => self.fdc.write_byte(port, value),
            0xf8ff => (), // peripheral soft reset (ignored)
            _ => {
                log::error!("Unhandled write to port {:#06x}: {:#010b}", port, value);
                unimplemented!();
            }
        }
    }
}
