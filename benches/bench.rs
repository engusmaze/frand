#![feature(test)]

extern crate test;
use std::hint::black_box;

use test::Bencher;

const ITERATIONS: usize = 100_000;

#[bench]
fn frand_rand_bench(b: &mut Bencher) {
    use frand::Rand;

    let mut rng = Rand::new();
    b.iter(|| {
        for _ in 0..ITERATIONS {
            black_box(rng.gen::<u64>());
        }
    });
}

#[bench]
fn frand_quality_rand_bench(b: &mut Bencher) {
    use frand::QualityRand;

    let mut rng = QualityRand::new();
    b.iter(|| {
        for _ in 0..ITERATIONS {
            black_box(rng.gen::<u64>());
        }
    });
}

#[bench]
fn rand_rand_bench(b: &mut Bencher) {
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    b.iter(|| {
        for _ in 0..ITERATIONS {
            black_box(rng.gen::<u64>());
        }
    });
}

#[bench]
fn rand_small_rng_bench(b: &mut Bencher) {
    use rand::{rngs::SmallRng, Rng, SeedableRng};

    let mut rng = SmallRng::from_entropy();
    b.iter(|| {
        for _ in 0..ITERATIONS {
            black_box(rng.gen::<u64>());
        }
    });
}

#[bench]
fn frand_rand_bench_u128(b: &mut Bencher) {
    use frand::Rand;

    let mut rng = Rand::new();
    b.iter(|| {
        for _ in 0..ITERATIONS {
            black_box(rng.gen::<u128>());
        }
    });
}

#[bench]
fn frand_quality_rand_bench_u128(b: &mut Bencher) {
    use frand::QualityRand;

    let mut rng = QualityRand::new();
    b.iter(|| {
        for _ in 0..ITERATIONS {
            black_box(rng.gen::<u128>());
        }
    });
}

#[bench]
fn rand_rand_bench_u128(b: &mut Bencher) {
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    b.iter(|| {
        for _ in 0..ITERATIONS {
            black_box(rng.gen::<u128>());
        }
    });
}

#[bench]
fn rand_small_rng_bench_u128(b: &mut Bencher) {
    use rand::{rngs::SmallRng, Rng, SeedableRng};

    let mut rng = SmallRng::from_entropy();
    b.iter(|| {
        for _ in 0..ITERATIONS {
            black_box(rng.gen::<u128>());
        }
    });
}

#[bench]
fn frand_rand_bench_f64(b: &mut Bencher) {
    use frand::Rand;

    let mut rng = Rand::new();
    b.iter(|| {
        for _ in 0..ITERATIONS {
            black_box(rng.gen::<f64>());
        }
    });
}
#[bench]
fn rand_small_rng_bench_f64(b: &mut Bencher) {
    use rand::{rngs::SmallRng, Rng, SeedableRng};

    let mut rng = SmallRng::from_entropy();
    b.iter(|| {
        for _ in 0..ITERATIONS {
            black_box(rng.gen::<f64>());
        }
    });
}

#[bench]
fn frand_quality_rand_bench_f64(b: &mut Bencher) {
    use frand::QualityRand;

    let mut rng = QualityRand::new();
    b.iter(|| {
        for _ in 0..ITERATIONS {
            black_box(rng.gen::<f64>());
        }
    });
}
#[bench]
fn rand_rand_bench_f64(b: &mut Bencher) {
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    b.iter(|| {
        for _ in 0..ITERATIONS {
            black_box(rng.gen::<f64>());
        }
    });
}
