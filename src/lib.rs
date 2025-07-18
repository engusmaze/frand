//! **FRand** is a blazingly fast, small, and simple pseudo-random number
//! generator (PRNG) written in Rust. The advantage of using FRand is that
//! it can produce more random numbers per second than other libraries. It
//! also produces high-quality random numbers using a fast
//! **non-cryptographic** hashing algorithm.
//!
//! # Usage
//!
//! This crate is [on crates.io](https://crates.io/crates/regex) and can be
//! used by adding `frand` to your dependencies in your project's `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! frand = "0.8"
//! ```
//!
//! # Example
//!
//! **FRand** is really simple to use. Here is a simple example of how to
//! use FRand to generate a random float:
//!
//! ```rust
//! use frand::Rand;
//!
//! let mut rng = Rand::new();
//! println!("{}", rng.random::<f32>());
//! ```
//!
//! # Crate features
//!
//! * **std** -
//!   (default) Enables the use of the standard library. This feature is
//!   required for the `new` and `rehash` functions.
//! * **impl_rng_core** -
//!   (default) Enables the implementation of the `rand::RngCore` and
//!   `rand::SeedableRng` traits for the `Rand` struct. This feature is
//!   required to use FRand with the `rand` and things like `rand::distributions`.
//! * **glam** -
//!   (default) Uses glam to enable the generation of random values
//!   for glam::Vec2, glam::Vec3, glam::Vec3A, and glam::Vec4.

#![cfg_attr(not(feature = "std"), no_std)]

use core::{mem::transmute, ops::Range};

mod random;
pub use random::*;

#[cfg(feature = "std")]
mod thread;
#[cfg(feature = "std")]
pub use thread::*;

#[cfg(feature = "impl_rng_core")]
mod rng_core_impl;

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

impl Default for Rand {
    fn default() -> Self {
        Self::new()
    }
}
