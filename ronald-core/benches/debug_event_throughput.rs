use ronald_core::{
    debug::{emit_event, event::CpuDebugEvent, DebugSource, EventSubscription},
    system::{clock::MasterClockTick, cpu::Register16},
};

fn debug_event_throughput(total_events: u64, batch_size: u64, multiple_subs: bool) {
    let mut subscriptions = if multiple_subs {
        vec![
            EventSubscription::new(DebugSource::Any),
            EventSubscription::new(DebugSource::Any),
        ]
    } else {
        vec![EventSubscription::new(DebugSource::Any)]
    };

    let mut count = 0;
    while count < total_events {
        for _ in 0..batch_size {
            let source = DebugSource::Cpu;
            let event = CpuDebugEvent::Register16Written {
                register: Register16::PC,
                is: 0x1000,
                was: 0x0000,
            }
            .into();
            let master_clock = MasterClockTick::default();

            emit_event(source, event, master_clock);
        }

        subscriptions[0].with_events(|_record| {});

        count += batch_size;
    }

    if multiple_subs {
        subscriptions[1].with_events(|_record| {});
    }
}

fn criterion_benchmark(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("debug_event_throughput");

    for &total_events in &[1_000, 10_000] {
        for &batch_size in &[10, 100, 1_000] {
            for &multiple_subscribers in &[false, true] {
                let id = format!(
                    "events_{}_batch_{}_multi_{}",
                    total_events, batch_size, multiple_subscribers
                );
                group.bench_function(&id, |b| {
                    b.iter(|| {
                        debug_event_throughput(total_events, batch_size, multiple_subscribers)
                    })
                });
            }
        }
    }

    group.finish();
}

criterion::criterion_group!(benches, criterion_benchmark);
criterion::criterion_main!(benches);
