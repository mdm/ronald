use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;

use ronald_core::constants::{SCREEN_BUFFER_HEIGHT, SCREEN_BUFFER_WIDTH};

const NUM_PIXELS: usize = SCREEN_BUFFER_WIDTH * SCREEN_BUFFER_HEIGHT;

fn generate() -> Vec<u8> {
    let mut packed = Vec::with_capacity(NUM_PIXELS);
    for _ in 0..NUM_PIXELS {
        packed.push(rand::random::<u8>());
    }

    packed
}

const fn mode0_pen(b: u8) -> u8 {
    ((b & 0x80) >> 7) | ((b & 0x20) >> 3) | ((b & 0x08) >> 2) | ((b & 0x02) << 2)
}

const fn mode1_pen(b: u8) -> u8 {
    ((b & 0x80) >> 7) | ((b & 0x08) >> 2)
}

const MODE0_PEN_LUT: [[u8; 2]; 256] = {
    let mut t = [[0u8; 2]; 256];
    let mut i = 0;
    while i < 256 {
        t[i] = [mode0_pen(i as u8), mode0_pen((i as u8) << 1)];
        i += 1;
    }
    t
};

const MODE1_PEN_LUT: [[u8; 4]; 256] = {
    let mut t = [[0u8; 4]; 256];
    let mut i = 0;
    while i < 256 {
        let b = i as u8;
        t[i] = [
            mode1_pen(b),
            mode1_pen(b << 1),
            mode1_pen(b << 2),
            mode1_pen(b << 3),
        ];
        i += 1;
    }
    t
};

const MODE2_PEN_LUT: [[u8; 8]; 256] = {
    let mut t = [[0u8; 8]; 256];
    let mut i = 0;
    while i < 256 {
        let b = i as u8;
        t[i] = [
            (b & 0x80) >> 7,
            (b & 0x40) >> 6,
            (b & 0x20) >> 5,
            (b & 0x10) >> 4,
            (b & 0x08) >> 3,
            (b & 0x04) >> 2,
            (b & 0x02) >> 1,
            b & 0x01,
        ];
        i += 1;
    }
    t
};

fn unpack_lookup(mode: u8, packed: &[u8]) -> Vec<u8> {
    let num_pixels = match mode {
        0 => packed.len() * 2,
        1 => packed.len() * 4,
        2 => packed.len() * 8,
        _ => panic!("invalid mode {mode}"),
    };

    let mut unpacked = Vec::with_capacity(num_pixels);

    match mode {
        0 => {
            for p in packed {
                unpacked.extend_from_slice(&MODE0_PEN_LUT[*p as usize]);
            }
        }
        1 => {
            for p in packed {
                unpacked.extend_from_slice(&MODE1_PEN_LUT[*p as usize]);
            }
        }
        2 => {
            for p in packed {
                unpacked.extend_from_slice(&MODE2_PEN_LUT[*p as usize]);
            }
        }
        _ => panic!("invalid mode {mode}"),
    }

    unpacked
}

fn unpack_calculate(mode: u8, packed: &[u8]) -> Vec<u8> {
    let num_pixels = match mode {
        0 => packed.len() * 2,
        1 => packed.len() * 4,
        2 => packed.len() * 8,
        _ => panic!("invalid mode {mode}"),
    };

    let mut unpacked = Vec::with_capacity(num_pixels);

    match mode {
        0 => {
            for p in packed {
                unpacked.extend_from_slice(&[
                    ((p & 0x80) >> 7) | ((p & 0x08) >> 2) | ((p & 0x20) >> 3) | ((p & 0x02) << 2),
                    ((p & 0x40) >> 6) | ((p & 0x04) >> 1) | ((p & 0x10) >> 2) | ((p & 0x01) << 3),
                ]);
            }
        }
        1 => {
            for p in packed {
                unpacked.extend_from_slice(&[
                    ((p & 0x80) >> 7) | ((p & 0x08) >> 2),
                    ((p & 0x40) >> 6) | ((p & 0x04) >> 1),
                    ((p & 0x20) >> 5) | (p & 0x02),
                    ((p & 0x10) >> 4) | ((p & 0x01) << 1),
                ]);
            }
        }
        2 => {
            for p in packed {
                unpacked.extend_from_slice(&[
                    (p & 0x80) >> 7,
                    (p & 0x40) >> 6,
                    (p & 0x20) >> 5,
                    (p & 0x10) >> 4,
                    (p & 0x08) >> 3,
                    (p & 0x04) >> 2,
                    (p & 0x02) >> 1,
                    p & 0x01,
                ]);
            }
        }
        _ => panic!("invalid mode {mode}"),
    }

    unpacked
}

fn pixel_unpacking(c: &mut Criterion) {
    for (name, mode) in [("mode0", 0), ("mode1", 1), ("mode2", 2)] {
        let packed = generate();
        let mut group = c.benchmark_group(name);
        group.throughput(Throughput::Elements(packed.len() as u64));

        group.bench_function(BenchmarkId::from_parameter("lookup"), |b| {
            b.iter(|| {
                unpack_lookup(mode, black_box(&packed));
            });
        });

        group.bench_function(BenchmarkId::from_parameter("calculate"), |b| {
            b.iter(|| {
                unpack_calculate(mode, black_box(&packed));
            });
        });

        group.finish();
    }
}

criterion_group!(benches, pixel_unpacking);
criterion_main!(benches);
