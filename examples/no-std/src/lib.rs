//! You can run this example with `cargo build -p no-std`

#![no_std]

use frand::Rand;
use rand::Rng;

#[no_mangle]
fn create_rng() -> Rand {
    return Rand::with_seed(0);
}

#[no_mangle]
fn gen_u64(rng: &mut Rand) -> u64 {
    rng.gen::<u64>()
}

#[no_mangle]
fn gen_bool_rand_core(rng: &mut Rand) -> bool {
    rng.gen_bool(0.5)
}
