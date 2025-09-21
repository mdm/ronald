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

#[derive(Debug, Clone)]
pub struct EventRecord {
    sequence: EventSequence,
    pub source: DebugSource,
    pub event: DebugEvent,
    pub master_clock: MasterClockTick,
}

struct EventLog {
    events: Vec<EventRecord>,
    first_sequence: EventSequence,
    next_sequence: EventSequence,
}

impl EventLog {
    fn new() -> Self {
        Self {
            events: Vec::new(),
            first_sequence: EventSequence(0),
            next_sequence: EventSequence(0),
        }
    }

    fn append(&mut self, source: DebugSource, event: DebugEvent, master_clock: MasterClockTick) {
        let sequence = self.next_sequence;
        self.next_sequence = self.next_sequence.next();
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
    id: SubscriptionId,
    source: DebugSource,
    first_unconsumed: EventSequence,
}

impl EventSubscription {
    pub fn new(source: DebugSource) -> Self {
        let first_unconsumed = DEBUG_EVENT_LOG.with(|log| log.borrow().next_sequence);
        let id = DEBUG_SUBSCRIPTION_REGISTRY
            .with(|registry| registry.borrow_mut().subcribe(first_unconsumed));

        Self {
            id,
            source,
            first_unconsumed,
        }
    }

    pub fn with_events<F>(&mut self, mut callback: F)
    where
        F: FnMut(&EventRecord),
    {
        DEBUG_EVENT_LOG.with(|log| {
            let log = log.borrow();
            let first_unconsumed = self.first_unconsumed.0 - log.first_sequence.0;

            for record in &log.events[first_unconsumed as usize..] {
                self.first_unconsumed = record.sequence.next();
                if self.source == DebugSource::Any || self.source == record.source {
                    callback(record);
                }
            }
        });

        DEBUG_SUBSCRIPTION_REGISTRY.with(|registry| {
            registry
                .borrow_mut()
                .consume_events(self.id, self.first_unconsumed);
        });
    }

    pub fn has_pending(&self) -> bool {
        DEBUG_EVENT_LOG.with(|log| log.borrow().next_sequence > self.first_unconsumed)
    }

    pub fn pending_count(&self) -> u64 {
        if !self.has_pending() {
            return 0;
        }

        DEBUG_EVENT_LOG.with(|log| {
            let log = log.borrow();
            let mut count = 0;

            for record in &log.events {
                if record.sequence >= self.first_unconsumed
                    && (self.source == DebugSource::Any || self.source == record.source)
                {
                    count += 1;
                }
            }

            count
        })
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

            let retain_from = min_first_unconsumed.0 - log.first_sequence.0;
            log.events.drain(0..retain_from as usize);
            log.first_sequence = min_first_unconsumed;
        });
    }
}

thread_local! {
    static DEBUG_EVENT_LOG: RefCell<EventLog> = RefCell::new(EventLog::new());
    static DEBUG_SUBSCRIPTION_REGISTRY: RefCell<SubscriptionRegistry> =
        RefCell::new(SubscriptionRegistry::new());
}

pub fn emit_event(source: DebugSource, event: DebugEvent, master_clock: MasterClockTick) {
    DEBUG_EVENT_LOG.with(|log| log.borrow_mut().append(source, event, master_clock));
}

pub trait Snapshotable {
    type View;

    fn debug_view(&self) -> Self::View;
}

pub trait Debuggable: Snapshotable {
    const SOURCE: DebugSource;
    type Event: Into<DebugEvent>;

    fn emit_debug_event(&self, event: Self::Event, master_clock: MasterClockTick) {
        emit_event(Self::SOURCE, event.into(), master_clock);
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

    #[test]
    fn test_subscribe_single_subscription() {
        let mut subscription = EventSubscription::new(DebugSource::Cpu);

        emit_event(
            DebugSource::Cpu,
            DebugEvent::Cpu(event::CpuDebugEvent::Register8Written {
                register: crate::system::cpu::Register8::A,
                is: 0x42,
                was: 0x00,
            }),
            MasterClockTick::default(),
        );

        assert_eq!(subscription.pending_count(), 1);
        subscription.with_events(|record| {
            assert!(matches!(record.event, DebugEvent::Cpu(_)));
        });
    }

    #[test]
    fn test_subscribe_multiple_subscriptions_same_source() {
        let subscription1 = EventSubscription::new(DebugSource::Memory);
        let subscription2 = EventSubscription::new(DebugSource::Memory);

        emit_event(
            DebugSource::Memory,
            DebugEvent::Memory(event::MemoryDebugEvent::MemoryRead {
                address: 0x1000,
                value: 0x42,
            }),
            MasterClockTick::default(),
        );

        assert_eq!(subscription1.pending_count(), 1);
        assert_eq!(subscription2.pending_count(), 1);
    }

    #[test]
    fn test_subscribe_multiple_sources() {
        let mut cpu_subscription = EventSubscription::new(DebugSource::Cpu);
        let mut memory_subscription = EventSubscription::new(DebugSource::Memory);

        emit_event(
            DebugSource::Cpu,
            DebugEvent::Cpu(event::CpuDebugEvent::Register8Written {
                register: crate::system::cpu::Register8::A,
                is: 0x42,
                was: 0x00,
            }),
            MasterClockTick::default(),
        );
        emit_event(
            DebugSource::Memory,
            DebugEvent::Memory(event::MemoryDebugEvent::MemoryRead {
                address: 0x1000,
                value: 0x42,
            }),
            MasterClockTick::default(),
        );

        assert_eq!(cpu_subscription.pending_count(), 1);
        cpu_subscription.with_events(|record| {
            assert!(matches!(record.event, DebugEvent::Cpu(_)));
        });

        assert_eq!(memory_subscription.pending_count(), 1);
        memory_subscription.with_events(|record| {
            assert!(matches!(record.event, DebugEvent::Memory(_)));
        });
    }

    #[test]
    fn test_any_receives_all_sources() {
        let subscription = EventSubscription::new(DebugSource::Any);

        emit_event(
            DebugSource::Cpu,
            DebugEvent::Cpu(event::CpuDebugEvent::Register8Written {
                register: crate::system::cpu::Register8::A,
                is: 0x42,
                was: 0x00,
            }),
            MasterClockTick::default(),
        );
        emit_event(
            DebugSource::Memory,
            DebugEvent::Memory(event::MemoryDebugEvent::MemoryRead {
                address: 0x1000,
                value: 0x42,
            }),
            MasterClockTick::default(),
        );

        assert_eq!(subscription.pending_count(), 2);
    }

    #[test]
    fn test_has_pending_events() {
        let mut subscription = EventSubscription::new(DebugSource::Cpu);
        assert!(!subscription.has_pending());

        emit_event(
            DebugSource::Cpu,
            DebugEvent::Cpu(event::CpuDebugEvent::Register8Written {
                register: crate::system::cpu::Register8::A,
                is: 0x42,
                was: 0x00,
            }),
            MasterClockTick::default(),
        );

        assert!(subscription.has_pending());

        subscription.with_events(|_record| {});
        assert!(!subscription.has_pending());
    }

    #[test]
    fn test_cleanup_old_events() {
        let mut subscription1 = EventSubscription::new(DebugSource::Cpu);
        let mut subscription2 = EventSubscription::new(DebugSource::Cpu);

        // Emit some events
        for i in 0..10 {
            emit_event(
                DebugSource::Cpu,
                DebugEvent::Cpu(event::CpuDebugEvent::Register8Written {
                    register: crate::system::cpu::Register8::A,
                    is: i,
                    was: 0x00,
                }),
                MasterClockTick::default(),
            );
        }

        // First subscription consumes all events
        subscription1.with_events(|_record| {});

        // Cleanup should not remove events that subscription2 hasn't consumed
        assert!(!subscription1.has_pending());
        assert_eq!(subscription2.pending_count(), 10);

        // Second subscription consumes all events
        subscription2.with_events(|_record| {});

        // Now cleanup should remove events since both have consumed them
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
            MasterClockTick::default(),
        ); // Should not panic
    }

    #[test]
    fn test_event_sequence_ordering() {
        let seq1 = EventSequence(1);
        let seq2 = EventSequence(2);
        let seq3 = seq1.next();

        assert!(seq1 < seq2);
        assert_eq!(seq3.0, 2);
        assert_eq!(seq3, seq2);
    }

    #[test]
    fn test_sequence_boundary_subscription_created_before_events() {
        let subscription = EventSubscription::new(DebugSource::Cpu);

        emit_event(
            DebugSource::Cpu,
            DebugEvent::Cpu(event::CpuDebugEvent::Register8Written {
                register: crate::system::cpu::Register8::A,
                is: 0x42,
                was: 0x00,
            }),
            MasterClockTick::default(),
        );

        assert_eq!(subscription.pending_count(), 1);
        assert!(subscription.has_pending());
    }

    #[test]
    fn test_sequence_boundary_subscription_created_after_events() {
        emit_event(
            DebugSource::Cpu,
            DebugEvent::Cpu(event::CpuDebugEvent::Register8Written {
                register: crate::system::cpu::Register8::A,
                is: 0x42,
                was: 0x00,
            }),
            MasterClockTick::default(),
        );

        let subscription = EventSubscription::new(DebugSource::Cpu);

        assert_eq!(subscription.pending_count(), 0);
        assert!(!subscription.has_pending());
    }

    #[test]
    fn test_sequence_boundary_exact_consumption() {
        let mut subscription = EventSubscription::new(DebugSource::Cpu);

        emit_event(
            DebugSource::Cpu,
            DebugEvent::Cpu(event::CpuDebugEvent::Register8Written {
                register: crate::system::cpu::Register8::A,
                is: 0x01,
                was: 0x00,
            }),
            MasterClockTick::default(),
        );
        emit_event(
            DebugSource::Cpu,
            DebugEvent::Cpu(event::CpuDebugEvent::Register8Written {
                register: crate::system::cpu::Register8::A,
                is: 0x02,
                was: 0x01,
            }),
            MasterClockTick::default(),
        );

        assert_eq!(subscription.pending_count(), 2);

        let mut consumed_count = 0;
        subscription.with_events(|_record| {
            consumed_count += 1;
        });

        assert_eq!(consumed_count, 2);
        assert_eq!(subscription.pending_count(), 0);
        assert!(!subscription.has_pending());
    }

    #[test]
    fn test_sequence_boundary_multiple_consumption_calls() {
        let mut subscription = EventSubscription::new(DebugSource::Cpu);

        for i in 0..5 {
            emit_event(
                DebugSource::Cpu,
                DebugEvent::Cpu(event::CpuDebugEvent::Register8Written {
                    register: crate::system::cpu::Register8::A,
                    is: i,
                    was: 0x00,
                }),
                MasterClockTick::default(),
            );
        }

        assert_eq!(subscription.pending_count(), 5);

        let mut first_batch_count = 0;
        subscription.with_events(|_record| {
            first_batch_count += 1;
        });

        assert_eq!(first_batch_count, 5);
        assert_eq!(subscription.pending_count(), 0);
        assert!(!subscription.has_pending());

        for i in 5..8 {
            emit_event(
                DebugSource::Cpu,
                DebugEvent::Cpu(event::CpuDebugEvent::Register8Written {
                    register: crate::system::cpu::Register8::A,
                    is: i,
                    was: 0x00,
                }),
                MasterClockTick::default(),
            );
        }

        assert_eq!(subscription.pending_count(), 3);

        let mut second_batch_count = 0;
        subscription.with_events(|_record| {
            second_batch_count += 1;
        });

        assert_eq!(second_batch_count, 3);
        assert_eq!(subscription.pending_count(), 0);
    }

    #[test]
    fn test_sequence_boundary_interleaved_subscriptions() {
        let mut subscription1 = EventSubscription::new(DebugSource::Cpu);

        emit_event(
            DebugSource::Cpu,
            DebugEvent::Cpu(event::CpuDebugEvent::Register8Written {
                register: crate::system::cpu::Register8::A,
                is: 0x01,
                was: 0x00,
            }),
            MasterClockTick::default(),
        );

        let mut subscription2 = EventSubscription::new(DebugSource::Cpu);

        emit_event(
            DebugSource::Cpu,
            DebugEvent::Cpu(event::CpuDebugEvent::Register8Written {
                register: crate::system::cpu::Register8::A,
                is: 0x02,
                was: 0x01,
            }),
            MasterClockTick::default(),
        );

        assert_eq!(subscription1.pending_count(), 2);
        assert_eq!(subscription2.pending_count(), 1);

        subscription1.with_events(|_record| {});

        assert_eq!(subscription1.pending_count(), 0);
        assert_eq!(subscription2.pending_count(), 1);

        subscription2.with_events(|_record| {});

        assert_eq!(subscription1.pending_count(), 0);
        assert_eq!(subscription2.pending_count(), 0);
    }

    #[test]
    fn test_sequence_boundary_event_cleanup_precision() {
        let mut subscription1 = EventSubscription::new(DebugSource::Cpu);
        let mut subscription2 = EventSubscription::new(DebugSource::Cpu);

        for i in 0..10 {
            emit_event(
                DebugSource::Cpu,
                DebugEvent::Cpu(event::CpuDebugEvent::Register8Written {
                    register: crate::system::cpu::Register8::A,
                    is: i,
                    was: 0x00,
                }),
                MasterClockTick::default(),
            );
        }

        let mut subscription1_events = Vec::new();
        subscription1.with_events(|record| {
            subscription1_events.push(record.sequence);
        });

        assert_eq!(subscription1_events.len(), 10);
        assert_eq!(subscription2.pending_count(), 10);

        let mut subscription2_events = Vec::new();
        subscription2.with_events(|record| {
            subscription2_events.push(record.sequence);
        });

        assert_eq!(subscription2_events.len(), 10);
        assert_eq!(subscription1_events, subscription2_events);
    }

    #[test]
    fn test_sequence_boundary_first_event_zero() {
        DEBUG_EVENT_LOG.with(|log| {
            let initial_sequence = log.borrow().next_sequence;
            assert_eq!(initial_sequence.0, 0);
        });

        emit_event(
            DebugSource::Cpu,
            DebugEvent::Cpu(event::CpuDebugEvent::Register8Written {
                register: crate::system::cpu::Register8::A,
                is: 0x42,
                was: 0x00,
            }),
            MasterClockTick::default(),
        );

        DEBUG_EVENT_LOG.with(|log| {
            let log = log.borrow();
            assert_eq!(log.next_sequence.0, 1);
            assert_eq!(log.events[0].sequence.0, 0);
        });
    }

    #[test]
    fn test_sequence_boundary_consecutive_events() {
        for i in 0..5 {
            emit_event(
                DebugSource::Cpu,
                DebugEvent::Cpu(event::CpuDebugEvent::Register8Written {
                    register: crate::system::cpu::Register8::A,
                    is: i,
                    was: 0x00,
                }),
                MasterClockTick::default(),
            );
        }

        DEBUG_EVENT_LOG.with(|log| {
            let log = log.borrow();
            assert_eq!(log.next_sequence.0, 5);

            for (idx, event) in log.events.iter().enumerate() {
                assert_eq!(event.sequence.0, idx as u64);
            }
        });
    }
}
