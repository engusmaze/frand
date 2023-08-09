#![feature(test)]

extern crate test;
use std::hint::black_box;

use rand::Rng;
use test::Bencher;

const ITERATIONS: usize = 100_000;

#[bench]
fn bench_1_frand_rand_u64(b: &mut Bencher) {
    use frand::Rand;

    let mut rng = Rand::new();
    b.iter(|| {
        for _ in 0..ITERATIONS {
            black_box(rng.gen::<u64>());
        }
    });
}

#[bench]
fn bench_2_rand_small_rng_u64(b: &mut Bencher) {
    use rand::{rngs::SmallRng, Rng, SeedableRng};

    let mut rng = SmallRng::from_entropy();
    b.iter(|| {
        for _ in 0..ITERATIONS {
            black_box(rng.gen::<u64>());
        }
    });
}

#[bench]
fn bench_3_rand_rand_u64(b: &mut Bencher) {
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    b.iter(|| {
        for _ in 0..ITERATIONS {
            black_box(rng.gen::<u64>());
        }
    });
}

#[bench]
fn bench_4_nanorand_wyrand_u64(b: &mut Bencher) {
    use nanorand::{Rng, WyRand};

    let mut rng = WyRand::new();
    b.iter(|| {
        for _ in 0..ITERATIONS {
            black_box(rng.generate::<u64>());
        }
    });
}

#[bench]
fn bench_5_fastrand_u64(b: &mut Bencher) {
    use fastrand::Rng;

    let mut rng = Rng::new();
    b.iter(|| {
        for _ in 0..ITERATIONS {
            black_box(rng.u64(..));
        }
    });
}
