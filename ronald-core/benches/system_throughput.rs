use std::collections::HashMap;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;

use ronald_core::debug::breakpoint::AnyBreakpoint;
use ronald_core::system::memory::RomSlot;
use ronald_core::system::{CpcModel, CrtcType, DiskDrives, SystemConfig};
use ronald_core::{AudioSink, Driver, VideoSink};

const EMULATED_MICROSECONDS: usize = 1_000_000; // we want to be able to see the throughput in MHz

struct NullVideo;

impl VideoSink for NullVideo {
    fn draw_frame(&mut self, buffer: &[u8]) {
        black_box(buffer);
    }
}

struct NullAudio;

impl AudioSink for NullAudio {
    /// `None` keeps the PSG from resampling, so the benchmark is not dominated by audio.
    fn get_sample_rate(&self) -> Option<f32> {
        None
    }
    fn play_audio(&self) {}
    fn pause_audio(&self) {}
    fn add_sample(&self, _sample: f32) {}
}

fn booted_driver() -> Driver {
    let roms = HashMap::from([
        (
            RomSlot::Lower,
            include_bytes!("../tests/roms/AmstradDiagLower.rom").to_vec(),
        ),
        (
            RomSlot::Upper(0),
            include_bytes!("../tests/roms/AmstradDiagUpper.rom").to_vec(),
        ),
    ]);

    let mut driver = Driver::with_config(SystemConfig {
        model: CpcModel::Cpc6128,
        crtc: CrtcType::Type0,
        disk_drives: DiskDrives::Two,
        roms,
    });

    driver.step(2_000_000, &mut NullVideo, &mut NullAudio);
    driver
}

fn system_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("system");
    group.throughput(Throughput::Elements(EMULATED_MICROSECONDS as u64));
    group.sample_size(20);

    group.bench_function("idle", |b| {
        let mut driver = booted_driver();
        b.iter(|| driver.step(EMULATED_MICROSECONDS, &mut NullVideo, &mut NullAudio));
    });

    group.bench_function("breakpoint_set", |b| {
        let mut driver = booted_driver();
        // An address the boot loop never reaches, so the run is never
        // interrupted and every event still has to be built and matched.
        driver
            .breakpoint_manager()
            .add_breakpoint(AnyBreakpoint::pc_breakpoint(0xfffe));
        b.iter(|| driver.step(EMULATED_MICROSECONDS, &mut NullVideo, &mut NullAudio));
    });

    group.finish();
}

criterion_group!(benches, system_throughput);
criterion_main!(benches);
