use crate::crtc;
use crate::fdc;
use crate::gate_array;
use crate::memory;
use crate::ppi;

use std::cell::RefCell;
use std::rc::Rc;

pub trait Bus {
    fn read_byte(&self, port: u16) -> u8;
    fn write_byte(&mut self, port: u16, value: u8);
}

pub struct DummyBus {}

impl DummyBus {
    pub fn new() -> DummyBus {
        DummyBus {}
    }
}

impl Bus for DummyBus {
    fn read_byte(&self, _port: u16) -> u8 {
        unimplemented!()
    }

    fn write_byte(&mut self, _port: u16, _value: u8) {
        unimplemented!();
    }
}

pub type StandardBusShared = Rc<RefCell<StandardBus>>;

pub struct StandardBus {
    crtc: crtc::CRTControllerShared,
    fdc: fdc::FloppyDiskControllerShared,
    gate_array: gate_array::GateArrayShared,
    memory: memory::MemoryShared,
    ppi: ppi::PeripheralInterfaceShared,
}

impl StandardBus {
    pub fn new_shared(
        crtc: crtc::CRTControllerShared,
        fdc: fdc::FloppyDiskControllerShared,
        gate_array: gate_array::GateArrayShared,
        memory: memory::MemoryShared,
        ppi: ppi::PeripheralInterfaceShared,
    ) -> StandardBusShared {
        let bus = StandardBus {
            crtc,
            fdc,
            gate_array,
            memory,
            ppi,
        };

        Rc::new(RefCell::new(bus))
    }

    pub fn step(&mut self) -> bool {
        self.crtc.borrow_mut().step();
        self.gate_array.borrow_mut().step()
    }

    pub fn acknowledge_interrupt(&mut self) {
        self.gate_array.borrow_mut().acknowledge_interrupt();
    }
}

impl Bus for StandardBus {
    fn read_byte(&self, port: u16) -> u8 {
        match port {
            _ if port & 0x4000 == 0 => self.crtc.borrow().read_byte(port),
            _ if port & 0x0800 == 0 => self.ppi.borrow().read_byte(port),
            0xfb7e | 0xfb7f => self.fdc.borrow_mut().read_byte(port),
            _ => {
                log::error!("Unhandled read from port {:#06x}", port);
                unimplemented!();
            }
        }
    }

    fn write_byte(&mut self, port: u16, value: u8) {
        // TODO: do we need "value" or is it always the lower half of "port"?
        match port {
            _ if port & 0x8000 == 0 && port & 0x4000 != 0 => {
                self.gate_array.borrow_mut().write_byte(port, value)
            }
            _ if port & 0x4000 == 0 => self.crtc.borrow_mut().write_byte(port, value),
            _ if port & 0xdf00 == 0xdf00 => self.memory.borrow_mut().select_upper_rom(value),
            _ if port & 0xef00 == 0xef00 => (), // printer port (unsupported)
            _ if port & 0x0800 == 0 => self.ppi.borrow_mut().write_byte(port, value),
            0xfa7e | 0xfb7f => self.fdc.borrow_mut().write_byte(port, value),
            0xf8ff => (), // peripheral soft reset (ignored)
            _ => {
                log::error!("Unhandled write to port {:#06x}: {:#010b}", port, value);
                unimplemented!();
            }
        }
    }
}
