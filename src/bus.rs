use crate::crtc;
use crate::fdc;
use crate::gate_array;
use crate::memory;
use crate::pio;
use crate::screen;


pub struct Bus {
    crtc: crtc::CRTController,
    fdc: fdc::FloppyDiskController,
    gate_array: gate_array::GateArray,
    pio: pio::PeripheralInterface,
}

impl Bus {
    pub fn new(
        crtc: crtc::CRTController,
        fdc: fdc::FloppyDiskController,
        gate_array: gate_array::GateArray,
        pio: pio::PeripheralInterface
    ) -> Bus {
        Bus {
            crtc,
            fdc,
            gate_array,
            pio
        }
    }

    pub fn read_byte(&self, port: u16) -> u8 {
        // TODO: map read to devices
        match port {
            _ if 0x4000 & port == 0 => self.crtc.read_byte(port),
            _ if 0x0800 & port == 0 => self.pio.read_byte(port),
            0xfb7e | 0xfb7f => self.fdc.read_byte(port),
            _ => unimplemented!(),
        }
    }

    pub fn write_byte(&mut self, memory: &mut memory::Memory, port: u16, value: u8) {
        // TODO: map write to devices
        match port {
            _ if 0x4000 & port == 0 => self.crtc.write_byte(port, value),
            _ if 0x8000 & port == 0 && 0x4000 & port != 0 => self.gate_array.write_byte(memory, port, value),
            _ if 0x0800 & port == 0 => self.pio.write_byte(port, value),
            0xfa7e | 0xfb7f => self.fdc.write_byte(port, value),
            _ => unimplemented!(),
        }
    }

    pub fn step(&mut self, memory: &mut memory::Memory, _screen: &mut screen::Screen) -> bool {
        self.crtc.step();
        self.gate_array.step(memory, &self.crtc)
    }
}
