#![feature(test)]

extern crate test;
use std::hint::black_box;

use test::Bencher;

const ITERATIONS: usize = 100_000;

#[bench]
fn bench_1_frand_rand_u128(b: &mut Bencher) {
    use frand::Rand;

    let mut rng = Rand::new();
    b.iter(|| {
        for _ in 0..ITERATIONS {
            black_box(rng.gen::<u128>());
        }
    });
}

#[bench]
fn bench_2_rand_small_rng_u128(b: &mut Bencher) {
    use rand::{rngs::SmallRng, Rng, SeedableRng};

    let mut rng = SmallRng::from_entropy();
    b.iter(|| {
        for _ in 0..ITERATIONS {
            black_box(rng.gen::<u128>());
        }
    });
}

#[bench]
fn bench_3_rand_rand_u128(b: &mut Bencher) {
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    b.iter(|| {
        for _ in 0..ITERATIONS {
            black_box(rng.gen::<u128>());
        }
    });
}
