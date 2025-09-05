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

#[derive(Debug, Clone)]
pub enum MemoryDebugEvent {
    #[cfg(test)]
    Test,
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
