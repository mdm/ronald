use std::collections::HashMap;
use std::fmt;

use crate::debug::event::{CpuDebugEvent, MemoryDebugEvent};
use crate::debug::{DebugEvent, DebugSource, EventSubscription};
use crate::system::cpu::{Register16, Register8};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BreakpointId(usize);

pub trait Breakpoint {
    fn should_break(&self, source: DebugSource, event: &DebugEvent) -> bool;
    fn enabled(&self) -> bool;
    fn set_enabled(&mut self, enabled: bool);
    fn one_shot(&self) -> bool;
    fn set_one_shot(&mut self, one_shot: bool);
    fn triggered(&self) -> bool;
    fn set_triggered(&mut self, triggered: bool);
}

#[derive(Debug, Clone)]
pub struct CpuRegister8Breakpoint {
    pub register: Register8,
    pub value: Option<u8>,
    enabled: bool,
    one_shot: bool,
    triggered: bool,
}

impl CpuRegister8Breakpoint {
    pub fn new(register: Register8, value: Option<u8>) -> Self {
        Self {
            register,
            value,
            enabled: true,
            one_shot: false,
            triggered: false,
        }
    }
}

impl Breakpoint for CpuRegister8Breakpoint {
    fn should_break(&self, source: DebugSource, event: &DebugEvent) -> bool {
        if !self.enabled || source != DebugSource::Cpu {
            return false;
        }

        match event {
            DebugEvent::Cpu(CpuDebugEvent::Register8Written { register, is, .. }) => {
                *register == self.register && self.value.is_none_or(|v| v == *is)
            }
            _ => false,
        }
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn one_shot(&self) -> bool {
        self.one_shot
    }

    fn set_one_shot(&mut self, one_shot: bool) {
        self.one_shot = one_shot;
    }

    fn triggered(&self) -> bool {
        self.triggered
    }

    fn set_triggered(&mut self, triggered: bool) {
        self.triggered = triggered;
    }
}

impl fmt::Display for CpuRegister8Breakpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.value {
            Some(val) => write!(f, "{:?} = 0x{:02X}", self.register, val),
            None => write!(f, "{:?} written", self.register),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CpuRegister16Breakpoint {
    pub register: Register16,
    pub value: Option<u16>,
    enabled: bool,
    one_shot: bool,
    triggered: bool,
}

impl CpuRegister16Breakpoint {
    pub fn new(register: Register16, value: Option<u16>) -> Self {
        Self {
            register,
            value,
            enabled: true,
            one_shot: false,
            triggered: false,
        }
    }

    pub fn pc_breakpoint(address: u16) -> Self {
        Self::new(Register16::PC, Some(address))
    }

    pub fn pc_step() -> Self {
        let mut bp = Self::new(Register16::PC, None);
        bp.one_shot = true;
        bp
    }
}

impl Breakpoint for CpuRegister16Breakpoint {
    fn should_break(&self, source: DebugSource, event: &DebugEvent) -> bool {
        if !self.enabled || source != DebugSource::Cpu {
            return false;
        }

        match event {
            DebugEvent::Cpu(CpuDebugEvent::Register16Written { register, is, .. }) => {
                *register == self.register && self.value.is_none_or(|v| v == *is)
            }
            _ => false,
        }
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn one_shot(&self) -> bool {
        self.one_shot
    }

    fn set_one_shot(&mut self, one_shot: bool) {
        self.one_shot = one_shot;
    }

    fn triggered(&self) -> bool {
        self.triggered
    }

    fn set_triggered(&mut self, triggered: bool) {
        self.triggered = triggered;
    }
}

impl fmt::Display for CpuRegister16Breakpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.value {
            Some(val) => write!(f, "{:?} = 0x{:04X}", self.register, val),
            None => write!(f, "{:?} written", self.register),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryBreakpoint {
    pub address: u16,
    pub on_read: bool,
    pub on_write: bool,
    pub value: Option<u8>,
    enabled: bool,
    one_shot: bool,
    triggered: bool,
}

impl MemoryBreakpoint {
    pub fn new(address: u16, on_read: bool, on_write: bool, value: Option<u8>) -> Self {
        Self {
            address,
            on_read,
            on_write,
            value,
            enabled: true,
            one_shot: false,
            triggered: false,
        }
    }
}

impl Breakpoint for MemoryBreakpoint {
    fn should_break(&self, source: DebugSource, event: &DebugEvent) -> bool {
        if !self.enabled || source != DebugSource::Memory {
            return false;
        }

        match event {
            DebugEvent::Memory(MemoryDebugEvent::MemoryRead { address, value, .. }) => {
                self.on_read
                    && *address == self.address as usize
                    && self.value.is_none_or(|v| v == *value)
            }
            DebugEvent::Memory(MemoryDebugEvent::MemoryWritten { address, is, .. }) => {
                self.on_write
                    && *address == self.address as usize
                    && self.value.is_none_or(|v| v == *is)
            }
            _ => false,
        }
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn one_shot(&self) -> bool {
        self.one_shot
    }

    fn set_one_shot(&mut self, one_shot: bool) {
        self.one_shot = one_shot;
    }

    fn triggered(&self) -> bool {
        self.triggered
    }

    fn set_triggered(&mut self, triggered: bool) {
        self.triggered = triggered;
    }
}

impl fmt::Display for MemoryBreakpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let access = match (self.on_read, self.on_write) {
            (true, true) => "access",
            (true, false) => "read",
            (false, true) => "write",
            (false, false) => "never",
        };

        match self.value {
            Some(val) => write!(f, "0x{:04X} {} = 0x{:02X}", self.address, access, val),
            None => write!(f, "0x{:04X} {}", self.address, access),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AnyBreakpoint {
    CpuRegister8(CpuRegister8Breakpoint),
    CpuRegister16(CpuRegister16Breakpoint),
    Memory(MemoryBreakpoint),
}

impl AnyBreakpoint {
    pub fn pc_breakpoint(address: u16) -> Self {
        Self::CpuRegister16(CpuRegister16Breakpoint::pc_breakpoint(address))
    }

    pub fn pc_step() -> Self {
        Self::CpuRegister16(CpuRegister16Breakpoint::pc_step())
    }

    pub fn register8_breakpoint(register: Register8, value: Option<u8>) -> Self {
        Self::CpuRegister8(CpuRegister8Breakpoint::new(register, value))
    }

    pub fn register16_breakpoint(register: Register16, value: Option<u16>) -> Self {
        Self::CpuRegister16(CpuRegister16Breakpoint::new(register, value))
    }

    pub fn memory_breakpoint(
        address: u16,
        on_read: bool,
        on_write: bool,
        value: Option<u8>,
    ) -> Self {
        Self::Memory(MemoryBreakpoint::new(address, on_read, on_write, value))
    }
}

impl Breakpoint for AnyBreakpoint {
    fn should_break(&self, source: DebugSource, event: &DebugEvent) -> bool {
        match self {
            AnyBreakpoint::CpuRegister8(bp) => bp.should_break(source, event),
            AnyBreakpoint::CpuRegister16(bp) => bp.should_break(source, event),
            AnyBreakpoint::Memory(bp) => bp.should_break(source, event),
        }
    }

    fn enabled(&self) -> bool {
        match self {
            AnyBreakpoint::CpuRegister8(bp) => bp.enabled(),
            AnyBreakpoint::CpuRegister16(bp) => bp.enabled(),
            AnyBreakpoint::Memory(bp) => bp.enabled(),
        }
    }

    fn set_enabled(&mut self, enabled: bool) {
        match self {
            AnyBreakpoint::CpuRegister8(bp) => bp.set_enabled(enabled),
            AnyBreakpoint::CpuRegister16(bp) => bp.set_enabled(enabled),
            AnyBreakpoint::Memory(bp) => bp.set_enabled(enabled),
        }
    }

    fn one_shot(&self) -> bool {
        match self {
            AnyBreakpoint::CpuRegister8(bp) => bp.one_shot(),
            AnyBreakpoint::CpuRegister16(bp) => bp.one_shot(),
            AnyBreakpoint::Memory(bp) => bp.one_shot(),
        }
    }

    fn set_one_shot(&mut self, one_shot: bool) {
        match self {
            AnyBreakpoint::CpuRegister8(bp) => bp.set_one_shot(one_shot),
            AnyBreakpoint::CpuRegister16(bp) => bp.set_one_shot(one_shot),
            AnyBreakpoint::Memory(bp) => bp.set_one_shot(one_shot),
        }
    }

    fn triggered(&self) -> bool {
        match self {
            AnyBreakpoint::CpuRegister8(bp) => bp.triggered(),
            AnyBreakpoint::CpuRegister16(bp) => bp.triggered(),
            AnyBreakpoint::Memory(bp) => bp.triggered(),
        }
    }

    fn set_triggered(&mut self, triggered: bool) {
        match self {
            AnyBreakpoint::CpuRegister8(bp) => bp.set_triggered(triggered),
            AnyBreakpoint::CpuRegister16(bp) => bp.set_triggered(triggered),
            AnyBreakpoint::Memory(bp) => bp.set_triggered(triggered),
        }
    }
}

impl fmt::Display for AnyBreakpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnyBreakpoint::CpuRegister8(bp) => bp.fmt(f),
            AnyBreakpoint::CpuRegister16(bp) => bp.fmt(f),
            AnyBreakpoint::Memory(bp) => bp.fmt(f),
        }
    }
}

pub struct BreakpointManager {
    breakpoints: HashMap<BreakpointId, AnyBreakpoint>,
    next_id: usize,
    subscription: EventSubscription,
}

impl BreakpointManager {
    pub fn new() -> Self {
        Self {
            breakpoints: HashMap::new(),
            next_id: 0,
            subscription: EventSubscription::new(DebugSource::Any),
        }
    }

    pub fn add_breakpoint(&mut self, breakpoint: AnyBreakpoint) -> BreakpointId {
        let id = BreakpointId(self.next_id);
        self.next_id += 1;
        self.breakpoints.insert(id, breakpoint);
        id
    }

    pub fn remove_breakpoint(&mut self, id: BreakpointId) -> bool {
        self.breakpoints.remove(&id).is_some()
    }

    pub fn enable_breakpoint(&mut self, id: BreakpointId, enabled: bool) -> bool {
        if let Some(breakpoint) = self.breakpoints.get_mut(&id) {
            breakpoint.set_enabled(enabled);
            true
        } else {
            false
        }
    }

    pub fn breakpoint(&self, id: BreakpointId) -> Option<&AnyBreakpoint> {
        self.breakpoints.get(&id)
    }

    pub fn breakpoint_mut(&mut self, id: BreakpointId) -> Option<&mut AnyBreakpoint> {
        self.breakpoints.get_mut(&id)
    }

    pub fn breakpoints_iter(&self) -> impl Iterator<Item = (&BreakpointId, &AnyBreakpoint)> {
        self.breakpoints.iter()
    }

    pub fn breakpoints_iter_mut(
        &mut self,
    ) -> impl Iterator<Item = (&BreakpointId, &mut AnyBreakpoint)> {
        self.breakpoints.iter_mut()
    }

    pub fn clear_all(&mut self) {
        self.breakpoints.clear();
    }

    pub fn any_triggered(&self) -> bool {
        self.breakpoints.values().any(|bp| bp.triggered())
    }

    pub fn prepare_breakpoints(&mut self) {
        // Remove triggered one-shot breakpoints
        self.breakpoints
            .retain(|_id, bp| !bp.triggered() || !bp.one_shot());

        for (_id, breakpoint) in self.breakpoints.iter_mut() {
            breakpoint.set_triggered(false);
        }
    }

    pub fn evaluate_breakpoints(&mut self) {
        self.subscription.with_events(|record| {
            for (_id, breakpoint) in self.breakpoints.iter_mut() {
                if breakpoint.should_break(record.source, &record.event) {
                    breakpoint.set_triggered(true);
                }
            }
        });
    }
}

impl Default for BreakpointManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pc_breakpoint_triggers_on_correct_address() {
        let bp = AnyBreakpoint::pc_breakpoint(0x1000);

        let event = DebugEvent::Cpu(CpuDebugEvent::Register16Written {
            register: Register16::PC,
            is: 0x1000,
            was: 0x0000,
        });

        assert!(bp.should_break(DebugSource::Cpu, &event));
    }

    #[test]
    fn test_pc_breakpoint_ignores_wrong_address() {
        let bp = AnyBreakpoint::pc_breakpoint(0x1000);

        let event = DebugEvent::Cpu(CpuDebugEvent::Register16Written {
            register: Register16::PC,
            is: 0x2000,
            was: 0x0000,
        });

        assert!(!bp.should_break(DebugSource::Cpu, &event));
    }

    #[test]
    fn test_pc_breakpoint_ignores_wrong_register() {
        let bp = AnyBreakpoint::pc_breakpoint(0x1000);

        let event = DebugEvent::Cpu(CpuDebugEvent::Register16Written {
            register: Register16::SP,
            is: 0x1000,
            was: 0x0000,
        });

        assert!(!bp.should_break(DebugSource::Cpu, &event));
    }

    #[test]
    fn test_pc_step_triggers_on_any_pc_change() {
        let bp = AnyBreakpoint::pc_step();

        let event1 = DebugEvent::Cpu(CpuDebugEvent::Register16Written {
            register: Register16::PC,
            is: 0x1000,
            was: 0x0000,
        });

        let event2 = DebugEvent::Cpu(CpuDebugEvent::Register16Written {
            register: Register16::PC,
            is: 0x2000,
            was: 0x1000,
        });

        assert!(bp.should_break(DebugSource::Cpu, &event1));
        assert!(bp.should_break(DebugSource::Cpu, &event2));
        assert!(bp.one_shot());
    }

    #[test]
    fn test_register8_breakpoint_with_specific_value() {
        let bp = AnyBreakpoint::register8_breakpoint(Register8::A, Some(0x42));

        let correct_event = DebugEvent::Cpu(CpuDebugEvent::Register8Written {
            register: Register8::A,
            is: 0x42,
            was: 0x00,
        });

        let wrong_value_event = DebugEvent::Cpu(CpuDebugEvent::Register8Written {
            register: Register8::A,
            is: 0x43,
            was: 0x00,
        });

        let wrong_register_event = DebugEvent::Cpu(CpuDebugEvent::Register8Written {
            register: Register8::B,
            is: 0x42,
            was: 0x00,
        });

        assert!(bp.should_break(DebugSource::Cpu, &correct_event));
        assert!(!bp.should_break(DebugSource::Cpu, &wrong_value_event));
        assert!(!bp.should_break(DebugSource::Cpu, &wrong_register_event));
    }

    #[test]
    fn test_register8_breakpoint_any_value() {
        let bp = AnyBreakpoint::register8_breakpoint(Register8::A, None);

        let event1 = DebugEvent::Cpu(CpuDebugEvent::Register8Written {
            register: Register8::A,
            is: 0x42,
            was: 0x00,
        });

        let event2 = DebugEvent::Cpu(CpuDebugEvent::Register8Written {
            register: Register8::A,
            is: 0xFF,
            was: 0x42,
        });

        assert!(bp.should_break(DebugSource::Cpu, &event1));
        assert!(bp.should_break(DebugSource::Cpu, &event2));
    }

    #[test]
    fn test_memory_breakpoint_write_only() {
        let bp = AnyBreakpoint::memory_breakpoint(0x4000, false, true, None);

        let write_event = DebugEvent::Memory(MemoryDebugEvent::MemoryWritten {
            address: 0x4000,
            is: 0x55,
            was: 0x00,
        });

        let read_event = DebugEvent::Memory(MemoryDebugEvent::MemoryRead {
            address: 0x4000,
            value: 0x55,
        });

        assert!(bp.should_break(DebugSource::Memory, &write_event));
        assert!(!bp.should_break(DebugSource::Memory, &read_event));
    }

    #[test]
    fn test_memory_breakpoint_read_only() {
        let bp = AnyBreakpoint::memory_breakpoint(0x4000, true, false, None);

        let write_event = DebugEvent::Memory(MemoryDebugEvent::MemoryWritten {
            address: 0x4000,
            is: 0x55,
            was: 0x00,
        });

        let read_event = DebugEvent::Memory(MemoryDebugEvent::MemoryRead {
            address: 0x4000,
            value: 0x55,
        });

        assert!(!bp.should_break(DebugSource::Memory, &write_event));
        assert!(bp.should_break(DebugSource::Memory, &read_event));
    }

    #[test]
    fn test_memory_breakpoint_with_value_filter() {
        let bp = AnyBreakpoint::memory_breakpoint(0x4000, false, true, Some(0x42));

        let correct_write = DebugEvent::Memory(MemoryDebugEvent::MemoryWritten {
            address: 0x4000,
            is: 0x42,
            was: 0x00,
        });

        let wrong_value_write = DebugEvent::Memory(MemoryDebugEvent::MemoryWritten {
            address: 0x4000,
            is: 0x43,
            was: 0x00,
        });

        assert!(bp.should_break(DebugSource::Memory, &correct_write));
        assert!(!bp.should_break(DebugSource::Memory, &wrong_value_write));
    }

    #[test]
    fn test_breakpoint_enabled_disabled() {
        let mut bp = AnyBreakpoint::pc_breakpoint(0x1000);
        assert!(bp.enabled());

        bp.set_enabled(false);
        assert!(!bp.enabled());

        let event = DebugEvent::Cpu(CpuDebugEvent::Register16Written {
            register: Register16::PC,
            is: 0x1000,
            was: 0x0000,
        });

        assert!(!bp.should_break(DebugSource::Cpu, &event));

        bp.set_enabled(true);
        assert!(bp.should_break(DebugSource::Cpu, &event));
    }

    #[test]
    fn test_breakpoint_one_shot_flag() {
        let mut bp = AnyBreakpoint::pc_breakpoint(0x1000);
        assert!(!bp.one_shot());

        bp.set_one_shot(true);
        assert!(bp.one_shot());

        bp.set_one_shot(false);
        assert!(!bp.one_shot());
    }

    #[test]
    fn test_breakpoint_manager_add_remove() {
        let mut manager = BreakpointManager::new();
        assert_eq!(manager.breakpoints.borrow().len(), 0);

        let id1 = manager.add_breakpoint(AnyBreakpoint::pc_breakpoint(0x1000));
        let id2 = manager.add_breakpoint(AnyBreakpoint::pc_breakpoint(0x2000));
        assert_eq!(manager.breakpoints.borrow().len(), 2);

        assert!(manager.remove_breakpoint(id1));
        assert_eq!(manager.breakpoints.borrow().len(), 1);

        assert!(!manager.remove_breakpoint(id1)); // Already removed
        assert_eq!(manager.breakpoints.borrow().len(), 1);

        assert!(manager.remove_breakpoint(id2));
        assert_eq!(manager.breakpoints.borrow().len(), 0);
    }

    #[test]
    fn test_breakpoint_manager_enable_disable() {
        let mut manager = BreakpointManager::new();
        let id = manager.add_breakpoint(AnyBreakpoint::pc_breakpoint(0x1000));

        assert!(manager.get_breakpoint(id).unwrap().enabled());

        assert!(manager.enable_breakpoint(id, false));
        assert!(!manager.get_breakpoint(id).unwrap().enabled());

        assert!(manager.enable_breakpoint(id, true));
        assert!(manager.get_breakpoint(id).unwrap().enabled());

        let invalid_id = BreakpointId(999);
        assert!(!manager.enable_breakpoint(invalid_id, false));
    }

    #[test]
    fn test_breakpoint_display_formats() {
        let pc_bp = AnyBreakpoint::pc_breakpoint(0x1000);
        assert_eq!(pc_bp.to_string(), "PC = 0x1000");

        let reg8_specific = AnyBreakpoint::register8_breakpoint(Register8::A, Some(0x42));
        assert_eq!(reg8_specific.to_string(), "A = 0x42");

        let reg8_any = AnyBreakpoint::register8_breakpoint(Register8::B, None);
        assert_eq!(reg8_any.to_string(), "B written");

        let mem_read = AnyBreakpoint::memory_breakpoint(0x4000, true, false, None);
        assert_eq!(mem_read.to_string(), "0x4000 read");

        let mem_write = AnyBreakpoint::memory_breakpoint(0x4000, false, true, None);
        assert_eq!(mem_write.to_string(), "0x4000 write");

        let mem_access = AnyBreakpoint::memory_breakpoint(0x4000, true, true, None);
        assert_eq!(mem_access.to_string(), "0x4000 access");

        let mem_write_value = AnyBreakpoint::memory_breakpoint(0x4000, false, true, Some(0xFF));
        assert_eq!(mem_write_value.to_string(), "0x4000 write = 0xFF");
    }

    #[test]
    fn test_breakpoint_ignores_wrong_debug_source() {
        let cpu_bp = AnyBreakpoint::pc_breakpoint(0x1000);
        let mem_event = DebugEvent::Memory(MemoryDebugEvent::MemoryRead {
            address: 0x1000,
            value: 0x42,
        });
        assert!(!cpu_bp.should_break(DebugSource::Memory, &mem_event));

        let mem_bp = AnyBreakpoint::memory_breakpoint(0x1000, true, false, None);
        let cpu_event = DebugEvent::Cpu(CpuDebugEvent::Register16Written {
            register: Register16::PC,
            is: 0x1000,
            was: 0x0000,
        });
        assert!(!mem_bp.should_break(DebugSource::Cpu, &cpu_event));
    }

    #[test]
    fn test_breakpoint_manager_one_shot_removal() {
        let mut manager = BreakpointManager::new();
        let mut callback_called = false;

        let id = manager.add_breakpoint(AnyBreakpoint::pc_step());
        assert_eq!(manager.breakpoints.borrow().len(), 1);
        assert!(manager.get_breakpoint(id).unwrap().one_shot());

        let event = DebugEvent::Cpu(CpuDebugEvent::Register16Written {
            register: Register16::PC,
            is: 0x1000,
            was: 0x0000,
        });

        manager.on_event(DebugSource::Cpu, &event);

        // One-shot breakpoint should be removed after triggering
        assert_eq!(manager.breakpoints.borrow().len(), 0);
        assert!(manager.get_breakpoint(id).is_none());
    }
}
