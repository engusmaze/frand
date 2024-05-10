/*!
**FRand** is a blazingly fast, small, and simple pseudo-random number 
generator (PRNG) written in Rust. The advantage of using FRand is that 
it can produce more random numbers per second than other libraries. It 
also produces high-quality random numbers using a fast 
**non-cryptographic** hashing algorithm.

# Usage

This crate is [on crates.io](https://crates.io/crates/regex) and can be
used by adding `regex` to your dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
frand = "0.8"
```

# Example

**FRand** is really simple to use. Here is a simple example of how to 
use FRand to generate a random float:

```rust
use frand::Rand;

let mut rng = Rand::new();
println!("{}", rng.gen::<f32>());
```

# Crate features

* **std** -
    (default) Enables the use of the standard library. This feature is 
    required for the `new` and `rehash` functions.
* **alloc* -
    (default) Enables the use of the `alloc` crate. This feature is 
    required to allow shuffling of Boxes and Vecs.
* **impl_rng_core** -
    (default) Enables the implementation of the `rand::RngCore` and 
    `rand::SeedableRng` traits for the `Rand` struct. This feature is
    required to use FRand with the `rand` and things like `rand::distributions`.
* **thread_rng** -
    (default) Enables the thread local version of FRand. This can be
    accessed using the `thread_frand` function.
* **glam_027** -
    (default) Uses glam version 0.27 to enable the generation of random 
    values for glam::Vec2, glam::Vec3, glam::Vec3A, and glam::Vec4.
* **glam_026** -
    (default) Uses glam version 0.26 to enable the generation of random 
    values for glam::Vec2, glam::Vec3, glam::Vec3A, and glam::Vec4.
* **glam_025** -
    (default) Uses glam version 0.25 to enable the generation of random 
    values for glam::Vec2, glam::Vec3, glam::Vec3A, and glam::Vec4.
* **glam_024** -
    (default) Uses glam version 0.24 to enable the generation of random 
    values for glam::Vec2, glam::Vec3, glam::Vec3A, and glam::Vec4.        


 */
#![cfg_attr(not(feature = "std"), no_std)]

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
