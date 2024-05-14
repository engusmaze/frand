#![cfg_attr(not(feature = "std"), no_std)]

use core::{mem::transmute, ops::Range};

mod gen;
pub use gen::*;

mod shuffle;
pub use shuffle::*;

mod bench_util;

// fhash
#[inline(always)]
const fn hash64(mut hash: u64) -> u64 {
    hash = (hash ^ hash >> 32).wrapping_mul(4997996261773036203);
    hash = (hash ^ hash >> 32).wrapping_mul(4997996261773036203);
    hash ^ hash >> 32
}

pub struct Rand {
    pub(crate) seed: u64,
}
impl Rand {
    /// Create a new Rand instance with a given seed.
    /// This function uses a hash function to randomize the seed.
    #[inline(always)]
    pub const fn with_seed(seed: u64) -> Self {
        Self { seed: hash64(seed) }
    }

    /// Mixes the current seed with the provided value.
    /// This mixing operation is particularly useful in `no_std` environments when you want
    /// to create a PRNG that incorporates external factors or environmental entropy, such
    /// as time, to increase randomness.
    #[inline]
    pub fn mix(&mut self, value: u64) {
        self.seed = hash64(self.seed.wrapping_add(value) ^ value << 10);
    }

    /// Create a new Rand instance using the system time as a seed.
    /// This function is only available when the "std" feature is enabled,
    /// as it relies on the standard library's time functionality.
    #[cfg(feature = "std")]
    #[inline]
    pub fn new() -> Self {
        let [a, b]: [u64; 2] = unsafe { transmute(std::time::Instant::now()) };
        let mut rand = Self { seed: a };
        rand.mix(b);
        rand
    }

    /// Rehashes the current Rand instance by creating a new one with a fresh seed.
    /// This function is only available when the "std" feature is enabled.
    #[cfg(feature = "std")]
    #[inline]
    pub fn rehash(&mut self) {
        *self = Self::new();
    }
}
