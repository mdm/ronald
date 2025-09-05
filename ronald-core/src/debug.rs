use std::{cell::RefCell, collections::HashMap};

use event::DebugEvent;

pub mod event;
pub mod view;

pub trait DebugEventSubscriber {
    fn on_event(&self, source: DebugSource, event: &DebugEvent);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubscriptionHandle {
    id: usize,
    source: DebugSource,
}

struct SubscriberEntry {
    id: usize,
    subscriber: Box<dyn DebugEventSubscriber>,
}

struct SubscriberRegistry {
    subscribers: HashMap<DebugSource, Vec<SubscriberEntry>>,
    next_id: usize,
}

impl SubscriberRegistry {
    fn new() -> Self {
        Self {
            subscribers: HashMap::new(),
            next_id: 0,
        }
    }

    fn subscribe(
        &mut self,
        source: DebugSource,
        sub: Box<dyn DebugEventSubscriber>,
    ) -> SubscriptionHandle {
        let id = self.next_id;
        self.next_id += 1;

        let handle = SubscriptionHandle { id, source };
        let entry = SubscriberEntry {
            id,
            subscriber: sub,
        };

        self.subscribers.entry(source).or_default().push(entry);
        handle
    }

    fn unsubscribe(&mut self, handle: SubscriptionHandle) {
        if let Some(source_subs) = self.subscribers.get_mut(&handle.source) {
            source_subs.retain(|entry| entry.id != handle.id);
        }
    }

    fn emit(&self, source: DebugSource, event: DebugEvent) {
        if let Some(list) = self.subscribers.get(&source) {
            for entry in list {
                entry.subscriber.on_event(source, &event);
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

pub fn subscribe(source: DebugSource, sub: Box<dyn DebugEventSubscriber>) -> SubscriptionHandle {
    REGISTRY.with(|reg| reg.borrow_mut().subscribe(source, sub))
}

pub fn unsubscribe(handle: SubscriptionHandle) {
    REGISTRY.with(|reg| reg.borrow_mut().unsubscribe(handle));
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

#[cfg(test)]
use std::rc::Rc;

#[cfg(test)]
pub struct TestSubscriber {
    events: Rc<RefCell<Vec<(DebugSource, DebugEvent)>>>,
}

#[cfg(test)]
impl TestSubscriber {
    pub fn new() -> (Self, Rc<RefCell<Vec<(DebugSource, DebugEvent)>>>) {
        let events = Rc::new(RefCell::new(Vec::new()));
        let subscriber = Self {
            events: events.clone(),
        };
        (subscriber, events)
    }
}

#[cfg(test)]
impl DebugEventSubscriber for TestSubscriber {
    fn on_event(&self, source: DebugSource, event: &DebugEvent) {
        self.events.borrow_mut().push((source, event.clone()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscribe_single_subscriber() {
        let (subscriber, events) = TestSubscriber::new();

        subscribe(DebugSource::Cpu, Box::new(subscriber));

        emit_event(
            DebugSource::Cpu,
            DebugEvent::Cpu(event::CpuDebugEvent::Register8Changed {
                register: crate::system::cpu::Register8::A,
                is: 0x42,
                was: 0x00,
            }),
        );

        let events = events.borrow();
        assert_eq!(events.len(), 1);
        assert!(matches!(&events[0], (DebugSource::Cpu, DebugEvent::Cpu(_))));
    }

    #[test]
    fn test_subscribe_multiple_subscribers_same_source() {
        let (subscriber1, events1) = TestSubscriber::new();
        let (subscriber2, events2) = TestSubscriber::new();

        subscribe(DebugSource::Memory, Box::new(subscriber1));
        subscribe(DebugSource::Memory, Box::new(subscriber2));

        emit_event(
            DebugSource::Memory,
            DebugEvent::Memory(event::MemoryDebugEvent::Test),
        );

        assert_eq!(events1.borrow().len(), 1);
        assert_eq!(events2.borrow().len(), 1);
    }

    #[test]
    fn test_subscribe_multiple_sources() {
        let (subscriber, events) = TestSubscriber::new();
        let (subscriber2, events2) = TestSubscriber::new();

        subscribe(DebugSource::Cpu, Box::new(subscriber));
        subscribe(DebugSource::Memory, Box::new(subscriber2));

        emit_event(
            DebugSource::Cpu,
            DebugEvent::Cpu(event::CpuDebugEvent::Register8Changed {
                register: crate::system::cpu::Register8::A,
                is: 0x42,
                was: 0x00,
            }),
        );
        emit_event(
            DebugSource::Memory,
            DebugEvent::Memory(event::MemoryDebugEvent::Test),
        );

        let events = events.borrow();
        assert_eq!(events.len(), 1);
        assert!(matches!(&events[0], (DebugSource::Cpu, DebugEvent::Cpu(_))));

        let events2 = events2.borrow();
        assert_eq!(events2.len(), 1);
        assert!(matches!(
            &events2[0],
            (DebugSource::Memory, DebugEvent::Memory(_))
        ));
    }

    #[test]
    fn test_unsubscribe_removes_subscriber() {
        let (subscriber, events) = TestSubscriber::new();

        let handle = subscribe(DebugSource::Cpu, Box::new(subscriber));

        emit_event(
            DebugSource::Cpu,
            DebugEvent::Cpu(event::CpuDebugEvent::Register8Changed {
                register: crate::system::cpu::Register8::A,
                is: 0x42,
                was: 0x00,
            }),
        );
        assert_eq!(events.borrow().len(), 1);

        unsubscribe(handle);

        emit_event(
            DebugSource::Cpu,
            DebugEvent::Cpu(event::CpuDebugEvent::Register8Changed {
                register: crate::system::cpu::Register8::A,
                is: 0x43,
                was: 0x42,
            }),
        );
        assert_eq!(events.borrow().len(), 1); // Should still be 1, no new events
    }

    #[test]
    fn test_unsubscribe_only_removes_matching_subscriber() {
        let (subscriber1, events1) = TestSubscriber::new();
        let (subscriber2, events2) = TestSubscriber::new();

        let handle1 = subscribe(DebugSource::Cpu, Box::new(subscriber1));
        let _handle2 = subscribe(DebugSource::Cpu, Box::new(subscriber2));

        unsubscribe(handle1);

        emit_event(
            DebugSource::Cpu,
            DebugEvent::Cpu(event::CpuDebugEvent::Register8Changed {
                register: crate::system::cpu::Register8::A,
                is: 0x42,
                was: 0x00,
            }),
        );

        assert_eq!(events1.borrow().len(), 0); // First subscriber removed
        assert_eq!(events2.borrow().len(), 1); // Second subscriber still active
    }

    #[test]
    fn test_unsubscribe_with_invalid_handle() {
        let invalid_handle = SubscriptionHandle {
            id: 999,
            source: DebugSource::Cpu,
        };
        unsubscribe(invalid_handle); // Should not panic
    }

    #[test]
    fn test_emit_with_no_subscribers() {
        emit_event(
            DebugSource::Cpu,
            DebugEvent::Cpu(event::CpuDebugEvent::Register8Changed {
                register: crate::system::cpu::Register8::A,
                is: 0x42,
                was: 0x00,
            }),
        ); // Should not panic
    }

    #[test]
    fn test_double_unsubscribe() {
        let (subscriber, _events) = TestSubscriber::new();
        let handle = subscribe(DebugSource::Cpu, Box::new(subscriber));

        unsubscribe(handle);
        unsubscribe(handle); // Should not panic
    }
}
