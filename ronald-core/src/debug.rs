use std::{cell::RefCell, collections::HashMap};

use event::DebugEvent;

pub mod event;
pub mod view;

pub trait DebugEventSubscriber {
    fn on_event(&self, source: DebugSource, event: &DebugEvent);
}

struct SubscriberRegistry {
    subscribers: HashMap<DebugSource, Vec<Box<dyn DebugEventSubscriber>>>,
}

impl SubscriberRegistry {
    fn new() -> Self {
        Self {
            subscribers: HashMap::new(),
        }
    }

    fn subscribe(&mut self, source: DebugSource, sub: Box<dyn DebugEventSubscriber>) {
        self.subscribers.entry(source).or_default().push(sub);
    }

    fn unsubscribe(&mut self, source: DebugSource, sub: Box<dyn DebugEventSubscriber>) {
        if let Some(source_subs) = self.subscribers.get_mut(&source) {
            source_subs.retain(|s| !std::ptr::eq(&**s, &*sub));
        }
    }

    fn emit(&self, source: DebugSource, event: DebugEvent) {
        if let Some(list) = self.subscribers.get(&source) {
            for sub in list {
                sub.on_event(source, &event);
            }
        }
    }
}

thread_local! {
    static REGISTRY: RefCell<SubscriberRegistry> = RefCell::new(SubscriberRegistry::new());
}

fn emit_event(source: DebugSource, event: DebugEvent) {
    REGISTRY.with(|reg| reg.borrow().emit(source, event));
}

pub fn subscribe(source: DebugSource, sub: Box<dyn DebugEventSubscriber>) {
    REGISTRY.with(|reg| reg.borrow_mut().subscribe(source, sub));
}

pub fn unsubscribe(source: DebugSource, sub: Box<dyn DebugEventSubscriber>) {
    REGISTRY.with(|reg| reg.borrow_mut().subscribe(source, sub));
}

pub trait Snapshotable {
    type View;

    fn debug_view(&self) -> Self::View;
}

pub trait Debuggable: Snapshotable {
    const SOURCE: DebugSource;
    type Event: Into<DebugEvent>;

    fn emit_debug_event(&self, event: Self::Event) {
        emit_event(Self::SOURCE, event.into());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DebugSource {
    Any,
    Cpu,
    Memory,
    Crtc,
    GateArray,
    Fdc,
    Ppi,
    Psg,
    Tape,
}
