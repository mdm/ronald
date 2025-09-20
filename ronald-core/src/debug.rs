use std::{cell::RefCell, collections::HashMap};

use event::DebugEvent;

use crate::system::clock::MasterClockTick;

pub mod breakpoint;
pub mod event;
pub mod view;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EventSequence(u64);

impl EventSequence {
    fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

pub struct EventRecord {
    sequence: EventSequence,
    pub source: DebugSource,
    pub event: DebugEvent,
    pub master_clock: MasterClockTick,
}

struct EventLog {
    events: Vec<EventRecord>,
    current_sequence: EventSequence,
}

impl EventLog {
    fn new() -> Self {
        Self {
            events: Vec::new(),
            current_sequence: EventSequence(0),
        }
    }

    fn append(&mut self, source: DebugSource, event: DebugEvent, master_clock: MasterClockTick) {
        let sequence = self.current_sequence;
        self.current_sequence = self.current_sequence.next();
        self.events.push(EventRecord {
            sequence,
            source,
            event,
            master_clock,
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SubscriptionId(usize);

pub struct EventSubscription {
    source: DebugSource,
    first_unconsumed: EventSequence,
    id: SubscriptionId,
}

impl EventSubscription {
    pub fn new(source: DebugSource) -> Self {
        let first_unconsumed = DEBUG_EVENT_LOG.with(|log| log.borrow().current_sequence);
        let id = DEBUG_SUBSCRIPTION_REGISTRY
            .with(|registry| registry.borrow_mut().subcribe(first_unconsumed));

        Self {
            source,
            first_unconsumed,
            id,
        }
    }

    pub fn poll_batch<F, R>(&mut self, mut callback: F) -> R
    where
        F: FnMut(&mut dyn Iterator<Item = (DebugSource, &DebugEvent)>) -> R,
    {
        let result = DEBUG_EVENT_LOG.with(|log| {
            let log = log.borrow();
            let mut matching_events = Vec::new();

            for record in &log.events {
                if record.sequence >= self.first_unconsumed
                    && (self.source == DebugSource::Any || self.source == record.source)
                {
                    matching_events.push((record.source, &record.event));
                    self.first_unconsumed = record.sequence;
                }
            }

            let mut iter = matching_events.into_iter();
            callback(&mut iter)
        });

        DEBUG_SUBSCRIPTION_REGISTRY.with(|registry| {
            registry
                .borrow_mut()
                .consume_events(self.id, self.first_unconsumed);
        });

        result
    }

    pub fn poll(&mut self) -> Vec<DebugEvent> {
        self.poll_batch(|iter| iter.map(|(_, event)| event.clone()).collect())
    }

    pub fn has_pending(&self) -> bool {
        DEBUG_EVENT_LOG.with(|log| log.borrow().current_sequence > self.first_unconsumed)
    }
}

impl Drop for EventSubscription {
    fn drop(&mut self) {
        DEBUG_SUBSCRIPTION_REGISTRY.with(|registry| {
            registry.borrow_mut().unsubscribe(self.id);
        });
    }
}

struct SubscriptionRegistry {
    active_subscriptions: HashMap<SubscriptionId, EventSequence>,
    next_id: SubscriptionId,
}

impl SubscriptionRegistry {
    fn new() -> Self {
        Self {
            active_subscriptions: HashMap::new(),
            next_id: SubscriptionId(0),
        }
    }

    fn subcribe(&mut self, current_sequence: EventSequence) -> SubscriptionId {
        let id = self.next_id;
        self.next_id.0 += 1;

        self.active_subscriptions.insert(id, current_sequence);
        id
    }

    fn unsubscribe(&mut self, id: SubscriptionId) {
        self.active_subscriptions.remove(&id);
    }

    fn consume_events(&mut self, id: SubscriptionId, first_unconsumed: EventSequence) {
        self.active_subscriptions.insert(id, first_unconsumed);

        let min_first_unconsumed = self
            .active_subscriptions
            .values()
            .copied()
            .min()
            .unwrap_or(EventSequence(0));

        if min_first_unconsumed < first_unconsumed {
            // There is a subscription that has consumed less than the caller. No cleanup needed.
            return;
        }

        DEBUG_EVENT_LOG.with(|log| {
            let mut log = log.borrow_mut();

            let retain_from = log
                .events
                .iter()
                .position(|record| record.sequence >= min_first_unconsumed)
                .unwrap_or(log.events.len());

            log.events.drain(0..retain_from);
        });
    }
}

thread_local! {
    static DEBUG_EVENT_LOG: RefCell<EventLog> = RefCell::new(EventLog::new());
    static DEBUG_SUBSCRIPTION_REGISTRY: RefCell<SubscriptionRegistry> =
        RefCell::new(SubscriptionRegistry::new());
}

pub trait Snapshotable {
    type View;

    fn debug_view(&self) -> Self::View;
}

pub trait Debuggable: Snapshotable {
    const SOURCE: DebugSource;
    type Event: Into<DebugEvent>;

    fn emit_debug_event(&self, event: Self::Event, master_clock: MasterClockTick) {
        DEBUG_EVENT_LOG.with(|log| {
            log.borrow_mut()
                .append(Self::SOURCE, event.into(), master_clock)
        });
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
mod tests {
    use super::*;

    fn emit_event(source: DebugSource, event: DebugEvent, master_clock: MasterClockTick) {
        DEBUG_EVENT_LOG.with(|log| log.borrow_mut().append(source, event, master_clock));
    }

    #[test]
    fn test_subscribe_single_subscription() {
        let mut subscription = subscribe(DebugSource::Cpu);

        emit_event(
            DebugSource::Cpu,
            DebugEvent::Cpu(event::CpuDebugEvent::Register8Written {
                register: crate::system::cpu::Register8::A,
                is: 0x42,
                was: 0x00,
            }),
        );

        let events = subscription.poll();
        assert_eq!(events.len(), 1);
        assert!(matches!(&events[0], DebugEvent::Cpu(_)));
    }

    #[test]
    fn test_subscribe_multiple_subscriptions_same_source() {
        let mut subscription1 = subscribe(DebugSource::Memory);
        let mut subscription2 = subscribe(DebugSource::Memory);

        emit_event(
            DebugSource::Memory,
            DebugEvent::Memory(event::MemoryDebugEvent::MemoryRead {
                address: 0x1000,
                value: 0x42,
            }),
        );

        let events1 = subscription1.poll();
        let events2 = subscription2.poll();
        assert_eq!(events1.len(), 1);
        assert_eq!(events2.len(), 1);
    }

    #[test]
    fn test_subscribe_multiple_sources() {
        let mut cpu_subscription = subscribe(DebugSource::Cpu);
        let mut memory_subscription = subscribe(DebugSource::Memory);

        emit_event(
            DebugSource::Cpu,
            DebugEvent::Cpu(event::CpuDebugEvent::Register8Written {
                register: crate::system::cpu::Register8::A,
                is: 0x42,
                was: 0x00,
            }),
        );
        emit_event(
            DebugSource::Memory,
            DebugEvent::Memory(event::MemoryDebugEvent::MemoryRead {
                address: 0x1000,
                value: 0x42,
            }),
        );

        let cpu_events = cpu_subscription.poll();
        let memory_events = memory_subscription.poll();

        assert_eq!(cpu_events.len(), 1);
        assert!(matches!(&cpu_events[0], DebugEvent::Cpu(_)));

        assert_eq!(memory_events.len(), 1);
        assert!(matches!(&memory_events[0], DebugEvent::Memory(_)));
    }

    #[test]
    fn test_poll_batch_zero_clone() {
        let mut subscription = subscribe(DebugSource::Cpu);

        emit_event(
            DebugSource::Cpu,
            DebugEvent::Cpu(event::CpuDebugEvent::Register8Written {
                register: crate::system::cpu::Register8::A,
                is: 0x42,
                was: 0x00,
            }),
        );

        let batch = subscription.poll_batch();
        assert_eq!(batch.len(), 1);

        for (source, event) in batch {
            assert_eq!(source, DebugSource::Cpu);
            assert!(matches!(event, DebugEvent::Cpu(_)));
        }
    }

    #[test]
    fn test_subscribe_all_receives_all_sources() {
        let mut subscription = subscribe_all();

        emit_event(
            DebugSource::Cpu,
            DebugEvent::Cpu(event::CpuDebugEvent::Register8Written {
                register: crate::system::cpu::Register8::A,
                is: 0x42,
                was: 0x00,
            }),
        );
        emit_event(
            DebugSource::Memory,
            DebugEvent::Memory(event::MemoryDebugEvent::MemoryRead {
                address: 0x1000,
                value: 0x42,
            }),
        );

        let events = subscription.poll();
        assert_eq!(events.len(), 2);
    }

    #[test]
    fn test_has_pending_events() {
        let mut subscription = subscribe(DebugSource::Cpu);
        assert!(!subscription.has_pending());

        emit_event(
            DebugSource::Cpu,
            DebugEvent::Cpu(event::CpuDebugEvent::Register8Written {
                register: crate::system::cpu::Register8::A,
                is: 0x42,
                was: 0x00,
            }),
        );

        assert!(subscription.has_pending());

        let _events = subscription.poll();
        assert!(!subscription.has_pending());
    }

    #[test]
    fn test_cleanup_old_events() {
        let mut subscription1 = subscribe(DebugSource::Cpu);
        let mut subscription2 = subscribe(DebugSource::Cpu);

        // Emit some events
        for i in 0..10 {
            emit_event(
                DebugSource::Cpu,
                DebugEvent::Cpu(event::CpuDebugEvent::Register8Written {
                    register: crate::system::cpu::Register8::A,
                    is: i,
                    was: 0x00,
                }),
            );
        }

        // First subscription consumes all events
        let events1 = subscription1.poll();
        assert_eq!(events1.len(), 10);

        // Cleanup should not remove events that subscription2 hasn't consumed
        cleanup_old_events();

        let events2 = subscription2.poll();
        assert_eq!(events2.len(), 10); // Should still get all events

        // Now cleanup should remove events since both have consumed them
        cleanup_old_events();

        // Verify log is empty by checking internal state would be consistent
        assert!(!subscription1.has_pending());
        assert!(!subscription2.has_pending());
    }

    #[test]
    fn test_emit_with_no_subscriptions() {
        emit_event(
            DebugSource::Cpu,
            DebugEvent::Cpu(event::CpuDebugEvent::Register8Written {
                register: crate::system::cpu::Register8::A,
                is: 0x42,
                was: 0x00,
            }),
        ); // Should not panic
    }

    #[test]
    fn test_event_sequence_ordering() {
        let seq1 = EventSequence::new(1);
        let seq2 = EventSequence::new(2);
        let seq3 = seq1.next();

        assert!(seq1 < seq2);
        assert_eq!(seq3.value(), 2);
        assert_eq!(seq3, seq2);
    }

    #[test]
    fn test_master_clock_tick() {
        let tick1 = MasterClockTick::new(100);
        let tick2 = MasterClockTick::new(200);

        assert!(tick1 < tick2);
        assert_eq!(tick1.value(), 100);
        assert_eq!(tick2.value(), 200);
    }
}
