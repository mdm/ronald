use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::system::clock::MasterClockTick;
use crate::system::memory::{AnyMemory, MemManage, MemRead};
use crate::{AudioSink, VideoSink};

pub mod crtc;
mod fdc;
pub mod gate_array;
pub mod keyboard; // TODO: refactor to not use pub
mod ppi;
mod psg;
pub mod screen; // TODO: refactor to not use pub
mod tape;

use crtc::CrtController;
use fdc::FloppyDiskController;
use gate_array::GateArray;
use keyboard::Keyboard;
use ppi::PeripheralInterface;
use psg::SoundGenerator;
use screen::Screen;
use tape::TapeController;

pub trait Bus: Default {
    // TODO: replace by BusDevice
    fn read_byte(&mut self, port: u16) -> u8;
    fn write_byte(&mut self, memory: &mut impl MemManage, port: u16, value: u8);
    fn step(
        &mut self,
        memory: &mut (impl MemRead + MemManage),
        video: &mut impl VideoSink,
        audio: &mut impl AudioSink,
        master_clock: MasterClockTick,
    ) -> bool;
    fn acknowledge_interrupt(&mut self);
    fn set_key(&mut self, line: usize, bit: u8);
    fn unset_key(&mut self, line: usize, bit: u8);
    fn load_disk(&mut self, drive: usize, rom: Vec<u8>, path: PathBuf);
}

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StandardBus<C, G>
where
    C: CrtController,
    G: GateArray,
{
    crtc: C,
    fdc: FloppyDiskController,
    gate_array: G,
    keyboard: Keyboard,
    ppi: PeripheralInterface,
    psg: SoundGenerator,
    screen: Screen,
    tape: TapeController,
}

impl<C, G> Bus for StandardBus<C, G>
where
    C: CrtController,
    G: GateArray,
{
    fn read_byte(&mut self, port: u16) -> u8 {
        match port {
            _ if port & 0x4000 == 0 => self.crtc.read_byte(port),
            _ if port & 0x0800 == 0 => self.ppi.read_byte(&self.crtc, &self.psg, &self.tape, port),
            0xfb7e | 0xfb7f => self.fdc.read_byte(port),
            _ => {
                log::error!("Unhandled read from port {port:#06x}");
                unimplemented!();
            }
        }
    }

    fn write_byte(&mut self, memory: &mut impl MemManage, port: u16, value: u8) {
        // TODO: do we need "value" or is it always the lower half of "port"?
        match port {
            _ if port & 0x8000 == 0 && port & 0x4000 != 0 => {
                self.gate_array.write_byte(memory, port, value)
            }
            _ if port & 0x4000 == 0 => self.crtc.write_byte(port, value),
            _ if port & 0xdf00 == 0xdf00 => memory.select_upper_rom(value),
            _ if port & 0xef00 == 0xef00 => (), // printer port (unsupported)
            _ if port & 0x0800 == 0 => self.ppi.write_byte(
                &mut self.keyboard,
                &mut self.psg,
                &mut self.tape,
                port,
                value,
            ),
            0xfa7e | 0xfb7f => self.fdc.write_byte(port, value),
            0xf8ff => (), // peripheral soft reset (ignored)
            _ => {
                log::error!("Unhandled write to port {port:#06x}: {value:#010b}");
                unimplemented!();
            }
        }
    }

    fn step(
        &mut self,
        memory: &mut (impl MemRead + MemManage),
        video: &mut impl VideoSink,
        audio: &mut impl AudioSink,
        master_clock: MasterClockTick,
    ) -> bool {
        self.psg.step(audio);
        self.crtc.step();
        self.gate_array
            .step(&self.crtc, memory, &mut self.screen, video)
    }

    fn acknowledge_interrupt(&mut self) {
        self.gate_array.acknowledge_interrupt();
    }

    fn set_key(&mut self, line: usize, bit: u8) {
        self.keyboard.set_key(line, bit)
    }

    fn unset_key(&mut self, line: usize, bit: u8) {
        self.keyboard.unset_key(line, bit)
    }

    fn load_disk(&mut self, drive: usize, rom: Vec<u8>, path: PathBuf) {
        self.fdc.load_disk(drive, rom, path);
    }
}
