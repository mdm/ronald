//! Compares `Screen` against the pre-optimization implementation on a frame's
//! worth of gate array output.

#[path = "../tests/common/reference_screen.rs"]
mod reference_screen;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;

use reference_screen::ReferenceScreen;
use ronald_core::VideoSink;
use ronald_core::system::bus::screen::Screen;

const VIRTUAL_BUFFER_WIDTH: usize = 64 * 16;
const VIRTUAL_BUFFER_HEIGHT: usize = 39 * 16;
const LINES_PER_FRAME: usize = VIRTUAL_BUFFER_HEIGHT / 2;
const CHARACTER_PAIRS_PER_LINE: usize = VIRTUAL_BUFFER_WIDTH / 16;

struct NullVideo;

impl VideoSink for NullVideo {
    fn draw_frame(&mut self, buffer: &[u8]) {
        black_box(buffer);
    }
}

/// One gate array step feeds two bytes worth of pixels to the screen
#[derive(Clone, Copy)]
enum Batch {
    Mode0([u8; 4]),
    Mode1([u8; 8]),
    Mode2([u8; 16]),
}

fn frame(mode: u8) -> Vec<Batch> {
    let color = || rand::random::<u8>() % 32;

    let mut steps = Vec::with_capacity(LINES_PER_FRAME * CHARACTER_PAIRS_PER_LINE);
    for _line in 0..LINES_PER_FRAME {
        for _character_pair in 0..CHARACTER_PAIRS_PER_LINE {
            steps.push(match mode {
                0 => Batch::Mode0(std::array::from_fn(|_| color())),
                1 => Batch::Mode1(std::array::from_fn(|_| color())),
                2 => Batch::Mode2(std::array::from_fn(|_| color())),
                _ => panic!("invalid mode {mode}"),
            });
        }
    }

    steps
}

fn write_frame_pixels(screen: &mut Screen, batches: &[Batch]) {
    for batch in batches {
        match batch {
            Batch::Mode0(colors) => {
                screen.write(&std::array::from_fn::<_, 16, _>(|i| colors[i / 4]))
            }
            Batch::Mode1(colors) => {
                screen.write(&std::array::from_fn::<_, 16, _>(|i| colors[i / 2]))
            }
            Batch::Mode2(colors) => screen.write(colors),
        }
    }
}

fn write_reference_frame_pixels(screen: &mut ReferenceScreen, batches: &[Batch]) {
    for batch in batches {
        match batch {
            Batch::Mode0(colors) => {
                for &color in colors {
                    screen.write(color as usize);
                }
            }
            Batch::Mode1(colors) => {
                for &color in colors {
                    screen.write(color as usize);
                }
            }
            Batch::Mode2(colors) => {
                for &color in colors {
                    screen.write(color as usize);
                }
            }
        }
    }
}

fn screen_throughput(c: &mut Criterion) {
    for (name, mode) in [("mode0", 0), ("mode1", 1), ("mode2", 2)] {
        let steps = frame(mode);
        let mut group = c.benchmark_group(name);
        group.throughput(Throughput::Elements(
            (LINES_PER_FRAME * CHARACTER_PAIRS_PER_LINE * 16) as u64,
        ));

        group.bench_function(BenchmarkId::from_parameter("batched"), |b| {
            let mut screen = Screen::default();
            b.iter(|| {
                // `Screen` starts out waiting for vsync; without this the
                // benchmark would measure an empty early return.
                screen.trigger_vsync(&mut NullVideo);
                write_frame_pixels(&mut screen, black_box(&steps));
            });
        });

        group.bench_function(BenchmarkId::from_parameter("reference"), |b| {
            let mut screen = ReferenceScreen::default();
            b.iter(|| {
                screen.trigger_vsync(&mut NullVideo);
                write_reference_frame_pixels(&mut screen, black_box(&steps));
            });
        });

        group.finish();
    }
}

criterion_group!(benches, screen_throughput);
criterion_main!(benches);
