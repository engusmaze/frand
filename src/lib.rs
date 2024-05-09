#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "impl_rng_core")]
use rand::{RngCore, SeedableRng};
#[cfg(feature = "impl_rng_core")]
use rand_core::impls::fill_bytes_via_next;
#[cfg(feature = "impl_rng_core")]
pub use rand::*;

mod gen;
pub use gen::*;

#[cfg(feature = "thread_rng")]
mod thread;
#[cfg(feature = "thread_rng")]
pub use thread::*;

mod shuffle;
pub use shuffle::*;

mod bench_util;

mod cast;
pub(crate) use cast::*;

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
        let duration = std::time::SystemTime::UNIX_EPOCH
            .elapsed()
            .expect("Failed to get current time");
        let mut rand = Self {
            seed: duration.as_secs(),
        };
        rand.mix(duration.subsec_nanos() as u64);
        rand
    }

    /// Rehashes the current Rand instance by creating a new one with a fresh seed.
    /// This function is only available when the "std" feature is enabled.
    #[cfg(feature = "std")]
    #[inline]
    pub fn rehash(&mut self) {
        let _ = core::mem::replace(self, Self::new());
    }

    /// Generates a random value of type T using this Rand instance.
    /// T must implement the RandomGeneratable trait, which defines how to generate random values.
    #[inline(always)]
    pub fn gen<T: RandomGeneratable>(&mut self) -> T {
        T::random(self)
    }
}


impl RngCore for Rand {
    fn next_u32(&mut self) -> u32 {
        self.gen::<u32>()
    }

    fn next_u64(&mut self) -> u64 {
        self.gen::<u64>()
    }
    
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        fill_bytes_via_next(self, dest);
    }
    
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

#[cfg(feature = "impl_rng_core")]
impl SeedableRng for Rand {
    type Seed = [u8; 8];

    fn from_seed(seed: Self::Seed) -> Self {
        Self::with_seed(u64::from_be_bytes(seed))
    }

    fn seed_from_u64(state: u64) -> Self {
        Self::with_seed(state)
    }
}
