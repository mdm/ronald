use crate::system::cpu::{Register16, Register8};

/// A DebugEvent is any internal state change and any input or output
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum CpuDebugEvent {
    Register8Written {
        register: Register8,
        is: u8,
        was: u8,
    },
    Register16Written {
        register: Register16,
        is: u16,
        was: u16,
    },
    ShadowRegister16Written {
        register: Register16,
        is: u16,
        was: u16,
    },
    CallFetched {
        interrupt: bool,
    },
    ReturnFetched {
        interrupt: bool,
    },
}

impl From<CpuDebugEvent> for DebugEvent {
    fn from(event: CpuDebugEvent) -> Self {
        DebugEvent::Cpu(event)
    }
}

#[derive(Debug, Clone)]
pub enum MemoryDebugEvent {
    MemoryRead { address: usize, value: u8 },
    MemoryWritten { address: usize, is: u8, was: u8 },
}

impl From<MemoryDebugEvent> for DebugEvent {
    fn from(event: MemoryDebugEvent) -> Self {
        DebugEvent::Memory(event)
    }
}

#[derive(Debug, Clone)]
pub enum CrtcDebugEvent {}

#[derive(Debug, Clone)]
pub enum GateArrayDebugEvent {}

#[derive(Debug, Clone)]
pub enum FdcDebugEvent {}

#[derive(Debug, Clone)]
pub enum PpiDebugEvent {}

#[derive(Debug, Clone)]
pub enum PsgDebugEvent {}

#[derive(Debug, Clone)]
pub enum TapeDebugEvent {}
