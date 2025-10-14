use crate::system::bus::crtc::Register as CrtcRegister;
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
pub enum CrtcDebugEvent {
    RegisterSelected {
        register: CrtcRegister,
    },
    RegisterWritten {
        register: CrtcRegister,
        is: u8,
        was: u8,
    },
    HorizontalSyncStart {
        horizontal_counter: u8,
        character_row: u8,
        scanline: u8,
    },
    HorizontalSyncEnd {
        horizontal_counter: u8,
        character_row: u8,
        scanline: u8,
    },
    VerticalSyncStart {
        character_row: u8,
    },
    VerticalSyncEnd {
        character_row: u8,
    },
    FrameStart,
    DisplayEnableChanged {
        enabled: bool,
        horizontal_counter: u8,
        character_row: u8,
    },
    ScanlineStart {
        scanline: u8,
        character_row: u8,
    },
    CharacterRowStart {
        row: u8,
    },
}

impl From<CrtcDebugEvent> for DebugEvent {
    fn from(event: CrtcDebugEvent) -> Self {
        DebugEvent::Crtc(event)
    }
}

#[derive(Debug, Clone)]
pub enum GateArrayDebugEvent {
    ScreenModeChanged {
        is: u8,
        was: u8,
        applied: bool,
    },
    PenSelected {
        pen: usize,
    },
    PenColorChanged {
        pen: usize,
        is: u8,
        was: u8,
    },
    InterruptGenerated,
    RomConfigChanged {
        lower_rom_enabled: bool,
        upper_rom_enabled: bool,
    },
}

impl From<GateArrayDebugEvent> for DebugEvent {
    fn from(event: GateArrayDebugEvent) -> Self {
        DebugEvent::GateArray(event)
    }
}

#[derive(Debug, Clone)]
pub enum FdcDebugEvent {}

#[derive(Debug, Clone)]
pub enum PpiDebugEvent {}

#[derive(Debug, Clone)]
pub enum PsgDebugEvent {}

#[derive(Debug, Clone)]
pub enum TapeDebugEvent {}
