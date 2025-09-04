use crate::system::cpu::{Register16, Register8};

/// A DebugEvent is any internal state change and any input or output
pub enum DebugEvent {
    Cpu(CpuDebugEvent),
    Memory(MemoryDebugEvent),
    Crtc(CrtcDebugEvent),
    GateArray(GateArrayDebugEvent),
    Fdc(FdcDebugEvent),
    Ppi(PpiDebugEvent),
    Psg(PsgDebugEvent),
    Tape(TapeDebugEvent),
}

pub enum CpuDebugEvent {
    Register8Changed {
        register: Register8,
        is: u8,
        was: u8,
    },
    Register16Changed {
        register: Register16,
        is: u16,
        was: u16,
    },
    ShadowRegister16Swapped {
        register: Register16,
        is: u16,
        was: u16,
    },
}

impl From<CpuDebugEvent> for DebugEvent {
    fn from(event: CpuDebugEvent) -> Self {
        DebugEvent::Cpu(event)
    }
}

pub enum MemoryDebugEvent {}

pub enum CrtcDebugEvent {}

pub enum GateArrayDebugEvent {}

pub enum FdcDebugEvent {}

pub enum PpiDebugEvent {}

pub enum PsgDebugEvent {}

pub enum TapeDebugEvent {}
